use core::slice;
use std::{error::Error, fmt::Display};

use windows::{
    w,
    Win32::{
        Foundation::{GetLastError, HANDLE, WIN32_ERROR},
        System::Memory::{MapViewOfFile, OpenFileMappingW, FILE_MAP_READ},
    },
};

pub mod data;

#[derive(Debug)]
pub struct Win32Error {
    pub last_error: WIN32_ERROR,
}

impl Display for Win32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "last error reported by GetLastError: {}",
            self.last_error.0
        )
    }
}

impl Error for Win32Error {}

impl Win32Error {
    fn get_last_error() -> Self {
        return Self {
            last_error: unsafe { GetLastError() },
        };
    }
}

pub const MAX_BUFFERS: usize = 4;
pub const _SUPPORTED_VERSION: i32 = 2;
pub const MAX_STRING: usize = 32;
pub const MAX_DESC: usize = 64;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Irsdk {
    handle: HANDLE,
    view: *const u8,
    /// Tick count of the last update.
    last_tick_count: i32,
}

impl Irsdk {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // SAFETY: Calling a foreign function is unsafe. Nothing else we can do here.
        let handle =
            unsafe { OpenFileMappingW(FILE_MAP_READ.0, false, w!("Local\\IRSDKMemMapFileName")) }?;

        // SAFETY: The returned pointer may be null to indicat that the operation has failed
        // and needs to be checked.
        let view = unsafe { MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0) as *const u8 };
        if view.is_null() {
            return Err(Win32Error::get_last_error().into());
        }

        return Ok(Self {
            handle,
            view,
            last_tick_count: 0,
        });
    }

    pub fn poll(&mut self) -> data::Data {
        // SAFETY: The pointer has been checked to be not null.
        // A Header struct is plain data and for all fields any bit pattern is a vlaid value.
        // Therefore dereferencing is fine.
        let header = unsafe { &*(self.view as *const Header) };

        let mut newest_buffer_index = 0;
        for i in 1..header.var_buffer_count {
            if header.var_buffers[i as usize].tick_count
                > header.var_buffers[newest_buffer_index].tick_count
            {
                newest_buffer_index = i as usize;
            }
        }
        let var_buffer_header = &header.var_buffers[newest_buffer_index];

        let current_tick_count = var_buffer_header.tick_count;
        let var_buffer = unsafe {
            slice::from_raw_parts(
                self.view.offset(var_buffer_header.offset as isize),
                header.var_buffer_len as usize,
            )
            .to_vec()
        };
        if var_buffer_header.tick_count != current_tick_count {
            eprintln!("WARNING: The variable buffer has changed while reading");
        }

        // TODO: Check if the var buffer is actually newer than the last buffer.
        self.last_tick_count = current_tick_count;

        let var_headers = unsafe {
            slice::from_raw_parts(
                (self.view as *const u8).offset(header.var_header_offset as isize)
                    as *const VarHeader,
                header.var_header_len as usize,
            )
            .to_vec()
        };

        let session_str_buffer = unsafe {
            slice::from_raw_parts(
                self.view.offset(header.session_info_offset as isize),
                header.session_info_len as usize,
            )
            .to_vec()
        };
        return data::Data::new(var_headers, var_buffer, session_str_buffer);
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Header {
    /// Api version.
    pub version: i32,
    /// Bitfield using risdk_StatusField
    pub status: i32,
    /// Ticks per second (60 or 360 etc)
    pub tick_rate: i32,

    // Session  information updates periodicaly
    /// Increments when session info changes
    pub session_info_update: i32,
    /// Length in bytes of session info string
    pub session_info_len: i32,
    /// Session info, encoded in Yaml format
    pub session_info_offset: i32,

    // Variable headers, updated every tick
    /// Length of the variable buffer
    pub var_header_len: i32,
    /// Offset for the irsdk_var_header[num_vars] array
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VarHeader {
    /// Type of the variable
    pub var_type: i32,
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
