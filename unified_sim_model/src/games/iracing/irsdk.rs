use core::slice;
use std::{ffi::c_void, fmt::Debug};
use thiserror::Error;
use tracing::{debug, info, warn};
use windows::{
    w,
    Win32::{
        Foundation::{
            CloseHandle, HANDLE, HWND, LPARAM, WAIT_ABANDONED, WAIT_FAILED, WAIT_OBJECT_0,
            WAIT_TIMEOUT, WPARAM,
        },
        System::{
            Memory::{MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ},
            Threading::{OpenEventW, WaitForSingleObject, SYNCHRONIZATION_SYNCHRONIZE},
        },
        UI::WindowsAndMessaging::{RegisterWindowMessageW, SendNotifyMessageW},
    },
};
use yore::code_pages::CP1252;

use crate::{games::iracing::irsdk::defines::VarHeader, Time};

use self::{
    defines::{
        CameraState, EngineWarnings, Flags, Header, Messages, PaceFlags, PitSvFlags, StatusField,
        TrkLoc, TrkSurf,
    },
    live_data::LiveData,
    static_data::StaticData,
};

pub mod defines;
pub mod live_data;
pub mod static_data;

/// Special handle used to SendMessage
const BROADCAST_HANDLE: HWND = HWND(0xffff);

#[derive(Default, Clone)]
pub struct Data {
    pub static_data: StaticData,
    pub live_data: LiveData,
}

#[derive(Debug, Error)]
pub enum PollError {
    #[error("The game is not connected")]
    NotConnected,
}

#[derive(Debug, Error)]
pub enum WaitError {
    #[error("The timeout expired")]
    Timeout,
    #[error("The wait failed with the error {0}")]
    Win32Error(windows::core::Error),
}

#[derive(Debug)]
pub struct Irsdk {
    /// Handle to the memory mapped file.
    file_mapping: HANDLE,
    /// Handle to wait for the data valid event.
    data_valid_event: HANDLE,
    /// The message id used to send messages to the game.
    message_id: u32,
    /// pointer into the memory mapped file.
    view: *const u8,
    /// Tick count of the last update.
    _last_tick_count: i32,
    /// List of var handlers to write the value into the data model.
    var_handlers: Vec<VarHandler>,
    /// If this helper is currently connected to the game or not.
    connected: bool,
    /// Last update number of the session data.
    session_data_last_udpate: i32,
    /// The current session data.
    session_data: StaticData,
}

impl Drop for Irsdk {
    fn drop(&mut self) {
        unsafe {
            UnmapViewOfFile(self.view as *const c_void);
            CloseHandle(self.file_mapping);
            CloseHandle(self.data_valid_event);
        };
    }
}

impl Irsdk {
    /// Create a new instance of the iracing sdk.
    /// Returns `Err` if the shared memory file mapping cannot be created.
    pub fn new() -> Result<Self, windows::core::Error> {
        // SAFETY: If this function failes it returns `null`; we must check for that case.
        let handle =
            unsafe { OpenFileMappingW(FILE_MAP_READ.0, false, w!("Local\\IRSDKMemMapFileName")) }?;
        if handle.is_invalid() {
            return Err(windows::core::Error::from_win32());
        }

        // SAFETY: The returned pointer may be null to indicate that the operation has failed
        // and needs to be checked.
        let view = unsafe { MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0) as *const u8 };
        if view.is_null() {
            return Err(windows::core::Error::from_win32());
        }

        // SAFETY: The returned handle can be invalid and needs to be checked.
        let data_valid_event = unsafe {
            OpenEventW(
                SYNCHRONIZATION_SYNCHRONIZE,
                false,
                w!("Local\\IRSDKDataValidEvent"),
            )
        }?;
        if data_valid_event.is_invalid() {
            return Err(windows::core::Error::from_win32());
        }

        // SAFETY: If the function fails the returned id is 0.
        let message_id = unsafe { RegisterWindowMessageW(w!("IRSDK_BROADCASTMSG")) };
        if message_id == 0 {
            return Err(windows::core::Error::from_win32());
        }

