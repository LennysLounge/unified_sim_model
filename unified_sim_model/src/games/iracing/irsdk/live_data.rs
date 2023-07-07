use crate::Time;
use bitflags::bitflags;

#[derive(Default, Clone)]
pub struct LiveData {
    pub session_time: Option<Time>,
    pub session_tick: Option<i32>,
    pub session_num: Option<i32>,
    pub session_state: Option<SessionState>,
    pub session_unique_id: Option<i32>,
    pub session_flags: Option<Flags>,
    pub session_time_remain: Option<Time>,
    pub session_laps_remain: Option<i32>,
    pub session_laps_remain_ex: Option<i32>,
    pub session_time_total: Option<Time>,
    pub session_laps_total: Option<i32>,
    pub session_joker_laps_remain: Option<i32>,
    pub session_on_joker_lap: Option<u8>,
    pub session_time_of_day: Option<Time>,
    pub radio_transmit_car_idx: Option<i32>,
    pub radio_transmit_radio_idx: Option<i32>,
    pub radio_transmit_frequency_idx: Option<i32>,
    pub display_units: Option<i32>,
    pub driver_marker: Option<u8>,
    pub push_to_talk: Option<u8>,
    pub push_to_pass: Option<u8>,
    pub manual_boost: Option<u8>,
    pub manual_no_boost: Option<u8>,
    pub is_on_track: Option<u8>,
    pub is_replay_playing: Option<u8>,
    pub replay_frame_num: Option<i32>,
    pub replay_frame_num_end: Option<i32>,
    pub is_disk_logging_enabled: Option<u8>,
    pub is_disk_logging_active: Option<u8>,
    pub frame_rate: Option<f32>,
    pub cpu_usage_fg: Option<f32>,
    pub gpu_usage: Option<f32>,
    pub chan_avg_latency: Option<Time>,
    pub chan_latency: Option<Time>,
    pub chan_quality: Option<f32>,
    pub chan_partner_quality: Option<f32>,
    pub cpu_usage_bg: Option<f32>,
    pub chan_clock_skew: Option<Time>,
    pub mem_page_fault_sec: Option<f32>,
    pub mem_soft_page_fault_sec: Option<f32>,
    pub player_car_position: Option<i32>,
    pub player_car_class_position: Option<i32>,
    pub player_car_class: Option<i32>,
    pub player_track_surface: Option<TrkLoc>,
    pub player_track_surface_material: Option<TrkSurf>,
    pub player_car_idx: Option<i32>,
    pub player_car_team_incident_count: Option<i32>,
    pub player_car_my_incident_count: Option<i32>,
    pub player_car_driver_incident_count: Option<i32>,
    ///  kg
    pub player_car_weight_penalty: Option<f32>,
    pub player_car_power_adjust: Option<f32>,
    pub player_car_dry_tire_set_limit: Option<i32>,
    pub player_car_tow_time: Option<Time>,
    pub player_car_in_pit_stall: Option<u8>,
    pub player_car_pit_sv_status: Option<PitSvStatus>,
    pub player_tire_compound: Option<i32>,
    pub player_fast_repairs_used: Option<i32>,
    pub car_idx_lap: Option<Vec<i32>>,
    pub car_idx_lap_completed: Option<Vec<i32>>,
    pub car_idx_lap_dist_pct: Option<Vec<f32>>,
    pub car_idx_track_surface: Option<Vec<TrkLoc>>,
    pub car_idx_track_surface_material: Option<Vec<TrkSurf>>,
    pub car_idx_on_pit_road: Option<Vec<u8>>,
    pub car_idx_position: Option<Vec<i32>>,
    pub car_idx_class_position: Option<Vec<i32>>,
    pub car_idx_class: Option<Vec<i32>>,
    pub car_idx_f2_time: Option<Vec<Time>>,
    pub car_idx_est_time: Option<Vec<Time>>,
    pub car_idx_last_lap_time: Option<Vec<Time>>,
    pub car_idx_best_lap_time: Option<Vec<Time>>,
    pub car_idx_best_lap_num: Option<Vec<i32>>,
    pub car_idx_tire_compound: Option<Vec<i32>>,
    pub car_idx_qual_tire_compound: Option<Vec<i32>>,
    pub car_idx_qual_tire_compound_locked: Option<Vec<u8>>,
    pub car_idx_fast_repairs_used: Option<Vec<i32>>,
    pub car_idx_session_flags: Option<Vec<Flags>>,
    pub pace_mode: Option<PaceMode>,
    pub car_idx_pace_line: Option<Vec<i32>>,
    pub car_idx_pace_row: Option<Vec<i32>>,
    pub car_idx_pace_flags: Option<Vec<PaceFlags>>,
    pub on_pit_road: Option<u8>,
    ///  rad
    pub car_idx_steer: Option<Vec<f32>>,
    ///  revs/min
    pub car_idx_rpm: Option<Vec<f32>>,
    pub car_idx_gear: Option<Vec<i32>>,
    ///  rad
    pub steering_wheel_angle: Option<f32>,
    ///  %
    pub throttle: Option<f32>,
    ///  %
    pub brake: Option<f32>,
    ///  %
    pub clutch: Option<f32>,
    pub gear: Option<i32>,
    ///  revs/min
    pub rpm: Option<f32>,
    pub lap: Option<i32>,
    pub lap_completed: Option<i32>,
    ///  m
    pub lap_dist: Option<f32>,
    ///  %
    pub lap_dist_pct: Option<f32>,
    pub race_laps: Option<i32>,
    pub lap_best_lap: Option<i32>,
    pub lap_best_lap_time: Option<Time>,
    pub lap_last_lap_time: Option<Time>,
    pub lap_current_lap_time: Option<Time>,
    pub lap_las_n_lap_seq: Option<i32>,
    pub lap_last_n_lap_time: Option<Time>,
    pub lap_best_n_lap_lap: Option<i32>,
    pub lap_best_n_lap_time: Option<Time>,
    pub lap_delta_to_best_lap: Option<Time>,
    ///  s/s
    pub lap_delta_to_best_lap_dd: Option<f32>,
    pub lap_delta_to_best_lap_ok: Option<u8>,
    pub lap_delta_to_optimal_lap: Option<Time>,
    ///  s/s
    pub lap_delta_to_optimal_lap_dd: Option<f32>,
    pub lap_delta_to_optimal_lap_ok: Option<u8>,
    pub lap_delta_to_session_best_lap: Option<Time>,
    ///  s/s
    pub lap_delta_to_session_best_lap_dd: Option<f32>,
    pub lap_delta_to_session_best_lap_ok: Option<u8>,
    pub lap_delta_to_session_optimal_lap: Option<Time>,
    ///  s/s
    pub lap_delta_to_session_optimal_lap_dd: Option<f32>,
    pub lap_delta_to_session_optimal_lap_ok: Option<u8>,
    pub lap_delta_to_session_lastl_lap: Option<Time>,
    ///  s/s
    pub lap_delta_to_session_lastl_lap_dd: Option<f32>,
    pub lap_delta_to_session_lastl_lap_ok: Option<u8>,
    ///  m/s
    pub speed: Option<f32>,
    ///  rad
    pub yaw: Option<f32>,
    ///  rad
    pub yaw_north: Option<f32>,
    ///  rad
    pub pitch: Option<f32>,
    ///  rad
    pub roll: Option<f32>,
    pub enter_exit_reset: Option<i32>,
    ///  C
    pub track_temp: Option<f32>,
    ///  C
    pub track_temp_crew: Option<f32>,
    ///  C
    pub air_temp: Option<f32>,
    pub weather_type: Option<i32>,
    pub skies: Option<i32>,
    ///  kg/m^3
    pub air_density: Option<f32>,
    ///  Hg
    pub air_pressure: Option<f32>,
    ///  m/s
    pub wind_vel: Option<f32>,
    ///  rad
    pub wind_dir: Option<f32>,
    ///  %
    pub relative_humidity: Option<f32>,
    ///  %
    pub fog_level: Option<f32>,
    ///  rad
    pub solar_altitude: Option<f32>,
    ///  rad
    pub solar_azimuth: Option<f32>,
    pub dc_lap_status: Option<i32>,
    pub dc_drivers_so_far: Option<i32>,
    pub ok_to_reload_textures: Option<u8>,
    pub load_num_textures: Option<u8>,
    pub car_left_right: Option<CarLeftRight>,
    pub pits_open: Option<u8>,
    pub vid_cap_enabled: Option<u8>,
    pub vid_cap_active: Option<u8>,
    pub pit_repair_left: Option<Time>,
    pub pit_opt_repair_left: Option<Time>,
    pub pitstop_active: Option<u8>,
    pub fast_repair_used: Option<i32>,
    pub fast_repair_available: Option<i32>,
    pub lf_tires_used: Option<i32>,
    pub rf_tires_used: Option<i32>,
    pub lr_tires_used: Option<i32>,
    pub rr_tires_used: Option<i32>,
    pub left_tire_sets_used: Option<i32>,
    pub right_tire_sets_used: Option<i32>,
    pub front_tire_sets_used: Option<i32>,
    pub rear_tire_sets_used: Option<i32>,
    pub tire_sets_used: Option<i32>,
    pub lf_tires_available: Option<i32>,
    pub rf_tires_available: Option<i32>,
    pub lr_tires_available: Option<i32>,
    pub rr_tires_available: Option<i32>,
    pub left_tire_sets_available: Option<i32>,
    pub right_tire_sets_available: Option<i32>,
    pub front_tire_sets_available: Option<i32>,
    pub rear_tire_sets_available: Option<i32>,
    pub tire_sets_available: Option<i32>,
    pub cam_car_idx: Option<i32>,
    pub cam_camera_number: Option<i32>,
    pub cam_group_number: Option<i32>,
    pub cam_camera_state: Option<CameraState>,
    pub is_on_track_car: Option<u8>,
    pub is_in_garage: Option<u8>,
    ///  rad
    pub steering_wheel_angle_max: Option<f32>,
    pub shift_power_pct: Option<f32>,
    ///  RPM
    pub shift_grind_rpm: Option<f32>,
    pub throttle_raw: Option<f32>,
    pub brake_raw: Option<f32>,
    pub clutch_raw: Option<f32>,
    pub handbrake_raw: Option<f32>,
    pub brake_ab_sactive: Option<u8>,
    pub engine_warnings: Option<EngineWarnings>,
    pub fuel_level_pct: Option<f32>,
    pub pit_sv_flags: Option<PitSvFlags>,
    ///  kPa
    pub pit_sv_lfp: Option<f32>,
    ///  kPa
    pub pit_sv_rfp: Option<f32>,
    ///  kPa
    pub pit_sv_lrp: Option<f32>,
    ///  kPa
    pub pit_sv_rrp: Option<f32>,
    ///  l or kWh
    pub pit_sv_fuel: Option<f32>,
    pub pit_sv_tire_compound: Option<i32>,
    pub car_idx_p2p_status: Option<Vec<u8>>,
    pub car_idx_p2p_count: Option<Vec<i32>>,
    pub steering_wheel_pct_torque: Option<f32>,
    pub steering_wheel_pct_torque_sign: Option<f32>,
    pub steering_wheel_pct_torque_sign_stops: Option<f32>,
    pub steering_wheel_pct_smoothing: Option<f32>,
    pub steering_wheel_pct_damper: Option<f32>,
    pub steering_wheel_limiter: Option<f32>,
    ///  N*m
    pub steering_wheel_max_force_nm: Option<f32>,
    ///  N*m
    pub steering_wheel_peak_force_nm: Option<f32>,
    pub steering_wheel_use_linear: Option<u8>,
    pub shift_indicator_pct: Option<f32>,
    pub replay_play_speed: Option<i32>,
    pub replay_play_slow_motion: Option<u8>,
    pub replay_session_time: Option<Time>,
    pub replay_session_num: Option<i32>,
    ///  Hz
    pub tire_lf_rumble_pitch: Option<f32>,
    ///  Hz
    pub tire_rf_rumble_pitch: Option<f32>,
    ///  Hz
    pub tire_lr_rumble_pitch: Option<f32>,
    ///  Hz
    pub tire_rr_rumble_pitch: Option<f32>,
    pub is_garage_visible: Option<u8>,
    ///  N*m
    pub steering_wheel_torque_st: Option<f32>,
    ///  N*m
    pub steering_wheel_torque: Option<f32>,
    ///  m/s at 360 Hz
    pub velocity_z_st: Option<f32>,
    ///  m/s at 360 Hz
    pub velocity_y_st: Option<f32>,
    ///  m/s at 360 Hz
    pub velocity_x_st: Option<f32>,
    ///  m/s
    pub velocity_z: Option<f32>,
    ///  m/s
    pub velocity_y: Option<f32>,
    ///  m/s
    pub velocity_x: Option<f32>,
    ///  rad/s
    pub yaw_rate_st: Option<f32>,
    ///  rad/s
    pub pitch_rate_st: Option<f32>,
    ///  rad/s
    pub roll_rate_st: Option<f32>,
    ///  rad/s
    pub yaw_rate: Option<f32>,
    ///  rad/s
    pub pitch_rate: Option<f32>,
    ///  rad/s
    pub roll_rate: Option<f32>,
    ///  m/s^2
    pub vert_accel_st: Option<f32>,
    ///  m/s^2
    pub lat_accel_st: Option<f32>,
    ///  m/s^2
    pub long_accel_st: Option<f32>,
    ///  m/s^2
    pub vert_accel: Option<f32>,
    ///  m/s^2
    pub lat_accel: Option<f32>,
    ///  m/s^2
    pub long_accel: Option<f32>,
    pub dc_starter: Option<u8>,
    pub dc_dash_page: Option<f32>,
    pub dc_tear_off_visor: Option<u8>,
    pub dp_tire_change: Option<f32>,
    pub dp_fuel_fill: Option<f32>,
    ///  kg
    pub dp_fuel_add_kg: Option<f32>,
    pub dp_fast_repair: Option<f32>,
    pub dc_brake_bias: Option<f32>,
    ///  Pa
    pub dp_lf_tire_cold_press: Option<f32>,
    ///  Pa
    pub dp_rf_tire_cold_press: Option<f32>,
    ///  Pa
    pub dp_lr_tire_cold_press: Option<f32>,
    ///  Pa
    pub dp_rr_tire_cold_press: Option<f32>,
    ///  bar
    pub r_fbrake_line_press: Option<f32>,
    ///  kPa
    pub r_fcold_pressure: Option<f32>,
    ///  C
    pub r_ftemp_cl: Option<f32>,
    ///  C
    pub r_ftemp_cm: Option<f32>,
    ///  C
    pub r_ftemp_cr: Option<f32>,
    pub r_fwear_l: Option<f32>,
    pub r_fwear_m: Option<f32>,
    pub r_fwear_r: Option<f32>,
    ///  bar
    pub l_fbrake_line_press: Option<f32>,
    ///  kPa
    pub l_fcold_pressure: Option<f32>,
    ///  C
    pub l_ftemp_cl: Option<f32>,
    ///  C
    pub l_ftemp_cm: Option<f32>,
    ///  C
    pub l_ftemp_cr: Option<f32>,
    pub l_fwear_l: Option<f32>,
    pub l_fwear_m: Option<f32>,
    pub l_fwear_r: Option<f32>,
    ///  kg/h
    pub fuel_use_per_hour: Option<f32>,
    ///  V
    pub voltage: Option<f32>,
    ///  C
    pub water_temp: Option<f32>,
    ///  l
    pub water_level: Option<f32>,
    ///  bar
    pub fuel_press: Option<f32>,
    ///  C
    pub oil_temp: Option<f32>,
    ///  bar
    pub oil_press: Option<f32>,
    ///  l
    pub oil_level: Option<f32>,
    ///  bar
    pub manifold_press: Option<f32>,
    ///  l
    pub fuel_level: Option<f32>,
    ///  revs/min
    pub engine0_rpm: Option<f32>,
    ///  bar
    pub r_rbrake_line_press: Option<f32>,
    ///  kPa
    pub r_rcold_pressure: Option<f32>,
    ///  C
    pub r_rtemp_cl: Option<f32>,
    ///  C
    pub r_rtemp_cm: Option<f32>,
    ///  C
    pub r_rtemp_cr: Option<f32>,
    pub r_rwear_l: Option<f32>,
    pub r_rwear_m: Option<f32>,
    pub r_rwear_r: Option<f32>,
    ///  bar
    pub l_rbrake_line_press: Option<f32>,
    ///  kPa
    pub l_rcold_pressure: Option<f32>,
    ///  C
    pub l_rtemp_cl: Option<f32>,
    ///  C
    pub l_rtemp_cm: Option<f32>,
    ///  C
    pub l_rtemp_cr: Option<f32>,
    pub l_rwear_l: Option<f32>,
    pub l_rwear_m: Option<f32>,
    pub l_rwear_r: Option<f32>,
    ///  m
    pub c_rshock_defl: Option<f32>,
    ///  m
    pub c_rshock_defl_st: Option<f32>,
    ///  m/s
    pub c_rshock_vel: Option<f32>,
    ///  m/s
    pub c_rshock_vel_st: Option<f32>,
    ///  m
    pub l_rshock_defl: Option<f32>,
    ///  m
    pub l_rshock_defl_st: Option<f32>,
    ///  m/s
    pub l_rshock_vel: Option<f32>,
    ///  m/s
    pub l_rshock_vel_st: Option<f32>,
    ///  m
    pub r_rshock_defl: Option<f32>,
    ///  m
    pub r_rshock_defl_st: Option<f32>,
    ///  m/s
    pub r_rshock_vel: Option<f32>,
    ///  m/s
    pub r_rshock_vel_st: Option<f32>,
    ///  m
    pub l_fshock_defl: Option<f32>,
    ///  m
    pub l_fshock_defl_st: Option<f32>,
    ///  m/s
    pub l_fshock_vel: Option<f32>,
    ///  m/s
    pub l_fshock_vel_st: Option<f32>,
    ///  m
    pub r_fshock_defl: Option<f32>,
    ///  m
    pub r_fshock_defl_st: Option<f32>,
    ///  m/s
    pub r_fshock_vel: Option<f32>,
    ///  m/s
    pub r_fshock_vel_st: Option<f32>,
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
