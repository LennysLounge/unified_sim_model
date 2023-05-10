use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct IncompleteTypeError {}

impl Display for IncompleteTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not enough data to parse this type")
    }
}

impl Error for IncompleteTypeError {}

#[derive(Debug)]
pub enum Response {
    Unknown(u8),
    RegistrationResult(RegistrationResult),
    RealtimeUpdate(RealtimeUpdate),
    RealtimeCarUpdate(RealtimeCarUpdate),
    EntryList(EntryList),
    TrackData(TrackData),
    EntryListCar(EntryListCar),
    BroadcastingEvent(BroadcastingEvent),
}

pub fn read_response(mut buf: &[u8]) -> Result<Response, IncompleteTypeError> {
    Ok(match read_u8(&mut buf)? {
        1 => read_registration_result(&mut buf)?,
        2 => read_realtime_update(&mut buf)?,
        3 => read_realtime_car_update(&mut buf)?,
        4 => read_entry_list(&mut buf)?,
        5 => read_track_data(&mut buf)?,
        6 => read_entry_list_car(&mut buf)?,
        7 => read_broadcasting_event(&mut buf)?,
        x => Response::Unknown(x),
    })
}

#[derive(Debug)]
pub struct RegistrationResult {
    pub connection_id: i32,
    pub success: bool,
    pub read_only: bool,
    pub message: String,
}

fn read_registration_result(buf: &mut &[u8]) -> Result<Response, IncompleteTypeError> {
    Ok(Response::RegistrationResult(RegistrationResult {
        connection_id: read_i32(buf)?,
        success: read_u8(buf)? > 0,
        read_only: read_u8(buf)? == 0,
        message: read_string(buf)?,
    }))
}

#[derive(Debug, Default)]
pub struct RealtimeUpdate {
    pub event_index: i16,
    pub session_index: i16,
    pub session_type: u8,
    pub session_phase: u8,
    pub session_time: f32,
    pub session_end_time: f32,
    pub focused_car_index: i32,
    pub active_camera_set: String,
    pub active_camera: String,
    pub current_hud_page: String,
    pub is_replay_playing: bool,
    pub replay_session_time: f32,
    pub replay_remaining_time: f32,
    pub time_of_day: f32,
    pub ambient_temp: u8,
    pub track_temp: u8,
    pub cloud_level: u8,
    pub rain_level: u8,
    pub wetness: u8,
    pub best_session_lap: LapInfo,
}

#[allow(clippy::field_reassign_with_default)]
fn read_realtime_update(buf: &mut &[u8]) -> Result<Response, IncompleteTypeError> {
    let mut me = RealtimeUpdate::default();
    me.event_index = read_i16(buf)?;
    me.session_index = read_i16(buf)?;
    me.session_type = read_u8(buf)?;
    me.session_phase = read_u8(buf)?;
    me.session_time = read_f32(buf)?;
    me.session_end_time = read_f32(buf)?;
    me.focused_car_index = read_i32(buf)?;
    me.active_camera_set = read_string(buf)?;
    me.active_camera = read_string(buf)?;
    me.current_hud_page = read_string(buf)?;
    me.is_replay_playing = read_u8(buf)? > 0;
    if me.is_replay_playing {
        me.replay_session_time = read_f32(buf)?;
        me.replay_remaining_time = read_f32(buf)?;
    }
    me.time_of_day = read_f32(buf)?;
    me.ambient_temp = read_u8(buf)?;
    me.track_temp = read_u8(buf)?;
    me.cloud_level = read_u8(buf)?;
    me.rain_level = read_u8(buf)?;
    me.wetness = read_u8(buf)?;
    me.best_session_lap = read_lap_info(buf)?;
    Ok(Response::RealtimeUpdate(me))
}

#[derive(Debug, Default)]
pub struct LapInfo {
    pub laptime_ms: i32,
    pub car_index: i16,
    pub driver_index: i16,
    pub splits: Vec<i32>,
    pub is_invaliud: bool,
    pub is_valid_for_best: bool,
    pub is_outlap: bool,
    pub is_inlap: bool,
}

fn read_lap_info(buf: &mut &[u8]) -> Result<LapInfo, IncompleteTypeError> {
    Ok(LapInfo {
        laptime_ms: read_i32(buf)?,
        car_index: read_i16(buf)?,
        driver_index: read_i16(buf)?,
        splits: {
            let mut splits = Vec::new();
            for _ in 0..read_u8(buf)? {
                splits.push(read_i32(buf)?);
            }
            splits
        },
        is_invaliud: read_u8(buf)? > 0,
        is_valid_for_best: read_u8(buf)? > 0,
        is_outlap: read_u8(buf)? > 0,
        is_inlap: read_u8(buf)? > 0,
    })
}

