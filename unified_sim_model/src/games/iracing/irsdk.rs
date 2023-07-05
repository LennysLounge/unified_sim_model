use bitflags::bitflags;
use core::slice;
use std::fmt::Debug;
use thiserror::Error;
use tracing::{info, warn};
use windows::{
    w,
    Win32::{
        Foundation::HANDLE,
        System::Memory::{MapViewOfFile, OpenFileMappingW, FILE_MAP_READ},
    },
};

use self::data::{Data, SessionData};

pub mod data;

pub const MAX_BUFFERS: usize = 4;
pub const _SUPPORTED_VERSION: i32 = 2;
pub const MAX_STRING: usize = 32;
pub const MAX_DESC: usize = 64;

#[derive(Debug, Error)]
pub enum PollError {
    #[error("The game is not connected")]
    NotConnected,
}

#[derive(Debug)]
pub struct Irsdk {
    /// Handle to the memory mapped file.
    _handle: HANDLE,
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
    session_data: SessionData,
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

        return Ok(Self {
            _handle: handle,
            view,
            _last_tick_count: 0,
            var_handlers: Vec::new(),
            connected: false,
            session_data_last_udpate: 0,
            session_data: SessionData::default(),
        });
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

        let mut data = Data::default();
        data.session_data = self.session_data.clone();

        // Read var buffer
        self.parse_var_buffer(header, &mut data);

        Ok(data)
    }

    fn parse_session_str(&mut self, header: &Header) {
        info!("Process session data");
        self.session_data_last_udpate = header.session_data_update;
        let session_str_buffer = unsafe {
            slice::from_raw_parts(
                self.view.offset(header.session_data_offset as isize),
                header.session_data_len as usize,
            )
            .to_vec()
        };
        let session_str = String::from_utf8_lossy(&session_str_buffer)
            .trim_matches('\0')
            .to_string();
        // TODO: This should probably create an error instead of using the default.
        let session_data = serde_yaml::from_str::<SessionData>(&session_str);
        if session_data.is_err() {
            warn!("Error parsing session data yaml. Using default instead");
        }
        self.session_data = session_data.unwrap_or_default();
    }

    fn parse_var_headers(&mut self, header: &Header) {
        info!("Parsing variable headers");
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

            let processor = match name.as_str() {
                "Gear" => Processor::i32(|d| &mut d.gear),
                "SessionTime" => Processor::f64(|data| &mut data.session_time),
                "CarIdxLap" => Processor::vec_i32(|data| &mut data.car_idx_lap),
                _ => {
                    info!("Unmapped variable \"{}\". {:?}", name, header);
                    Processor::None
                }
            };

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
            handler.process(&var_buffer, data);
        }
    }
}

/// The header of the shared memory.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Header {
    /// Api version.
    pub version: i32,
    /// Bitfield using risdk_StatusField
    pub status: StatusField,
    /// Ticks per second (60 or 360 etc)
    pub tick_rate: i32,

    // Session information updates periodicaly
    /// Increments when session data changes
    pub session_data_update: i32,
    /// Length in bytes of session data string
    pub session_data_len: i32,
    /// Session data, encoded in Yaml format
    pub session_data_offset: i32,

    // Variable headers, updated every tick
    /// Amount of elements in the var header buffer.
    pub var_header_element_count: i32,
    /// Offset for the var header arrasy.
    pub var_header_offset: i32,

    // Variable buffers, updated every tick
    /// Number of buffers
    pub var_buffer_count: i32,
    /// length in bytes for one buffer
    pub var_buffer_len: i32,
    // (16 byte align)
    pad: [i32; 2],
    /// Var buffers
    pub var_buffers: [VarBuffer; MAX_BUFFERS],
}

bitflags! {
    /// Shared memory status bifflags
    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct StatusField: i32 {
        const CONNECTED = 1;
    }
}

/// Information about a variable in the shared memroy.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VarHeader {
    /// Type of the variable
    pub var_type: VarType,
    /// Offset from the header
    pub offset: i32,
    /// Number of entries for this variable in case of an array.
    pub count: i32,

    pub count_as_time: bool,
    pad: [u8; 3],

    /// Name of the variable
    pub name: [u8; MAX_STRING],
    /// Description of the variable
    pub description: [u8; MAX_DESC],
    /// Unit of the variable
    pub unit: [u8; MAX_STRING],
}

/// Types of variables in the shared memory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[repr(i32)]
pub enum VarType {
    Char,
    Bool,
    Int,
    Bitfield,
    Float,
    Double,
}

/// A buffer that holds the variables in the shared memory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VarBuffer {
    // Tick count when this buffer was generated
    pub tick_count: i32,
    // Offset from the header
    pub offset: i32,
    // (16 byte align)
    pad: [i32; 2],
}

/// A handler to read a variable from the var buffer and write its data into the model.
#[derive(Debug)]
pub struct VarHandler {
    header: VarHeader,
    processor: Processor,
}

impl VarHandler {
    fn process(&self, buffer: &[u8], data: &mut Data) {
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
                let value = i32::from_le_bytes(raw.try_into().unwrap());
                *p(data) = value;
            }
            Processor::VecI32(p) => {
                let target = p(data);
                target.clear();
                for i in 0..count {
                    let bytes = &raw[i * size..i * size + size];
                    let value = i32::from_le_bytes(bytes.try_into().unwrap());
                    target.push(value);
                }
            }
            Processor::F64(p) => {
                let value = f64::from_le_bytes(raw.try_into().unwrap());
                *p(data) = value;
            }
            Processor::None => (),
        }
    }
}

/// Types of processors to process differnt types of variables.
pub enum Processor {
    I32(Box<dyn Fn(&mut Data) -> &mut i32>),
    VecI32(Box<dyn Fn(&mut Data) -> &mut Vec<i32>>),
    F64(Box<dyn Fn(&mut Data) -> &mut f64>),
    None,
}

impl Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Processor::I32(_) => write!(f, "i32"),
            Processor::F64(_) => write!(f, "f64"),
            Processor::VecI32(_) => write!(f, "Vec<i32>"),
            Processor::None => write!(f, "None"),
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
        }
    }
    fn i32(target: impl Fn(&mut Data) -> &mut i32 + 'static) -> Self {
        Processor::I32(Box::new(target))
    }
    fn f64(target: impl Fn(&mut Data) -> &mut f64 + 'static) -> Self {
        Processor::F64(Box::new(target))
    }
    fn vec_i32(target: impl Fn(&mut Data) -> &mut Vec<i32> + 'static) -> Self {
        Processor::VecI32(Box::new(target))
    }
}
