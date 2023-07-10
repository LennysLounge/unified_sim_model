use bitflags::bitflags;

pub const MAX_BUFFERS: usize = 4;
pub const _SUPPORTED_VERSION: i32 = 2;
pub const MAX_STRING: usize = 32;
pub const MAX_DESC: usize = 64;

/// Messages that can be send to the game.
#[repr(i32)]
pub enum Messages {
    /// Switch the camera to a position.
    CamSwitchPos {
        // If position is zero then only switches the camera.
        position: u16,
        camera_group: u16,
        camera: u16,
    },
    /// Switch the camera to a driver number.
    CamSwitchNum {
        driver_num: u16,
        camera_group: u16,
        camera: u16,
    },
    /// Set the camera state.
    CamSetState {
        state: CameraState,
    },
    /// Set replay playback speed
    ReplaySetPlaySpeed {
        speed: u16,
        slow_motion: bool,
    },
    /// Set the replay position.
    ReplaySetPlayPosition {
        mode: ReplayPosMode,
        frame_number: u32,
    },
    /// Set replay search mode.
    ReplaySearch {
        mode: ReplaySearchMode,
    },
    /// Set the replay state.
    ReplaySetState {
        state: ReplayStateMode,
    },
    ReloadTextures {
        mode: ReloadTexturesMode,
        car_idx: u16,
    },
    /// Execute a chat command.
    ChatComand {
        mode: ChatCommandMode,
        macro_num: u16,
    },
    PitCommand {
        command: PitCommandMode,
        parameter: u32,
    },
    TelemCommand {
        mode: TelemetryCommandMode,
    },
    FFBCommand {
        mode: FFBCommandMode,
        force: f32,
    },
    ReplaySearchSessionTime {
        session_num: u16,
        session_time_ms: u32,
    },
    VideoCapture {
        mode: VideoCaptureMode,
    },
}

impl Messages {
    pub fn id(&self) -> u16 {
        match self {
            Messages::CamSwitchPos { .. } => 0,
            Messages::CamSwitchNum { .. } => 1,
            Messages::CamSetState { .. } => 2,
            Messages::ReplaySetPlaySpeed { .. } => 3,
            Messages::ReplaySetPlayPosition { .. } => 4,
            Messages::ReplaySearch { .. } => 5,
            Messages::ReplaySetState { .. } => 6,
            Messages::ReloadTextures { .. } => 7,
            Messages::ChatComand { .. } => 8,
            Messages::PitCommand { .. } => 9,
            Messages::TelemCommand { .. } => 10,
            Messages::FFBCommand { .. } => 11,
            Messages::ReplaySearchSessionTime { .. } => 12,
            Messages::VideoCapture { .. } => 13,
        }
    }