        Ok(Self {
            file_mapping: handle,
            view,
            _last_tick_count: 0,
            var_handlers: Vec::new(),
            connected: false,
            session_data_last_udpate: 0,
            session_data: StaticData::default(),
            data_valid_event,
            message_id,
        })
    }

    pub fn send_message(&self, message: Messages) {
        let (p1, p2) = message.map_to_paramters();
        unsafe {
            SendNotifyMessageW(
                BROADCAST_HANDLE,
                self.message_id,
                WPARAM(p1 as usize),
                LPARAM(p2 as isize),
            )
        };
    }

    /// Wait for the data update signal with a maximum timeout.
    pub fn wait_for_update(&self, timeout_ms: u32) -> Result<(), WaitError> {
        // SAFETY: The data_valid_event must be a valid handle. This is
        // check in the constructor.
        let status = unsafe { WaitForSingleObject(self.data_valid_event, timeout_ms) };
        match status {
            WAIT_OBJECT_0 => Ok(()),
            WAIT_TIMEOUT => Err(WaitError::Timeout),
            // This error is related to when the object is a mutex.
            // Since this is not the case this is unreachable.
            WAIT_ABANDONED => unreachable!(),
            WAIT_FAILED => Err(WaitError::Win32Error(windows::core::Error::from_win32())),
            // The returned status is only a subset of all possible errors and is specified
            // in the win32 docs.
            _ => unreachable!(),
        }
    }

    pub fn poll(&mut self) -> Result<Data, PollError> {
        // SAFETY: The pointer has been checked to be not null.
        // A Header struct is plain data and for all fields any bit pattern is a vlaid value.
        // Therefore dereferencing is fine.
        // `Header` must also be repr C.
        let header = unsafe { &*(self.view as *const Header) };

        let is_connected = header.status.contains(StatusField::CONNECTED);
        if !is_connected {
            self.connected = false;
            return Err(PollError::NotConnected);
        }

        let is_new_connection = !self.connected && is_connected;
        self.connected = is_connected;

        // Read session data
        let session_str_changed = header.session_data_update != self.session_data_last_udpate;
        if session_str_changed || is_new_connection {
            self.parse_session_str(header);
        }

        // Process variable headers.
        let var_headers_changed = header.var_header_element_count != self.var_handlers.len() as i32;
        if is_new_connection || var_headers_changed {
            self.parse_var_headers(header);
        }

        let mut data = Data {
            static_data: self.session_data.clone(),
            ..Default::default()
        };

        // Read var buffer
        self.parse_var_buffer(header, &mut data);

        Ok(data)
    }

    fn parse_session_str(&mut self, header: &Header) {
        debug!("Process session data");
        self.session_data_last_udpate = header.session_data_update;
        let session_str_buffer = unsafe {
            slice::from_raw_parts(
                self.view.offset(header.session_data_offset as isize),
                header.session_data_len as usize,
            )
            .to_vec()
        };
        let session_str = CP1252
            .decode(&session_str_buffer)
            .trim_matches('\0')
            .to_string();
        let session_data = serde_yaml::from_str::<StaticData>(&session_str);
        if let Err(ref e) = session_data {
            warn!(
                "Error parsing session data yaml. Using default instead: {}",
                e
            );
        }
        // TODO: This should probably create an error instead of using the default.
        self.session_data = session_data.unwrap_or_default();
        self.session_data.update_count = header.session_data_update;
        for entry in self.session_data.get_unmapped().iter() {
            warn!("Unmapped field in session string: {:?}", entry);
        }
    }

    fn parse_var_headers(&mut self, header: &Header) {
        debug!("Parsing variable headers");
        let var_headers = unsafe {
            slice::from_raw_parts(
                self.view.offset(header.var_header_offset as isize) as *const VarHeader,
                header.var_header_element_count as usize,
            )
            .to_vec()
        };
        self.var_handlers.clear();
        for header in var_headers {
            let name = String::from_utf8_lossy(&header.name)
                .trim_matches(char::from(0))
                .to_owned();

            let processor = map_processors(&name);
            if let Processor::None = processor {
                let desc = String::from_utf8_lossy(&header.description)
                    .trim_matches(char::from(0))
                    .to_owned();
                let unit = String::from_utf8_lossy(&header.unit)
                    .trim_matches(char::from(0))
                    .to_owned();
                info!("Unmapped variable \"{name}\".\ndesc: {desc}\n:unit: {unit}\n type: {:?}, count: {}" , header.var_type, header.count);
            }

            self.var_handlers.push(VarHandler { header, processor });
        }
    }

    fn parse_var_buffer(&self, header: &Header, data: &mut Data) {
        let var_buffer = {
            let newest_buffer = header
                .var_buffers
                .iter()
                .max_by(|b1, b2| b1.tick_count.cmp(&b2.tick_count))
                .expect("The iterate should not be empty");
            let current_tick_count = newest_buffer.tick_count;
            let var_buffer = unsafe {
                slice::from_raw_parts(
                    self.view.offset(newest_buffer.offset as isize),
                    header.var_buffer_len as usize,
                )
                .to_vec()
            };
            if newest_buffer.tick_count != current_tick_count {
                warn!("The variable buffer has changed while reading");
            }
            var_buffer
        };

        // Write variables into data struct.
        for handler in self.var_handlers.iter() {
            handler.process(&var_buffer, &mut data.live_data);
        }
    }
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

/// A handler to read a variable from the var buffer and write its data into the model.
#[derive(Debug)]
pub struct VarHandler {
    header: VarHeader,
    processor: Processor,
}

impl VarHandler {
    fn process(&self, buffer: &[u8], data: &mut LiveData) {
        let offset = self.header.offset as usize;
        let count = self.header.count as usize;
        let size = self.processor.size();

        if buffer.len() < offset + size * count {
            warn!(
                "Buffer is to small for var buffer len: {}, header: {:?}",
                buffer.len(),
                self
            );
            return;
        }
        let raw = &buffer[offset..(offset + size * count)];

        match &self.processor {
            Processor::I32(p) => {
                let value = i32::from_le_bytes(raw[0..4].try_into().unwrap());
                p(data, value);
            }
            Processor::F64(p) => {
                let value = f64::from_le_bytes(raw[0..8].try_into().unwrap());
                p(data, value);
            }
            Processor::U8(p) => {
                let value = u8::from_le_bytes(raw[0..1].try_into().unwrap());
                p(data, value);
            }
            Processor::Bool(p) => {
                let value = raw[0] > 0;
                p(data, value);
            }
            Processor::F32(p) => {
                let value = f32::from_le_bytes(raw[0..4].try_into().unwrap());
                p(data, value);
            }
            Processor::VecI32(p) => {
                let mut vec = Vec::new();
                for i in 0..count {
                    let bytes = &raw[i * size..i * size + size];
                    let value = i32::from_le_bytes(bytes.try_into().unwrap());
                    vec.push(value);
                }
                p(data, vec);
            }
            Processor::VecF32(p) => {
                let mut vec = Vec::new();
                for i in 0..count {
                    let bytes = &raw[i * size..i * size + size];
                    let value = f32::from_le_bytes(bytes.try_into().unwrap());
                    vec.push(value);
                }
                p(data, vec);
            }
            Processor::VecU8(p) => {
                let mut vec = Vec::new();
                for i in 0..count {
                    let bytes = &raw[i * size..i * size + size];
                    vec.push(bytes[0]);
                }
                p(data, vec);
            }
            Processor::VecBool(p) => {
                let mut vec = Vec::new();
                for i in 0..count {
                    let bytes = &raw[i * size..i * size + size];
                    vec.push(bytes[0] > 0);
                }
                p(data, vec);
            }
            Processor::None => (),
        }
    }
}

/// Types of processors to process differnt types of variables.
#[allow(clippy::type_complexity)]
pub enum Processor {
    U8(Box<dyn Fn(&mut LiveData, u8)>),
    VecU8(Box<dyn Fn(&mut LiveData, Vec<u8>)>),
    Bool(Box<dyn Fn(&mut LiveData, bool)>),
    VecBool(Box<dyn Fn(&mut LiveData, Vec<bool>)>),
    I32(Box<dyn Fn(&mut LiveData, i32)>),
    VecI32(Box<dyn Fn(&mut LiveData, Vec<i32>)>),
    F32(Box<dyn Fn(&mut LiveData, f32)>),
    VecF32(Box<dyn Fn(&mut LiveData, Vec<f32>)>),
    F64(Box<dyn Fn(&mut LiveData, f64)>),
    None,
}

impl Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Processor::I32(_) => write!(f, "i32"),
            Processor::F64(_) => write!(f, "f64"),
            Processor::VecI32(_) => write!(f, "Vec<i32>"),
            Processor::None => write!(f, "None"),
            Processor::U8(_) => write!(f, "u8"),
            Processor::VecU8(_) => write!(f, "Vec<u8>"),
            Processor::Bool(_) => write!(f, "bool"),
            Processor::VecBool(_) => write!(f, "Vec<bool>"),
            Processor::F32(_) => write!(f, "f32"),
            Processor::VecF32(_) => write!(f, "Vec<f32>"),
        }
    }
}