#[derive(Debug)]
pub struct RealtimeCarUpdate {
    pub car_index: i16,
    pub driver_index: i16,
    pub driver_cound: u8,
    pub gear: u8,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub car_location: u8,
    pub kmh: i16,
    pub position: i16,
    pub cup_position: i16,
    pub track_position: i16,
    pub spline_position: f32,
    pub laps: i16,
    pub delta: i32,
    pub best_session_lap: LapInfo,
    pub last_lap: LapInfo,
    pub current_lap: LapInfo,
}

fn read_realtime_car_update(buf: &mut &[u8]) -> Result<Response, IncompleteTypeError> {
    Ok(Response::RealtimeCarUpdate(RealtimeCarUpdate {
        car_index: read_i16(buf)?,
        driver_index: read_i16(buf)?,
        driver_cound: read_u8(buf)?,
        gear: read_u8(buf)?,
        yaw: read_f32(buf)?,
        pitch: read_f32(buf)?,
        roll: read_f32(buf)?,
        car_location: read_u8(buf)?,
        kmh: read_i16(buf)?,
        position: read_i16(buf)?,
        cup_position: read_i16(buf)?,
        track_position: read_i16(buf)?,
        spline_position: read_f32(buf)?,
        laps: read_i16(buf)?,
        delta: read_i32(buf)?,
        best_session_lap: read_lap_info(buf)?,
        last_lap: read_lap_info(buf)?,
        current_lap: read_lap_info(buf)?,
    }))
}

#[derive(Debug, Default)]
pub struct EntryList {
    pub connection_id: i32,
    pub car_entries: Vec<i16>,
}

fn read_entry_list(buf: &mut &[u8]) -> Result<Response, IncompleteTypeError> {
    Ok(Response::EntryList(EntryList {
        connection_id: read_i32(buf)?,
        car_entries: {
            let mut entries = Vec::new();
            for _ in 0..read_i16(buf)? {
                entries.push(read_i16(buf)?);
            }
            entries
        },
    }))
}

#[derive(Debug, Default)]
pub struct TrackData {
    pub connection_id: i32,
    pub track_name: String,
    pub track_id: i32,
    pub track_meter: i32,
    pub camera_sets: HashMap<String, Vec<String>>,
    pub hud_pages: Vec<String>,
}

fn read_track_data(buf: &mut &[u8]) -> Result<Response, IncompleteTypeError> {
    Ok(Response::TrackData(TrackData {
        connection_id: read_i32(buf)?,
        track_name: read_string(buf)?,
        track_id: read_i32(buf)?,
        track_meter: read_i32(buf)?,
        camera_sets: {
            let mut camera_sets = HashMap::new();
            for _ in 0..read_u8(buf)? {
                let set_name = read_string(buf)?;
                let mut cameras = Vec::new();
                for _ in 0..read_u8(buf)? {
                    cameras.push(read_string(buf)?);
                }
                camera_sets.insert(set_name, cameras);
            }
            camera_sets
        },
        hud_pages: {
            let mut hud_pages = Vec::new();
            for _ in 0..read_u8(buf)? {
                hud_pages.push(read_string(buf)?);
            }
            hud_pages
        },
    }))
}

#[derive(Debug, Default)]
pub struct EntryListCar {
    pub car_index: i16,
    pub car_model_type: u8,
    pub team_name: String,
    pub race_number: i32,
    pub cup_category: u8,
    pub current_driver_index: u8,
    pub car_nationality: i16,
    pub drivers: Vec<DriverInfo>,
}

fn read_entry_list_car(buf: &mut &[u8]) -> Result<Response, IncompleteTypeError> {
    Ok(Response::EntryListCar(EntryListCar {
        car_index: read_i16(buf)?,
        car_model_type: read_u8(buf)?,
        team_name: read_string(buf)?,
        race_number: read_i32(buf)?,
        cup_category: read_u8(buf)?,
        current_driver_index: read_u8(buf)?,
        car_nationality: read_i16(buf)?,
        drivers: {
            let mut drivers = Vec::new();
            for _ in 0..read_u8(buf)? {
                drivers.push(read_driver_info(buf)?);
            }
            drivers
        },
    }))
}

#[derive(Debug)]
pub struct DriverInfo {
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
    pub category: u8,
    pub nationality: i16,
}