    pub fn map_to_paramters(&self) -> (u32, u32) {
        fn make_u32(p1: u16, p2: u16) -> u32 {
            p1 as u32 + ((p2 as u32) << 16)
        }
        fn cast_float(f: f32) -> u32 {
            // multiply by 2^16-1 to move fractional part to the integer part
            (f * 65536.0f32) as u32
        }
        let (p1, p2): (u16, u32) = match self {
            Messages::CamSwitchPos {
                position,
                camera_group,
                camera,
            } => (*position, make_u32(*camera_group, *camera)),
            Messages::CamSwitchNum {
                driver_num,
                camera_group,
                camera,
            } => (*driver_num, make_u32(*camera_group, *camera)),
            Messages::CamSetState {
                state: camera_state,
            } => (camera_state.bits() as u16, 0),
            Messages::ReplaySetPlaySpeed { speed, slow_motion } => {
                (*speed, make_u32(*slow_motion as u16, 0))
            }
            Messages::ReplaySetPlayPosition {
                mode: pos_mode,
                frame_number,
            } => (*pos_mode as u16, *frame_number),
            Messages::ReplaySearch { mode: search_mode } => (*search_mode as u16, 0u32),
            Messages::ReplaySetState {
                state: replay_state,
            } => (*replay_state as u16, 0),
            Messages::ReloadTextures { mode, car_idx } => (*mode as u16, make_u32(*car_idx, 0)),
            Messages::ChatComand { mode, macro_num } => (*mode as u16, make_u32(*macro_num, 0)),
            Messages::PitCommand { command, parameter } => (*command as u16, *parameter),
            Messages::TelemCommand { mode } => (*mode as u16, 0),
            Messages::FFBCommand { mode, force } => (*mode as u16, cast_float(*force)),
            Messages::ReplaySearchSessionTime {
                session_num,
                session_time_ms,
            } => (*session_num, *session_time_ms),
            Messages::VideoCapture { mode } => (*mode as u16, 0),
        };

        (make_u32(self.id(), p1), p2)
    }
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum VideoCaptureMode {
    TriggerScreenShot = 0, // save a screenshot to disk
    StartVideoCapture,     // start capturing video
    EndVideoCapture,       // stop capturing video
    ToggleVideoCapture,    // toggle video capture on/off
    ShowVideoTimer,        // show video timer in upper left corner of display
    HideVideoTimer,        // hide video timer
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum FFBCommandMode
// You can call this any time
{
    MaxForce = 0, // Set the maximum force when mapping steering torque force to direct input units (float in Nm)
    Last,         // unused placeholder
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum TelemetryCommandMode
// You can call this any time, but telemtry only records when driver is in there car
{
    /// Turn telemetry recording off
    Stop = 0,
    /// Turn telemetry recording on
    Start,
    /// Write current file to disk and start a new one
    Restart,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ReloadTexturesMode {
    /// reload all textuers
    All = 0,
    /// reload only textures for the specific carIdx
    CarIdx,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum PitCommandMode
// this only works when the driver is in the car
{
    /// Clear all pit checkboxes
    PitCommandClear = 0,
    /// Clean the winshield, using one tear off
    PitCommandWS,
    /// Add fuel, optionally specify the amount to add in liters or pass '0' to use existing amount     
    PitCommandFuel,
    /// Change the left front tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure
    PitCommandLF,
    /// Change the right front tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure
    PitCommandRF,
    /// Change the left rear tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure
    PitCommandLR,
    /// Change the right rear tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure
    PitCommandRR,
    /// Clear tire pit checkboxes
    PitCommandClearTires,
    /// Request a fast repair
    PitCommandFR,
    /// Uncheck Clean the winshield checkbox
    PitCommandClearWS,
    /// Uncheck request a fast repair
    PitCommandClearFR,
    /// Uncheck add fuel
    PitCommandClearFuel,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ChatCommandMode {
    ChatCommandMacro = 0, // pass in a number from 1-15 representing the chat macro to launch
    ChatCommandBeginChat, // Open up a new chat window
    ChatCommandReply,     // Reply to last private chat
    ChatCommandCancel,    // Close chat window
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ReplayStateMode {
    ReplayStateEraseTape = 0, // clear any data in the replay tape
    ReplayStateLast,          // unused place holder
}

// Search replay tape for events
#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ReplaySearchMode {
    ReplaySearchToStart = 0,
    ReplaySearchToEnd,
    ReplaySearchPrevSession,
    ReplaySearchNextSession,
    ReplaySearchPrevLap,
    ReplaySearchNextLap,
    ReplaySearchPrevFrame,
    ReplaySearchNextFrame,
    ReplaySearchPrevIncident,
    ReplaySearchNextIncident,
    ReplaySearchLast, // unused placeholder
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ReplayPosMode {
    ReplayPosBegin = 0,
    ReplayPosCurrent,
    ReplayPosEnd,
    ReplayPosLast, // unused placeholder
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

#[derive(Clone, Debug)]
#[repr(i32)]
pub enum SessionState {
    StateInvalid,
    StateGetInCar,
    StateWarmup,
    StateParadeLaps,
    StateRacing,
    StateCheckered,
    StateCoolDown,
}

impl From<i32> for SessionState {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::StateGetInCar,
            2 => Self::StateWarmup,
            3 => Self::StateParadeLaps,
            4 => Self::StateRacing,
            5 => Self::StateCheckered,
            6 => Self::StateCoolDown,
            _ => Self::StateInvalid,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct Flags: u32 {
        // global flags
        const irsdk_checkered        = 0x00000001;
        const irsdk_white            = 0x00000002;
        const irsdk_green            = 0x00000004;
        const irsdk_yellow           = 0x00000008;
        const irsdk_red              = 0x00000010;
        const irsdk_blue             = 0x00000020;
        const irsdk_debris           = 0x00000040;
        const irsdk_crossed          = 0x00000080;
        const irsdk_yellowWaving     = 0x00000100;
        const irsdk_oneLapToGreen    = 0x00000200;
        const irsdk_greenHeld        = 0x00000400;
        const irsdk_tenToGo          = 0x00000800;
        const irsdk_fiveToGo         = 0x00001000;
        const irsdk_randomWaving     = 0x00002000;
        const irsdk_caution          = 0x00004000;
        const irsdk_cautionWaving    = 0x00008000;

        // drivers black flags
        const irsdk_black			 = 0x00010000;
        const irsdk_disqualify		 = 0x00020000;
        const irsdk_servicible		 = 0x00040000; // car is allowed service (not a flag;
        const irsdk_furled			 = 0x00080000;
        const irsdk_repair			 = 0x00100000;

        // start lights
        const irsdk_startHidden		 = 0x10000000;
        const irsdk_startReady		 = 0x20000000;
        const irsdk_startSet		 = 0x40000000;
        const irsdk_startGo			 = 0x80000000;
    }
}

#[derive(Clone)]
#[repr(i32)]
pub enum TrkLoc {
    NotInWorld,
    OffTrack,
    InPitStall,
    AproachingPits,
    OnTrack,
}

impl From<i32> for TrkLoc {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::OffTrack,
            1 => Self::InPitStall,
            2 => Self::AproachingPits,
            3 => Self::OnTrack,
            _ => Self::NotInWorld,
        }
    }
}

#[derive(Clone)]
#[repr(i32)]
pub enum TrkSurf {
    SurfaceNotInWorld,
    UndefinedMaterial,
    Asphalt1Material,
    Asphalt2Material,
    Asphalt3Material,
    Asphalt4Material,
    Concrete1Material,
    Concrete2Material,
    RacingDirt1Material,
    RacingDirt2Material,
    Paint1Material,
    Paint2Material,
    Rumble1Material,
    Rumble2Material,
    Rumble3Material,
    Rumble4Material,
    Grass1Material,
    Grass2Material,
    Grass3Material,
    Grass4Material,
    Dirt1Material,
    Dirt2Material,
    Dirt3Material,
    Dirt4Material,
    SandMaterial,
    Gravel1Material,
    Gravel2Material,
    GrasscreteMaterial,
    AstroturfMaterial,
}

impl From<i32> for TrkSurf {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::UndefinedMaterial,
            1 => Self::Asphalt1Material,
            2 => Self::Asphalt2Material,
            3 => Self::Asphalt3Material,
            4 => Self::Asphalt4Material,
            5 => Self::Concrete1Material,
            6 => Self::Concrete2Material,
            7 => Self::RacingDirt1Material,
            8 => Self::RacingDirt2Material,
            9 => Self::Paint1Material,
            10 => Self::Paint2Material,
            11 => Self::Rumble1Material,
            12 => Self::Rumble2Material,
            13 => Self::Rumble3Material,
            14 => Self::Rumble4Material,
            15 => Self::Grass1Material,
            16 => Self::Grass2Material,
            17 => Self::Grass3Material,
            18 => Self::Grass4Material,
            19 => Self::Dirt1Material,
            20 => Self::Dirt2Material,
            21 => Self::Dirt3Material,
            22 => Self::Dirt4Material,
            23 => Self::SandMaterial,
            24 => Self::Gravel1Material,
            25 => Self::Gravel2Material,
            26 => Self::GrasscreteMaterial,
            27 => Self::AstroturfMaterial,
            _ => Self::SurfaceNotInWorld,
        }
    }
}

#[derive(Clone)]
#[repr(i32)]
pub enum PitSvStatus {
    PitSvNone,
    PitSvInProgress,
    PitSvComplete,
    PitSvTooFarLeft,
    PitSvTooFarRight,
    PitSvTooFarForward,
    PitSvTooFarBack,
    PitSvBadAngle,
    PitSvCantFixThat,
}

impl From<i32> for PitSvStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::PitSvNone,
            1 => Self::PitSvInProgress,
            2 => Self::PitSvComplete,
            100 => Self::PitSvTooFarLeft,
            101 => Self::PitSvTooFarRight,
            102 => Self::PitSvTooFarForward,
            103 => Self::PitSvTooFarBack,
            104 => Self::PitSvBadAngle,
            105 => Self::PitSvCantFixThat,
            _ => Self::PitSvNone,
        }
    }
}

#[derive(Clone)]
#[repr(i32)]
pub enum PaceMode {
    PaceModeSingleFileStart,
    PaceModeDoubleFileStart,
    PaceModeSingleFileRestart,
    PaceModeDoubleFileRestart,
    PaceModeNotPacing,
}

impl From<i32> for PaceMode {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::PaceModeSingleFileStart,
            1 => Self::PaceModeDoubleFileStart,
            2 => Self::PaceModeSingleFileRestart,
            3 => Self::PaceModeDoubleFileRestart,
            _ => Self::PaceModeNotPacing,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PaceFlags: u32 {
        const PaceFlagsEndOfLine = 0x01;
        const PaceFlagsFreePass = 0x02;
        const PaceFlagsWavedAround = 0x04;
    }
}

#[derive(Clone)]
#[repr(i32)]
pub enum CarLeftRight {
    LROff,
    LRClear,
    LRCarLeft,
    LRCarRight,
    LRCarLeftRight,
    LR2CarsLeft,
    LR2CarsRight,
}
impl From<i32> for CarLeftRight {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::LROff,
            1 => Self::LRClear,
            2 => Self::LRCarLeft,
            3 => Self::LRCarRight,
            4 => Self::LRCarLeftRight,
            5 => Self::LR2CarsLeft,
            6 => Self::LR2CarsRight,
            _ => Self::LROff,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct CameraState: u32 {
        const IsSessionScreen          = 0x0001;
        const IsScenicActive           = 0x0002;
        const CamToolActive            = 0x0004;
        const UIHidden                 = 0x0008;
        const UseAutoShotSelection     = 0x0010;
        const UseTemporaryEdits        = 0x0020;
        const UseKeyAcceleration       = 0x0040;
        const UseKey10xAcceleration    = 0x0080;
        const UseMouseAimMode          = 0x0100;
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct EngineWarnings: u32{
        const waterTempWarning		= 0x01;
        const fuelPressureWarning	= 0x02;
        const oilPressureWarning	= 0x04;
        const engineStalled			= 0x08;
        const pitSpeedLimiter		= 0x10;
        const revLimiterActive		= 0x20;
        const oilTempWarning		= 0x40;
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PitSvFlags: u32{
        const LFTireChange		= 0x0001;
        const RFTireChange		= 0x0002;
        const LRTireChange		= 0x0004;
        const RRTireChange		= 0x0008;
        const FuelFill			= 0x0010;
        const WindshieldTearoff	= 0x0020;
        const FastRepair		= 0x0040;
    }
}