impl Processor {
    /// Returns the number of bytes required to create a value for this processor.
    /// In case of vector types this is the size of a single element.
    fn size(&self) -> usize {
        match self {
            Processor::I32(_) => 4,
            Processor::VecI32(_) => 4,
            Processor::F64(_) => 8,
            Processor::None => 0,
            Processor::U8(_) => 1,
            Processor::VecU8(_) => 1,
            Processor::Bool(_) => 1,
            Processor::VecBool(_) => 1,
            Processor::F32(_) => 4,
            Processor::VecF32(_) => 4,
        }
    }
    #[allow(dead_code)]
    fn u8(target: impl Fn(&mut LiveData, u8) + 'static) -> Self {
        Processor::U8(Box::new(target))
    }
    fn bool(target: impl Fn(&mut LiveData, bool) + 'static) -> Self {
        Processor::Bool(Box::new(target))
    }
    fn i32(target: impl Fn(&mut LiveData, i32) + 'static) -> Self {
        Processor::I32(Box::new(target))
    }
    fn f32(target: impl Fn(&mut LiveData, f32) + 'static) -> Self {
        Processor::F32(Box::new(target))
    }
    fn f64(target: impl Fn(&mut LiveData, f64) + 'static) -> Self {
        Processor::F64(Box::new(target))
    }
    fn vec_i32(target: impl Fn(&mut LiveData, Vec<i32>) + 'static) -> Self {
        Processor::VecI32(Box::new(target))
    }
    fn vec_f32(target: impl Fn(&mut LiveData, Vec<f32>) + 'static) -> Self {
        Processor::VecF32(Box::new(target))
    }
    #[allow(dead_code)]
    fn vec_u8(target: impl Fn(&mut LiveData, Vec<u8>) + 'static) -> Self {
        Processor::VecU8(Box::new(target))
    }
    fn vec_bool(target: impl Fn(&mut LiveData, Vec<bool>) + 'static) -> Self {
        Processor::VecBool(Box::new(target))
    }
}