fn read_driver_info(buf: &mut &[u8]) -> Result<DriverInfo, IncompleteTypeError> {
    Ok(DriverInfo {
        first_name: read_string(buf)?,
        last_name: read_string(buf)?,
        short_name: read_string(buf)?,
        category: read_u8(buf)?,
        nationality: read_i16(buf)?,
    })
}

#[derive(Debug, Default)]
pub struct BroadcastingEvent {
    pub message: String,
    pub time: i32,
    pub car_index: i32,
}

fn read_broadcasting_event(buf: &mut &[u8]) -> Result<Response, IncompleteTypeError> {
    Ok(Response::BroadcastingEvent(BroadcastingEvent {
        message: read_string(buf)?,
        time: read_i32(buf)?,
        car_index: read_i32(buf)?,
    }))
}

fn read_u8(buf: &mut &[u8]) -> Result<u8, IncompleteTypeError> {
    if buf.is_empty() {
        return Err(IncompleteTypeError {});
    }
    let (value, rest) = buf.split_at(1);
    *buf = rest;
    Ok(value[0])
}

fn read_i16(buf: &mut &[u8]) -> Result<i16, IncompleteTypeError> {
    if buf.len() < 4 {
        return Err(IncompleteTypeError {});
    }
    let (value, rest) = buf.split_at(2);
    *buf = rest;
    Ok(i16::from_le_bytes(value.try_into().unwrap()))
}

fn read_i32(buf: &mut &[u8]) -> Result<i32, IncompleteTypeError> {
    if buf.len() < 4 {
        return Err(IncompleteTypeError {});
    }
    let (value, rest) = buf.split_at(4);
    *buf = rest;
    Ok(i32::from_le_bytes(value.try_into().unwrap()))
}

fn read_string(buf: &mut &[u8]) -> Result<String, IncompleteTypeError> {
    let length = read_i16(buf)? as usize;
    if buf.len() < length {
        return Err(IncompleteTypeError {});
    }
    let (value, rest) = buf.split_at(length);
    *buf = rest;
    String::from_utf8(value.to_vec()).map_err(|_| IncompleteTypeError {})
}

fn read_f32(buf: &mut &[u8]) -> Result<f32, IncompleteTypeError> {
    if buf.len() < 4 {
        return Err(IncompleteTypeError {});
    }
    let (value, rest) = buf.split_at(4);
    *buf = rest;
    Ok(f32::from_le_bytes(value.try_into().unwrap()))
}

pub fn register_request(password: &str, update_interval: i32, command_password: &str) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    buf.push(1);
    buf.push(4);
    push_string(&mut buf, "");
    push_string(&mut buf, password);
    buf.extend(update_interval.to_le_bytes());
    push_string(&mut buf, command_password);
    buf
}

#[allow(dead_code)]
pub fn unregister_request(connection_id: i32) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    buf.push(9);
    buf.extend(connection_id.to_le_bytes());
    buf
}

#[allow(dead_code)]
pub fn entry_list_request(connection_id: i32) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    buf.push(10);
    buf.extend(connection_id.to_le_bytes());
    buf
}

#[allow(dead_code)]
pub fn track_data_request(connection_id: i32) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    buf.push(11);
    buf.extend(connection_id.to_le_bytes());
    buf
}

#[allow(dead_code)]
pub fn hud_page_request(connection_id: i32, page: String) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    buf.push(49);
    buf.extend(connection_id.to_le_bytes());
    push_string(&mut buf, &page);
    buf
}

#[allow(dead_code)]
pub fn focus_request(
    connection_id: i32,
    car_index: Option<i16>,
    camera: Option<(String, String)>,
) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    buf.push(50);
    buf.extend(connection_id.to_le_bytes());
    if let Some(car_index) = car_index {
        buf.push(1);
        buf.extend(car_index.to_le_bytes());
    } else {
        buf.push(0);
    }
    if let Some((set, camera)) = camera {
        buf.push(1);
        push_string(&mut buf, &set);
        push_string(&mut buf, &camera);
    } else {
        buf.push(0);
    }
    buf
}

#[allow(dead_code)]
pub fn instant_replay_request(
    connection_id: i32,
    session_start_time: f32,
    duration: f32,
    initial_focused_car: i32,
    initial_camera_set: String,
    initial_camera: String,
) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    buf.push(51);
    buf.extend(connection_id.to_le_bytes());
    buf.extend(session_start_time.to_le_bytes());
    buf.extend(duration.to_le_bytes());
    buf.extend(initial_focused_car.to_le_bytes());
    push_string(&mut buf, &initial_camera_set);
    push_string(&mut buf, &initial_camera);
    buf
}

fn push_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.bytes();
    buf.extend((bytes.len() as u16).to_le_bytes());
    buf.extend(bytes);
}