fn map_processors(name: &str) -> Processor {
    match name {
        "SessionTime" => Processor::f64(|d, v| d.session_time = Some(Time::from_secs(v))), //s
        "SessionTick" => Processor::i32(|d, v| d.session_tick = Some(v)),
        "SessionNum" => Processor::i32(|d, v| d.session_num = Some(v)),
        "SessionState" => Processor::i32(|d, v| d.session_state = Some(v.into())),
        "SessionUniqueID" => Processor::i32(|d, v| d.session_unique_id = Some(v)),
        "SessionFlags" => {
            Processor::i32(|d, v| d.session_flags = Some(Flags::from_bits_retain(v as u32)))
        }
        "SessionTimeRemain" => {
            Processor::f64(|d, v| d.session_time_remain = Some(Time::from_secs(v)))
        } //s
        "SessionLapsRemain" => Processor::i32(|d, v| d.session_laps_remain = Some(v)),
        "SessionLapsRemainEx" => Processor::i32(|d, v| d.session_laps_remain_ex = Some(v)),
        "SessionTimeTotal" => {
            Processor::f64(|d, v| d.session_time_total = Some(Time::from_secs(v)))
        } //s
        "SessionLapsTotal" => Processor::i32(|d, v| d.session_laps_total = Some(v)),
        "SessionJokerLapsRemain" => Processor::i32(|d, v| d.session_joker_laps_remain = Some(v)),
        "SessionOnJokerLap" => Processor::bool(|d, v| d.session_on_joker_lap = Some(v)),
        "SessionTimeOfDay" => {
            Processor::f32(|d, v| d.session_time_of_day = Some(Time::from_secs(v)))
        } //s
        "RadioTransmitCarIdx" => Processor::i32(|d, v| d.radio_transmit_car_idx = Some(v)),
        "RadioTransmitRadioIdx" => Processor::i32(|d, v| d.radio_transmit_radio_idx = Some(v)),
        "RadioTransmitFrequencyIdx" => {
            Processor::i32(|d, v| d.radio_transmit_frequency_idx = Some(v))
        }
        "DisplayUnits" => Processor::i32(|d, v| d.display_units = Some(v)),
        "DriverMarker" => Processor::bool(|d, v| d.driver_marker = Some(v)),
        "PushToTalk" => Processor::bool(|d, v| d.push_to_talk = Some(v)),
        "PushToPass" => Processor::bool(|d, v| d.push_to_pass = Some(v)),
        "ManualBoost" => Processor::bool(|d, v| d.manual_boost = Some(v)),
        "ManualNoBoost" => Processor::bool(|d, v| d.manual_no_boost = Some(v)),
        "IsOnTrack" => Processor::bool(|d, v| d.is_on_track = Some(v)),
        "IsReplayPlaying" => Processor::bool(|d, v| d.is_replay_playing = Some(v)),
        "ReplayFrameNum" => Processor::i32(|d, v| d.replay_frame_num = Some(v)),
        "ReplayFrameNumEnd" => Processor::i32(|d, v| d.replay_frame_num_end = Some(v)),
        "IsDiskLoggingEnabled" => Processor::bool(|d, v| d.is_disk_logging_enabled = Some(v)),
        "IsDiskLoggingActive" => Processor::bool(|d, v| d.is_disk_logging_active = Some(v)),
        "FrameRate" => Processor::f32(|d, v| d.frame_rate = Some(v)), //fps
        "CpuUsageFG" => Processor::f32(|d, v| d.cpu_usage_fg = Some(v)), //%
        "GpuUsage" => Processor::f32(|d, v| d.gpu_usage = Some(v)),   //%
        "ChanAvgLatency" => Processor::f32(|d, v| d.chan_avg_latency = Some(Time::from_secs(v))), //s
        "ChanLatency" => Processor::f32(|d, v| d.chan_latency = Some(Time::from_secs(v))), //s
        "ChanQuality" => Processor::f32(|d, v| d.chan_quality = Some(v)),                  //%
        "ChanPartnerQuality" => Processor::f32(|d, v| d.chan_partner_quality = Some(v)),   //%
        "CpuUsageBG" => Processor::f32(|d, v| d.cpu_usage_bg = Some(v)),                   //%
        "ChanClockSkew" => Processor::f32(|d, v| d.chan_clock_skew = Some(Time::from_secs(v))), //s
        "MemPageFaultSec" => Processor::f32(|d, v| d.mem_page_fault_sec = Some(v)),
        "MemSoftPageFaultSec" => Processor::f32(|d, v| d.mem_soft_page_fault_sec = Some(v)),
        "PlayerCarPosition" => Processor::i32(|d, v| d.player_car_position = Some(v)),
        "PlayerCarClassPosition" => Processor::i32(|d, v| d.player_car_class_position = Some(v)),
        "PlayerCarClass" => Processor::i32(|d, v| d.player_car_class = Some(v)),
        "PlayerTrackSurface" => Processor::i32(|d, v| d.player_track_surface = Some(v)),
        "PlayerTrackSurfaceMaterial" => {
            Processor::i32(|d, v| d.player_track_surface_material = Some(v))
        }
        "PlayerCarIdx" => Processor::i32(|d, v| d.player_car_idx = Some(v)),
        "PlayerCarTeamIncidentCount" => {
            Processor::i32(|d, v| d.player_car_team_incident_count = Some(v))
        }
        "PlayerCarMyIncidentCount" => {
            Processor::i32(|d, v| d.player_car_my_incident_count = Some(v))
        }
        "PlayerCarDriverIncidentCount" => {
            Processor::i32(|d, v| d.player_car_driver_incident_count = Some(v))
        }
        "PlayerCarWeightPenalty" => Processor::f32(|d, v| d.player_car_weight_penalty = Some(v)), //kg
        "PlayerCarPowerAdjust" => Processor::f32(|d, v| d.player_car_power_adjust = Some(v)), //%
        "PlayerCarDryTireSetLimit" => {
            Processor::i32(|d, v| d.player_car_dry_tire_set_limit = Some(v))
        }
        "PlayerCarTowTime" => {
            Processor::f32(|d, v| d.player_car_tow_time = Some(Time::from_secs(v)))
        } //s
        "PlayerCarInPitStall" => Processor::bool(|d, v| d.player_car_in_pit_stall = Some(v)),
        "PlayerCarPitSvStatus" => {
            Processor::i32(|d, v| d.player_car_pit_sv_status = Some(v))
        }
        "PlayerTireCompound" => Processor::i32(|d, v| d.player_tire_compound = Some(v)),
        "PlayerFastRepairsUsed" => Processor::i32(|d, v| d.player_fast_repairs_used = Some(v)),
        "CarIdxLap" => Processor::vec_i32(|d, v| d.car_idx_lap = Some(v)),
        "CarIdxLapCompleted" => Processor::vec_i32(|d, v| d.car_idx_lap_completed = Some(v)),
        "CarIdxLapDistPct" => Processor::vec_f32(|d, v| d.car_idx_lap_dist_pct = Some(v)),
        "CarIdxTrackSurface" => Processor::vec_i32(|d, v| {
            d.car_idx_track_surface = Some(v.iter().map(|v| TrkLoc::from(*v)).collect())
        }), //irsdk_TrkLoc
        "CarIdxTrackSurfaceMaterial" => Processor::vec_i32(|d, v| {
            d.car_idx_track_surface_material = Some(v.iter().map(|v| TrkSurf::from(*v)).collect())
        }), //irsdk_TrkSurf
        "CarIdxOnPitRoad" => Processor::vec_bool(|d, v| d.car_idx_on_pit_road = Some(v)),
        "CarIdxPosition" => Processor::vec_i32(|d, v| d.car_idx_position = Some(v)),
        "CarIdxClassPosition" => Processor::vec_i32(|d, v| d.car_idx_class_position = Some(v)),
        "CarIdxClass" => Processor::vec_i32(|d, v| d.car_idx_class = Some(v)),
        "CarIdxF2Time" => Processor::vec_f32(|d, v| {
            d.car_idx_f2_time = Some(v.iter().map(|v| Time::from_secs(*v)).collect())
        }), //s
        "CarIdxEstTime" => Processor::vec_f32(|d, v| {
            d.car_idx_est_time = Some(v.iter().map(|v| Time::from_secs(*v)).collect())
        }), //s
        "CarIdxLastLapTime" => Processor::vec_f32(|d, v| {
            d.car_idx_last_lap_time = Some(v.iter().map(|v| Time::from_secs(*v)).collect())
        }), //s
        "CarIdxBestLapTime" => Processor::vec_f32(|d, v| {
            d.car_idx_best_lap_time = Some(v.iter().map(|v| Time::from_secs(*v)).collect())
        }), //s
        "CarIdxBestLapNum" => Processor::vec_i32(|d, v| d.car_idx_best_lap_num = Some(v)),
        "CarIdxTireCompound" => Processor::vec_i32(|d, v| d.car_idx_tire_compound = Some(v)),
        "CarIdxQualTireCompound" => {
            Processor::vec_i32(|d, v| d.car_idx_qual_tire_compound = Some(v))
        }
        "CarIdxQualTireCompoundLocked" => {
            Processor::vec_bool(|d, v| d.car_idx_qual_tire_compound_locked = Some(v))
        }
        "CarIdxFastRepairsUsed" => Processor::vec_i32(|d, v| d.car_idx_fast_repairs_used = Some(v)),
        "CarIdxSessionFlags" => Processor::vec_i32(|d, v| {
            d.car_idx_session_flags = Some(
                v.iter()
                    .map(|v| Flags::from_bits_retain(*v as u32))
                    .collect(),
            )
        }),
        "PaceMode" => Processor::i32(|d, v| d.pace_mode = Some(v)), //irsdk_PaceMode
        "CarIdxPaceLine" => Processor::vec_i32(|d, v| d.car_idx_pace_line = Some(v)),
        "CarIdxPaceRow" => Processor::vec_i32(|d, v| d.car_idx_pace_row = Some(v)),
        "CarIdxPaceFlags" => Processor::vec_i32(|d, v| {
            d.car_idx_pace_flags = Some(
                v.iter()
                    .map(|v| PaceFlags::from_bits_retain(*v as u32))
                    .collect(),
            )
        }), //irsdk_PaceFlags
        "OnPitRoad" => Processor::bool(|d, v| d.on_pit_road = Some(v)),
        "CarIdxSteer" => Processor::vec_f32(|d, v| d.car_idx_steer = Some(v)), //rad
        "CarIdxRPM" => Processor::vec_f32(|d, v| d.car_idx_rpm = Some(v)),     //revs/min
        "CarIdxGear" => Processor::vec_i32(|d, v| d.car_idx_gear = Some(v)),
        "SteeringWheelAngle" => Processor::f32(|d, v| d.steering_wheel_angle = Some(v)), //rad
        "Throttle" => Processor::f32(|d, v| d.throttle = Some(v)),                       //%
        "Brake" => Processor::f32(|d, v| d.brake = Some(v)),                             //%
        "Clutch" => Processor::f32(|d, v| d.clutch = Some(v)),                           //%
        "Gear" => Processor::i32(|d, v| d.gear = Some(v)),
        "RPM" => Processor::f32(|d, v| d.rpm = Some(v)), //revs/min
        "Lap" => Processor::i32(|d, v| d.lap = Some(v)),
        "LapCompleted" => Processor::i32(|d, v| d.lap_completed = Some(v)),
        "LapDist" => Processor::f32(|d, v| d.lap_dist = Some(v)), //m
        "LapDistPct" => Processor::f32(|d, v| d.lap_dist_pct = Some(v)), //%
        "RaceLaps" => Processor::i32(|d, v| d.race_laps = Some(v)),
        "LapBestLap" => Processor::i32(|d, v| d.lap_best_lap = Some(v)),
        "LapBestLapTime" => Processor::f32(|d, v| d.lap_best_lap_time = Some(Time::from_secs(v))), //s
        "LapLastLapTime" => Processor::f32(|d, v| d.lap_last_lap_time = Some(Time::from_secs(v))), //s
        "LapCurrentLapTime" => {
            Processor::f32(|d, v| d.lap_current_lap_time = Some(Time::from_secs(v)))
        } //s
        "LapLasNLapSeq" => Processor::i32(|d, v| d.lap_las_n_lap_seq = Some(v)),
        "LapLastNLapTime" => {
            Processor::f32(|d, v| d.lap_last_n_lap_time = Some(Time::from_secs(v)))
        } //s
        "LapBestNLapLap" => Processor::i32(|d, v| d.lap_best_n_lap_lap = Some(v)),
        "LapBestNLapTime" => {
            Processor::f32(|d, v| d.lap_best_n_lap_time = Some(Time::from_secs(v)))
        } //s
        "LapDeltaToBestLap" => {
            Processor::f32(|d, v| d.lap_delta_to_best_lap = Some(Time::from_secs(v)))
        } //s
        "LapDeltaToBestLap_DD" => Processor::f32(|d, v| d.lap_delta_to_best_lap_dd = Some(v)), //s/s
        "LapDeltaToBestLap_OK" => Processor::bool(|d, v| d.lap_delta_to_best_lap_ok = Some(v)),
        "LapDeltaToOptimalLap" => {
            Processor::f32(|d, v| d.lap_delta_to_optimal_lap = Some(Time::from_secs(v)))
        } //s
        "LapDeltaToOptimalLap_DD" => Processor::f32(|d, v| d.lap_delta_to_optimal_lap_dd = Some(v)), //s/s
        "LapDeltaToOptimalLap_OK" => {
            Processor::bool(|d, v| d.lap_delta_to_optimal_lap_ok = Some(v))
        }
        "LapDeltaToSessionBestLap" => {
            Processor::f32(|d, v| d.lap_delta_to_session_best_lap = Some(Time::from_secs(v)))
        } //s
        "LapDeltaToSessionBestLap_DD" => {
            Processor::f32(|d, v| d.lap_delta_to_session_best_lap_dd = Some(v))
        } //s/s
        "LapDeltaToSessionBestLap_OK" => {
            Processor::bool(|d, v| d.lap_delta_to_session_best_lap_ok = Some(v))
        }
        "LapDeltaToSessionOptimalLap" => {
            Processor::f32(|d, v| d.lap_delta_to_session_optimal_lap = Some(Time::from_secs(v)))
        } //s
        "LapDeltaToSessionOptimalLap_DD" => {
            Processor::f32(|d, v| d.lap_delta_to_session_optimal_lap_dd = Some(v))
        } //s/s
        "LapDeltaToSessionOptimalLap_OK" => {
            Processor::bool(|d, v| d.lap_delta_to_session_optimal_lap_ok = Some(v))
        }
        "LapDeltaToSessionLastlLap" => {
            Processor::f32(|d, v| d.lap_delta_to_session_lastl_lap = Some(Time::from_secs(v)))
        } //s
        "LapDeltaToSessionLastlLap_DD" => {
            Processor::f32(|d, v| d.lap_delta_to_session_lastl_lap_dd = Some(v))
        } //s/s
        "LapDeltaToSessionLastlLap_OK" => {
            Processor::bool(|d, v| d.lap_delta_to_session_lastl_lap_ok = Some(v))
        }
        "Speed" => Processor::f32(|d, v| d.speed = Some(v)), //m/s
        "Yaw" => Processor::f32(|d, v| d.yaw = Some(v)),     //rad
        "YawNorth" => Processor::f32(|d, v| d.yaw_north = Some(v)), //rad
        "Pitch" => Processor::f32(|d, v| d.pitch = Some(v)), //rad
        "Roll" => Processor::f32(|d, v| d.roll = Some(v)),   //rad
        "EnterExitReset" => Processor::i32(|d, v| d.enter_exit_reset = Some(v)),
        "TrackTemp" => Processor::f32(|d, v| d.track_temp = Some(v)), //C
        "TrackTempCrew" => Processor::f32(|d, v| d.track_temp_crew = Some(v)), //C
        "AirTemp" => Processor::f32(|d, v| d.air_temp = Some(v)),     //C
        "WeatherType" => Processor::i32(|d, v| d.weather_type = Some(v)),
        "Skies" => Processor::i32(|d, v| d.skies = Some(v)),
        "AirDensity" => Processor::f32(|d, v| d.air_density = Some(v)), //kg/m^3
        "AirPressure" => Processor::f32(|d, v| d.air_pressure = Some(v)), //Hg
        "WindVel" => Processor::f32(|d, v| d.wind_vel = Some(v)),       //m/s
        "WindDir" => Processor::f32(|d, v| d.wind_dir = Some(v)),       //rad
        "RelativeHumidity" => Processor::f32(|d, v| d.relative_humidity = Some(v)), //%
        "FogLevel" => Processor::f32(|d, v| d.fog_level = Some(v)),     //%
        "SolarAltitude" => Processor::f32(|d, v| d.solar_altitude = Some(v)), //rad
        "SolarAzimuth" => Processor::f32(|d, v| d.solar_azimuth = Some(v)), //rad
        "DCLapStatus" => Processor::i32(|d, v| d.dc_lap_status = Some(v)),
        "DCDriversSoFar" => Processor::i32(|d, v| d.dc_drivers_so_far = Some(v)),
        "OkToReloadTextures" => Processor::bool(|d, v| d.ok_to_reload_textures = Some(v)),
        "LoadNumTextures" => Processor::bool(|d, v| d.load_num_textures = Some(v)),
        "CarLeftRight" => Processor::i32(|d, v| d.car_left_right = Some(v)),
        "PitsOpen" => Processor::bool(|d, v| d.pits_open = Some(v)),
        "VidCapEnabled" => Processor::bool(|d, v| d.vid_cap_enabled = Some(v)),
        "VidCapActive" => Processor::bool(|d, v| d.vid_cap_active = Some(v)),
        "PitRepairLeft" => Processor::f32(|d, v| d.pit_repair_left = Some(Time::from_secs(v))), //s
        "PitOptRepairLeft" => {
            Processor::f32(|d, v| d.pit_opt_repair_left = Some(Time::from_secs(v)))
        } //s
        "PitstopActive" => Processor::bool(|d, v| d.pitstop_active = Some(v)),
        "FastRepairUsed" => Processor::i32(|d, v| d.fast_repair_used = Some(v)),
        "FastRepairAvailable" => Processor::i32(|d, v| d.fast_repair_available = Some(v)),
        "LFTiresUsed" => Processor::i32(|d, v| d.lf_tires_used = Some(v)),
        "RFTiresUsed" => Processor::i32(|d, v| d.rf_tires_used = Some(v)),
        "LRTiresUsed" => Processor::i32(|d, v| d.lr_tires_used = Some(v)),
        "RRTiresUsed" => Processor::i32(|d, v| d.rr_tires_used = Some(v)),
        "LeftTireSetsUsed" => Processor::i32(|d, v| d.left_tire_sets_used = Some(v)),
        "RightTireSetsUsed" => Processor::i32(|d, v| d.right_tire_sets_used = Some(v)),
        "FrontTireSetsUsed" => Processor::i32(|d, v| d.front_tire_sets_used = Some(v)),
        "RearTireSetsUsed" => Processor::i32(|d, v| d.rear_tire_sets_used = Some(v)),
        "TireSetsUsed" => Processor::i32(|d, v| d.tire_sets_used = Some(v)),
        "LFTiresAvailable" => Processor::i32(|d, v| d.lf_tires_available = Some(v)),
        "RFTiresAvailable" => Processor::i32(|d, v| d.rf_tires_available = Some(v)),
        "LRTiresAvailable" => Processor::i32(|d, v| d.lr_tires_available = Some(v)),
        "RRTiresAvailable" => Processor::i32(|d, v| d.rr_tires_available = Some(v)),
        "LeftTireSetsAvailable" => Processor::i32(|d, v| d.left_tire_sets_available = Some(v)),
        "RightTireSetsAvailable" => Processor::i32(|d, v| d.right_tire_sets_available = Some(v)),
        "FrontTireSetsAvailable" => Processor::i32(|d, v| d.front_tire_sets_available = Some(v)),
        "RearTireSetsAvailable" => Processor::i32(|d, v| d.rear_tire_sets_available = Some(v)),
        "TireSetsAvailable" => Processor::i32(|d, v| d.tire_sets_available = Some(v)),
        "CamCarIdx" => Processor::i32(|d, v| d.cam_car_idx = Some(v)),
        "CamCameraNumber" => Processor::i32(|d, v| d.cam_camera_number = Some(v)),
        "CamGroupNumber" => Processor::i32(|d, v| d.cam_group_number = Some(v)),
        "CamCameraState" => Processor::i32(|d, v| {
            d.cam_camera_state = Some(CameraState::from_bits_retain(v as u32))
        }),
        "IsOnTrackCar" => Processor::bool(|d, v| d.is_on_track_car = Some(v)),
        "IsInGarage" => Processor::bool(|d, v| d.is_in_garage = Some(v)),
        "SteeringWheelAngleMax" => Processor::f32(|d, v| d.steering_wheel_angle_max = Some(v)), //rad
        "ShiftPowerPct" => Processor::f32(|d, v| d.shift_power_pct = Some(v)),                  //%
        "ShiftGrindRPM" => Processor::f32(|d, v| d.shift_grind_rpm = Some(v)), //RPM
        "ThrottleRaw" => Processor::f32(|d, v| d.throttle_raw = Some(v)),      //%
        "BrakeRaw" => Processor::f32(|d, v| d.brake_raw = Some(v)),            //%
        "ClutchRaw" => Processor::f32(|d, v| d.clutch_raw = Some(v)),          //%
        "HandbrakeRaw" => Processor::f32(|d, v| d.handbrake_raw = Some(v)),    //%
        "BrakeABSactive" => Processor::bool(|d, v| d.brake_ab_sactive = Some(v)),
        "EngineWarnings" => Processor::i32(|d, v| {
            d.engine_warnings = Some(EngineWarnings::from_bits_retain(v as u32))
        }),
        "FuelLevelPct" => Processor::f32(|d, v| d.fuel_level_pct = Some(v)), //%
        "PitSvFlags" => {
            Processor::i32(|d, v| d.pit_sv_flags = Some(PitSvFlags::from_bits_retain(v as u32)))
        }
        "PitSvLFP" => Processor::f32(|d, v| d.pit_sv_lfp = Some(v)), //kPa
        "PitSvRFP" => Processor::f32(|d, v| d.pit_sv_rfp = Some(v)), //kPa
        "PitSvLRP" => Processor::f32(|d, v| d.pit_sv_lrp = Some(v)), //kPa
        "PitSvRRP" => Processor::f32(|d, v| d.pit_sv_rrp = Some(v)), //kPa
        "PitSvFuel" => Processor::f32(|d, v| d.pit_sv_fuel = Some(v)), //l or kWh
        "PitSvTireCompound" => Processor::i32(|d, v| d.pit_sv_tire_compound = Some(v)),
        "CarIdxP2P_Status" => Processor::vec_bool(|d, v| d.car_idx_p2p_status = Some(v)),
        "CarIdxP2P_Count" => Processor::vec_i32(|d, v| d.car_idx_p2p_count = Some(v)),
        "SteeringWheelPctTorque" => Processor::f32(|d, v| d.steering_wheel_pct_torque = Some(v)), //%
        "SteeringWheelPctTorqueSign" => {
            Processor::f32(|d, v| d.steering_wheel_pct_torque_sign = Some(v))
        } //%
        "SteeringWheelPctTorqueSignStops" => {
            Processor::f32(|d, v| d.steering_wheel_pct_torque_sign_stops = Some(v))
        } //%
        "SteeringWheelPctSmoothing" => {
            Processor::f32(|d, v| d.steering_wheel_pct_smoothing = Some(v))
        } //%
        "SteeringWheelPctDamper" => Processor::f32(|d, v| d.steering_wheel_pct_damper = Some(v)), //%
        "SteeringWheelLimiter" => Processor::f32(|d, v| d.steering_wheel_limiter = Some(v)), //%
        "SteeringWheelMaxForceNm" => Processor::f32(|d, v| d.steering_wheel_max_force_nm = Some(v)), //N*m
        "SteeringWheelPeakForceNm" => {
            Processor::f32(|d, v| d.steering_wheel_peak_force_nm = Some(v))
        } //N*m
        "SteeringWheelUseLinear" => Processor::bool(|d, v| d.steering_wheel_use_linear = Some(v)),
        "ShiftIndicatorPct" => Processor::f32(|d, v| d.shift_indicator_pct = Some(v)), //%
        "ReplayPlaySpeed" => Processor::i32(|d, v| d.replay_play_speed = Some(v)),
        "ReplayPlaySlowMotion" => Processor::bool(|d, v| d.replay_play_slow_motion = Some(v)),
        "ReplaySessionTime" => {
            Processor::f64(|d, v| d.replay_session_time = Some(Time::from_secs(v)))
        } //s
        "ReplaySessionNum" => Processor::i32(|d, v| d.replay_session_num = Some(v)),
        "TireLF_RumblePitch" => Processor::f32(|d, v| d.tire_lf_rumble_pitch = Some(v)), //Hz
        "TireRF_RumblePitch" => Processor::f32(|d, v| d.tire_rf_rumble_pitch = Some(v)), //Hz
        "TireLR_RumblePitch" => Processor::f32(|d, v| d.tire_lr_rumble_pitch = Some(v)), //Hz
        "TireRR_RumblePitch" => Processor::f32(|d, v| d.tire_rr_rumble_pitch = Some(v)), //Hz
        "IsGarageVisible" => Processor::bool(|d, v| d.is_garage_visible = Some(v)),
        "SteeringWheelTorque_ST" => Processor::vec_f32(|d, v| d.steering_wheel_torque_st = Some(v)), //N*m
        "SteeringWheelTorque" => Processor::f32(|d, v| d.steering_wheel_torque = Some(v)), //N*m
        "VelocityZ_ST" => Processor::vec_f32(|d, v| d.velocity_z_st = Some(v)), //m/s at 360 Hz
        "VelocityY_ST" => Processor::vec_f32(|d, v| d.velocity_y_st = Some(v)), //m/s at 360 Hz
        "VelocityX_ST" => Processor::vec_f32(|d, v| d.velocity_x_st = Some(v)), //m/s at 360 Hz
        "VelocityZ" => Processor::f32(|d, v| d.velocity_z = Some(v)),           //m/s
        "VelocityY" => Processor::f32(|d, v| d.velocity_y = Some(v)),           //m/s
        "VelocityX" => Processor::f32(|d, v| d.velocity_x = Some(v)),           //m/s
        "YawRate_ST" => Processor::vec_f32(|d, v| d.yaw_rate_st = Some(v)),     //rad/s
        "PitchRate_ST" => Processor::vec_f32(|d, v| d.pitch_rate_st = Some(v)), //rad/s
        "RollRate_ST" => Processor::vec_f32(|d, v| d.roll_rate_st = Some(v)),   //rad/s
        "YawRate" => Processor::f32(|d, v| d.yaw_rate = Some(v)),               //rad/s
        "PitchRate" => Processor::f32(|d, v| d.pitch_rate = Some(v)),           //rad/s
        "RollRate" => Processor::f32(|d, v| d.roll_rate = Some(v)),             //rad/s
        "VertAccel_ST" => Processor::vec_f32(|d, v| d.vert_accel_st = Some(v)), //m/s^2
        "LatAccel_ST" => Processor::vec_f32(|d, v| d.lat_accel_st = Some(v)),   //m/s^2
        "LongAccel_ST" => Processor::vec_f32(|d, v| d.long_accel_st = Some(v)), //m/s^2
        "VertAccel" => Processor::f32(|d, v| d.vert_accel = Some(v)),           //m/s^2
        "LatAccel" => Processor::f32(|d, v| d.lat_accel = Some(v)),             //m/s^2
        "LongAccel" => Processor::f32(|d, v| d.long_accel = Some(v)),           //m/s^2
        "dcStarter" => Processor::bool(|d, v| d.dc_starter = Some(v)),
        "dcDashPage" => Processor::f32(|d, v| d.dc_dash_page = Some(v)),
        "dcTearOffVisor" => Processor::bool(|d, v| d.dc_tear_off_visor = Some(v)),
        "dpTireChange" => Processor::f32(|d, v| d.dp_tire_change = Some(v)),
        "dpFuelFill" => Processor::f32(|d, v| d.dp_fuel_fill = Some(v)),
        "dpFuelAddKg" => Processor::f32(|d, v| d.dp_fuel_add_kg = Some(v)), //kg
        "dpFastRepair" => Processor::f32(|d, v| d.dp_fast_repair = Some(v)),
        "dcBrakeBias" => Processor::f32(|d, v| d.dc_brake_bias = Some(v)),
        "dpLFTireColdPress" => Processor::f32(|d, v| d.dp_lf_tire_cold_press = Some(v)), //Pa
        "dpRFTireColdPress" => Processor::f32(|d, v| d.dp_rf_tire_cold_press = Some(v)), //Pa
        "dpLRTireColdPress" => Processor::f32(|d, v| d.dp_lr_tire_cold_press = Some(v)), //Pa
        "dpRRTireColdPress" => Processor::f32(|d, v| d.dp_rr_tire_cold_press = Some(v)), //Pa
        "RFbrakeLinePress" => Processor::f32(|d, v| d.r_fbrake_line_press = Some(v)),    //bar
        "RFcoldPressure" => Processor::f32(|d, v| d.r_fcold_pressure = Some(v)),         //kPa
        "RFtempCL" => Processor::f32(|d, v| d.r_ftemp_cl = Some(v)),                     //C
        "RFtempCM" => Processor::f32(|d, v| d.r_ftemp_cm = Some(v)),                     //C
        "RFtempCR" => Processor::f32(|d, v| d.r_ftemp_cr = Some(v)),                     //C
        "RFwearL" => Processor::f32(|d, v| d.r_fwear_l = Some(v)),                       //%
        "RFwearM" => Processor::f32(|d, v| d.r_fwear_m = Some(v)),                       //%
        "RFwearR" => Processor::f32(|d, v| d.r_fwear_r = Some(v)),                       //%
        "LFbrakeLinePress" => Processor::f32(|d, v| d.l_fbrake_line_press = Some(v)),    //bar
        "LFcoldPressure" => Processor::f32(|d, v| d.l_fcold_pressure = Some(v)),         //kPa
        "LFtempCL" => Processor::f32(|d, v| d.l_ftemp_cl = Some(v)),                     //C
        "LFtempCM" => Processor::f32(|d, v| d.l_ftemp_cm = Some(v)),                     //C
        "LFtempCR" => Processor::f32(|d, v| d.l_ftemp_cr = Some(v)),                     //C
        "LFwearL" => Processor::f32(|d, v| d.l_fwear_l = Some(v)),                       //%
        "LFwearM" => Processor::f32(|d, v| d.l_fwear_m = Some(v)),                       //%
        "LFwearR" => Processor::f32(|d, v| d.l_fwear_r = Some(v)),                       //%
        "FuelUsePerHour" => Processor::f32(|d, v| d.fuel_use_per_hour = Some(v)),        //kg/h
        "Voltage" => Processor::f32(|d, v| d.voltage = Some(v)),                         //V
        "WaterTemp" => Processor::f32(|d, v| d.water_temp = Some(v)),                    //C
        "WaterLevel" => Processor::f32(|d, v| d.water_level = Some(v)),                  //l
        "FuelPress" => Processor::f32(|d, v| d.fuel_press = Some(v)),                    //bar
        "OilTemp" => Processor::f32(|d, v| d.oil_temp = Some(v)),                        //C
        "OilPress" => Processor::f32(|d, v| d.oil_press = Some(v)),                      //bar
        "OilLevel" => Processor::f32(|d, v| d.oil_level = Some(v)),                      //l
        "ManifoldPress" => Processor::f32(|d, v| d.manifold_press = Some(v)),            //bar
        "FuelLevel" => Processor::f32(|d, v| d.fuel_level = Some(v)),                    //l
        "Engine0_RPM" => Processor::f32(|d, v| d.engine0_rpm = Some(v)),                 //revs/min
        "RRbrakeLinePress" => Processor::f32(|d, v| d.r_rbrake_line_press = Some(v)),    //bar
        "RRcoldPressure" => Processor::f32(|d, v| d.r_rcold_pressure = Some(v)),         //kPa
        "RRtempCL" => Processor::f32(|d, v| d.r_rtemp_cl = Some(v)),                     //C
        "RRtempCM" => Processor::f32(|d, v| d.r_rtemp_cm = Some(v)),                     //C
        "RRtempCR" => Processor::f32(|d, v| d.r_rtemp_cr = Some(v)),                     //C
        "RRwearL" => Processor::f32(|d, v| d.r_rwear_l = Some(v)),                       //%
        "RRwearM" => Processor::f32(|d, v| d.r_rwear_m = Some(v)),                       //%
        "RRwearR" => Processor::f32(|d, v| d.r_rwear_r = Some(v)),                       //%
        "LRbrakeLinePress" => Processor::f32(|d, v| d.l_rbrake_line_press = Some(v)),    //bar
        "LRcoldPressure" => Processor::f32(|d, v| d.l_rcold_pressure = Some(v)),         //kPa
        "LRtempCL" => Processor::f32(|d, v| d.l_rtemp_cl = Some(v)),                     //C
        "LRtempCM" => Processor::f32(|d, v| d.l_rtemp_cm = Some(v)),                     //C
        "LRtempCR" => Processor::f32(|d, v| d.l_rtemp_cr = Some(v)),                     //C
        "LRwearL" => Processor::f32(|d, v| d.l_rwear_l = Some(v)),                       //%
        "LRwearM" => Processor::f32(|d, v| d.l_rwear_m = Some(v)),                       //%
        "LRwearR" => Processor::f32(|d, v| d.l_rwear_r = Some(v)),                       //%
        "CRshockDefl" => Processor::f32(|d, v| d.c_rshock_defl = Some(v)),               //m
        "CRshockDefl_ST" => Processor::vec_f32(|d, v| d.c_rshock_defl_st = Some(v)),     //m
        "CRshockVel" => Processor::f32(|d, v| d.c_rshock_vel = Some(v)),                 //m/s
        "CRshockVel_ST" => Processor::vec_f32(|d, v| d.c_rshock_vel_st = Some(v)),       //m/s
        "LRshockDefl" => Processor::f32(|d, v| d.l_rshock_defl = Some(v)),               //m
        "LRshockDefl_ST" => Processor::vec_f32(|d, v| d.l_rshock_defl_st = Some(v)),     //m
        "LRshockVel" => Processor::f32(|d, v| d.l_rshock_vel = Some(v)),                 //m/s
        "LRshockVel_ST" => Processor::vec_f32(|d, v| d.l_rshock_vel_st = Some(v)),       //m/s
        "RRshockDefl" => Processor::f32(|d, v| d.r_rshock_defl = Some(v)),               //m
        "RRshockDefl_ST" => Processor::vec_f32(|d, v| d.r_rshock_defl_st = Some(v)),     //m
        "RRshockVel" => Processor::f32(|d, v| d.r_rshock_vel = Some(v)),                 //m/s
        "RRshockVel_ST" => Processor::vec_f32(|d, v| d.r_rshock_vel_st = Some(v)),       //m/s
        "LFshockDefl" => Processor::f32(|d, v| d.l_fshock_defl = Some(v)),               //m
        "LFshockDefl_ST" => Processor::vec_f32(|d, v| d.l_fshock_defl_st = Some(v)),     //m
        "LFshockVel" => Processor::f32(|d, v| d.l_fshock_vel = Some(v)),                 //m/s
        "LFshockVel_ST" => Processor::vec_f32(|d, v| d.l_fshock_vel_st = Some(v)),       //m/s
        "RFshockDefl" => Processor::f32(|d, v| d.r_fshock_defl = Some(v)),               //m
        "RFshockDefl_ST" => Processor::vec_f32(|d, v| d.r_fshock_defl_st = Some(v)),     //m
        "RFshockVel" => Processor::f32(|d, v| d.r_fshock_vel = Some(v)),                 //m/s
        "RFshockVel_ST" => Processor::vec_f32(|d, v| d.r_fshock_vel_st = Some(v)), //m/s        //m/s
        _ => Processor::None,
    }
}
