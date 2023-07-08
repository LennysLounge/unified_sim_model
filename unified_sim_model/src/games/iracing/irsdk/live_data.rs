use crate::Time;
use bitflags::bitflags;

#[derive(Default, Clone)]
pub struct LiveData {
    /// Seconds since session start.
    /// unit: s
    pub session_time: Option<Time>,
    /// Current update number.
    pub session_tick: Option<i32>,
    /// Session number.
    pub session_num: Option<i32>,
    /// Session state.
    /// unit: irsdk_SessionState
    pub session_state: Option<SessionState>,
    /// Session ID.
    pub session_unique_id: Option<i32>,
    /// Session flags.
    /// unit: irsdk_Flags
    pub session_flags: Option<Flags>,
    /// Seconds left till session ends.
    /// unit: s
    pub session_time_remain: Option<Time>,
    /// Old laps left till session ends use SessionLapsRemainEx.
    pub session_laps_remain: Option<i32>,
    /// New improved laps left till session ends.
    pub session_laps_remain_ex: Option<i32>,
    /// Total number of seconds in session.
    /// unit: s
    pub session_time_total: Option<Time>,
    /// Total number of laps in session.
    pub session_laps_total: Option<i32>,
    /// Joker laps remaining to be taken.
    pub session_joker_laps_remain: Option<i32>,
    /// Player is currently completing a joker lap.
    pub session_on_joker_lap: Option<bool>,
    /// Time of day in seconds.
    /// unit: s
    pub session_time_of_day: Option<Time>,
    /// The car index of the current person speaking on the radio.
    pub radio_transmit_car_idx: Option<i32>,
    /// The radio index of the current person speaking on the radio.
    pub radio_transmit_radio_idx: Option<i32>,
    /// The frequency index of the current person speaking on the radio.
    pub radio_transmit_frequency_idx: Option<i32>,
    /// Default units for the user interface 0 = english 1 = metric.
    pub display_units: Option<i32>,
    /// Driver activated flag.
    pub driver_marker: Option<bool>,
    /// Push to talk button state.
    pub push_to_talk: Option<bool>,
    /// Push to pass button state.
    pub push_to_pass: Option<bool>,
    /// Hybrid manual boost state.
    pub manual_boost: Option<bool>,
    /// Hybrid manual no boost state.
    pub manual_no_boost: Option<bool>,
    /// 1=Car on track physics running with player in car.
    pub is_on_track: Option<bool>,
    /// 0=replay not playing  1=replay playing.
    pub is_replay_playing: Option<bool>,
    /// Integer replay frame number (60 per second).
    pub replay_frame_num: Option<i32>,
    /// Integer replay frame number from end of tape.
    pub replay_frame_num_end: Option<i32>,
    /// 0=disk based telemetry turned off  1=turned on.
    pub is_disk_logging_enabled: Option<bool>,
    /// 0=disk based telemetry file not being written  1=being written.
    pub is_disk_logging_active: Option<bool>,
    /// Average frames per second.
    /// unit: fps
    pub frame_rate: Option<f32>,
    /// Percent of available tim fg thread took with a 1 sec avg.
    /// unit: %
    pub cpu_usage_fg: Option<f32>,
    /// Percent of available tim gpu took with a 1 sec avg.
    /// unit: %
    pub gpu_usage: Option<f32>,
    /// Communications average latency.
    /// unit: s
    pub chan_avg_latency: Option<Time>,
    /// Communications latency.
    /// unit: s
    pub chan_latency: Option<Time>,
    /// Communications quality.
    /// unit: %
    pub chan_quality: Option<f32>,
    /// Partner communications quality.
    /// unit: %
    pub chan_partner_quality: Option<f32>,
    /// Percent of available tim bg thread took with a 1 sec avg.
    /// unit: %
    pub cpu_usage_bg: Option<f32>,
    /// Communications server clock skew.
    /// unit: s
    pub chan_clock_skew: Option<Time>,
    /// Memory page faults per second.
    pub mem_page_fault_sec: Option<f32>,
    /// Memory soft page faults per second.
    pub mem_soft_page_fault_sec: Option<f32>,
    /// Players position in race.
    pub player_car_position: Option<i32>,
    /// Players class position in race.
    pub player_car_class_position: Option<i32>,
    /// Player car class id.
    pub player_car_class: Option<i32>,
    /// Players car track surface type.
    /// unit: irsdk_TrkLoc
    pub player_track_surface: Option<i32>,
    /// Players car track surface material type.
    /// unit: irsdk_TrkSurf
    pub player_track_surface_material: Option<i32>,
    /// Players carIdx.
    pub player_car_idx: Option<i32>,
    /// Players team incident count for this session.
    pub player_car_team_incident_count: Option<i32>,
    /// Players own incident count for this session.
    pub player_car_my_incident_count: Option<i32>,
    /// Teams current drivers incident count for this session.
    pub player_car_driver_incident_count: Option<i32>,
    /// Players weight penalty.
    /// unit: kg
    pub player_car_weight_penalty: Option<f32>,
    /// Players power adjust.
    /// unit: %
    pub player_car_power_adjust: Option<f32>,
    /// Players dry tire set limit.
    pub player_car_dry_tire_set_limit: Option<i32>,
    /// Players car is being towed if time is greater than zero.
    /// unit: s
    pub player_car_tow_time: Option<Time>,
    /// Players car is properly in there pitstall.
    pub player_car_in_pit_stall: Option<bool>,
    /// Players car pit service status bits.
    /// unit: irsdk_PitSvStatus
    pub player_car_pit_sv_status: Option<i32>,
    /// Players car current tire compound.
    pub player_tire_compound: Option<i32>,
    /// Players car number of fast repairs used.
    pub player_fast_repairs_used: Option<i32>,
    /// Laps started by car index.
    pub car_idx_lap: Option<Vec<i32>>,
    /// Laps completed by car index.
    pub car_idx_lap_completed: Option<Vec<i32>>,
    /// Percentage distance around lap by car index.
    /// unit: %
    pub car_idx_lap_dist_pct: Option<Vec<f32>>,
    /// Track surface type by car index.
    pub car_idx_track_surface: Option<Vec<TrkLoc>>,
    /// Track surface material type by car index.
    pub car_idx_track_surface_material: Option<Vec<TrkSurf>>,
    /// On pit road between the cones by car index.
    pub car_idx_on_pit_road: Option<Vec<bool>>,
    /// Cars position in race by car index.
    pub car_idx_position: Option<Vec<i32>>,
    /// Cars class position in race by car index.
    pub car_idx_class_position: Option<Vec<i32>>,
    /// Cars class id by car index.
    pub car_idx_class: Option<Vec<i32>>,
    /// Race time behind leader or fastest lap time otherwise.
    /// unit: s
    pub car_idx_f2_time: Option<Vec<Time>>,
    /// Estimated time to reach current location on track.
    /// unit: s
    pub car_idx_est_time: Option<Vec<Time>>,
    /// Cars last lap time.
    /// unit: s
    pub car_idx_last_lap_time: Option<Vec<Time>>,
    /// Cars best lap time.
    /// unit: s
    pub car_idx_best_lap_time: Option<Vec<Time>>,
    /// Cars best lap number.
    pub car_idx_best_lap_num: Option<Vec<i32>>,
    /// Cars current tire compound.
    pub car_idx_tire_compound: Option<Vec<i32>>,
    /// Cars Qual tire compound.
    pub car_idx_qual_tire_compound: Option<Vec<i32>>,
    /// Cars Qual tire compound is locked-in.
    pub car_idx_qual_tire_compound_locked: Option<Vec<bool>>,
    /// How many fast repairs each car has used.
    pub car_idx_fast_repairs_used: Option<Vec<i32>>,
    /// Session flags for each player.
    /// unit: irsdk_Flags
    pub car_idx_session_flags: Option<Vec<Flags>>,
    /// Are we pacing or not.
    /// unit: irsdk_PaceMode
    pub pace_mode: Option<i32>,
    /// What line cars are pacing in  or -1 if not pacing.
    pub car_idx_pace_line: Option<Vec<i32>>,
    /// What row cars are pacing in  or -1 if not pacing.
    pub car_idx_pace_row: Option<Vec<i32>>,
    /// Pacing status flags for each car.
    /// unit: irsdk_PaceFlags
    pub car_idx_pace_flags: Option<Vec<PaceFlags>>,
    /// Is the player car on pit road between the cones.
    pub on_pit_road: Option<bool>,
    /// Steering wheel angle by car index.
    /// unit: rad
    pub car_idx_steer: Option<Vec<f32>>,
    /// Engine rpm by car index.
    /// unit: revs/min
    pub car_idx_rpm: Option<Vec<f32>>,
    /// -1=reverse  0=neutral  1..n=current gear by car index.
    pub car_idx_gear: Option<Vec<i32>>,
    /// Steering wheel angle.
    /// unit: rad
    pub steering_wheel_angle: Option<f32>,
    /// 0=off throttle to 1=full throttle.
    /// unit: %
    pub throttle: Option<f32>,
    /// 0=brake released to 1=max pedal force.
    /// unit: %
    pub brake: Option<f32>,
    /// 0=disengaged to 1=fully engaged.
    /// unit: %
    pub clutch: Option<f32>,
    /// -1=reverse  0=neutral  1..n=current gear.
    pub gear: Option<i32>,
    /// Engine rpm.
    /// unit: revs/min
    pub rpm: Option<f32>,
    /// Laps started count.
    pub lap: Option<i32>,
    /// Laps completed count.
    pub lap_completed: Option<i32>,
    /// Meters traveled from S/F this lap.
    /// unit: m
    pub lap_dist: Option<f32>,
    /// Percentage distance around lap.
    /// unit: %
    pub lap_dist_pct: Option<f32>,
    /// Laps completed in race.
    pub race_laps: Option<i32>,
    /// Players best lap number.
    pub lap_best_lap: Option<i32>,
    /// Players best lap time.
    /// unit: s
    pub lap_best_lap_time: Option<Time>,
    /// Players last lap time.
    /// unit: s
    pub lap_last_lap_time: Option<Time>,
    /// Estimate of players current lap time as shown in F3 box.
    /// unit: s
    pub lap_current_lap_time: Option<Time>,
    /// Player num consecutive clean laps completed for N average.
    pub lap_las_n_lap_seq: Option<i32>,
    /// Player last N average lap time.
    /// unit: s
    pub lap_last_n_lap_time: Option<Time>,
    /// Player last lap in best N average lap time.
    pub lap_best_n_lap_lap: Option<i32>,
    /// Player best N average lap time.
    /// unit: s
    pub lap_best_n_lap_time: Option<Time>,
    /// Delta time for best lap.
    /// unit: s
    pub lap_delta_to_best_lap: Option<Time>,
    /// Rate of change of delta time for best lap.
    /// unit: s/s
    pub lap_delta_to_best_lap_dd: Option<f32>,
    /// Delta time for best lap is valid.
    pub lap_delta_to_best_lap_ok: Option<bool>,
    /// Delta time for optimal lap.
    /// unit: s
    pub lap_delta_to_optimal_lap: Option<Time>,
    /// Rate of change of delta time for optimal lap.
    /// unit: s/s
    pub lap_delta_to_optimal_lap_dd: Option<f32>,
    /// Delta time for optimal lap is valid.
    pub lap_delta_to_optimal_lap_ok: Option<bool>,
    /// Delta time for session best lap.
    /// unit: s
    pub lap_delta_to_session_best_lap: Option<Time>,
    /// Rate of change of delta time for session best lap.
    /// unit: s/s
    pub lap_delta_to_session_best_lap_dd: Option<f32>,
    /// Delta time for session best lap is valid.
    pub lap_delta_to_session_best_lap_ok: Option<bool>,
    /// Delta time for session optimal lap.
    /// unit: s
    pub lap_delta_to_session_optimal_lap: Option<Time>,
    /// Rate of change of delta time for session optimal lap.
    /// unit: s/s
    pub lap_delta_to_session_optimal_lap_dd: Option<f32>,
    /// Delta time for session optimal lap is valid.
    pub lap_delta_to_session_optimal_lap_ok: Option<bool>,
    /// Delta time for session last lap.
    /// unit: s
    pub lap_delta_to_session_lastl_lap: Option<Time>,
    /// Rate of change of delta time for session last lap.
    /// unit: s/s
    pub lap_delta_to_session_lastl_lap_dd: Option<f32>,
    /// Delta time for session last lap is valid.
    pub lap_delta_to_session_lastl_lap_ok: Option<bool>,
    /// GPS vehicle speed.
    /// unit: m/s
    pub speed: Option<f32>,
    /// Yaw orientation.
    /// unit: rad
    pub yaw: Option<f32>,
    /// Yaw orientation relative to north.
    /// unit: rad
    pub yaw_north: Option<f32>,
    /// Pitch orientation.
    /// unit: rad
    pub pitch: Option<f32>,
    /// Roll orientation.
    /// unit: rad
    pub roll: Option<f32>,
    /// Indicate action the reset key will take 0 enter 1 exit 2 reset.
    pub enter_exit_reset: Option<i32>,
    /// Deprecated  set to TrackTempCrew.
    /// unit: C
    pub track_temp: Option<f32>,
    /// Temperature of track measured by crew around track.
    /// unit: C
    pub track_temp_crew: Option<f32>,
    /// Temperature of air at start/finish line.
    /// unit: C
    pub air_temp: Option<f32>,
    /// Weather type (0=constant  1=dynamic).
    pub weather_type: Option<i32>,
    /// Skies (0=clear/1=p cloudy/2=m cloudy/3=overcast).
    pub skies: Option<i32>,
    /// Density of air at start/finish line.
    /// unit: kg/m^3
    pub air_density: Option<f32>,
    /// Pressure of air at start/finish line.
    /// unit: Hg
    pub air_pressure: Option<f32>,
    /// Wind velocity at start/finish line.
    /// unit: m/s
    pub wind_vel: Option<f32>,
    /// Wind direction at start/finish line.
    /// unit: rad
    pub wind_dir: Option<f32>,
    /// Relative Humidity.
    /// unit: %
    pub relative_humidity: Option<f32>,
    /// Fog level.
    /// unit: %
    pub fog_level: Option<f32>,
    /// Sun angle above horizon in radians.
    /// unit: rad
    pub solar_altitude: Option<f32>,
    /// Sun angle clockwise from north in radians.
    /// unit: rad
    pub solar_azimuth: Option<f32>,
    /// Status of driver change lap requirements.
    pub dc_lap_status: Option<i32>,
    /// Number of team drivers who have run a stint.
    pub dc_drivers_so_far: Option<i32>,
    /// True if it is ok to reload car textures at this time.
    pub ok_to_reload_textures: Option<bool>,
    /// True if the car_num texture will be loaded.
    pub load_num_textures: Option<bool>,
    /// Notify if car is to the left or right of driver.
    /// unit: irsdk_CarLeftRight
    pub car_left_right: Option<i32>,
    /// True if pit stop is allowed for the current player.
    pub pits_open: Option<bool>,
    /// True if video capture system is enabled.
    pub vid_cap_enabled: Option<bool>,
    /// True if video currently being captured.
    pub vid_cap_active: Option<bool>,
    /// Time left for mandatory pit repairs if repairs are active.
    /// unit: s
    pub pit_repair_left: Option<Time>,
    /// Time left for optional repairs if repairs are active.
    /// unit: s
    pub pit_opt_repair_left: Option<Time>,
    /// Is the player getting pit stop service.
    pub pitstop_active: Option<bool>,
    /// How many fast repairs used so far.
    pub fast_repair_used: Option<i32>,
    /// How many fast repairs left  255 is unlimited.
    pub fast_repair_available: Option<i32>,
    /// How many left front tires used so far.
    pub lf_tires_used: Option<i32>,
    /// How many right front tires used so far.
    pub rf_tires_used: Option<i32>,
    /// How many left rear tires used so far.
    pub lr_tires_used: Option<i32>,
    /// How many right rear tires used so far.
    pub rr_tires_used: Option<i32>,
    /// How many left tire sets used so far.
    pub left_tire_sets_used: Option<i32>,
    /// How many right tire sets used so far.
    pub right_tire_sets_used: Option<i32>,
    /// How many front tire sets used so far.
    pub front_tire_sets_used: Option<i32>,
    /// How many rear tire sets used so far.
    pub rear_tire_sets_used: Option<i32>,
    /// How many tire sets used so far.
    pub tire_sets_used: Option<i32>,
    /// How many left front tires are remaining  255 is unlimited.
    pub lf_tires_available: Option<i32>,
    /// How many right front tires are remaining  255 is unlimited.
    pub rf_tires_available: Option<i32>,
    /// How many left rear tires are remaining  255 is unlimited.
    pub lr_tires_available: Option<i32>,
    /// How many right rear tires are remaining  255 is unlimited.
    pub rr_tires_available: Option<i32>,
    /// How many left tire sets are remaining  255 is unlimited.
    pub left_tire_sets_available: Option<i32>,
    /// How many right tire sets are remaining  255 is unlimited.
    pub right_tire_sets_available: Option<i32>,
    /// How many front tire sets are remaining  255 is unlimited.
    pub front_tire_sets_available: Option<i32>,
    /// How many rear tire sets are remaining  255 is unlimited.
    pub rear_tire_sets_available: Option<i32>,
    /// How many tire sets are remaining  255 is unlimited.
    pub tire_sets_available: Option<i32>,
    /// Active camera's focus car index.
    pub cam_car_idx: Option<i32>,
    /// Active camera number.
    pub cam_camera_number: Option<i32>,
    /// Active camera group number.
    pub cam_group_number: Option<i32>,
    /// State of camera system.
    /// unit: irsdk_CameraState
    pub cam_camera_state: Option<CameraState>,
    /// 1=Car on track physics running.
    pub is_on_track_car: Option<bool>,
    /// 1=Car in garage physics running.
    pub is_in_garage: Option<bool>,
    /// Steering wheel max angle.
    /// unit: rad
    pub steering_wheel_angle_max: Option<f32>,
    /// Friction torque applied to gears when shifting or grinding.
    /// unit: %
    pub shift_power_pct: Option<f32>,
    /// RPM of shifter grinding noise.
    /// unit: RPM
    pub shift_grind_rpm: Option<f32>,
    /// Raw throttle input 0=off throttle to 1=full throttle.
    /// unit: %
    pub throttle_raw: Option<f32>,
    /// Raw brake input 0=brake released to 1=max pedal force.
    /// unit: %
    pub brake_raw: Option<f32>,
    /// Raw clutch input 0=disengaged to 1=fully engaged.
    /// unit: %
    pub clutch_raw: Option<f32>,
    /// Raw handbrake input 0=handbrake released to 1=max force.
    /// unit: %
    pub handbrake_raw: Option<f32>,
    /// true if abs is currently reducing brake force pressure.
    pub brake_ab_sactive: Option<bool>,
    /// Bitfield for warning lights.
    /// unit: irsdk_EngineWarnings
    pub engine_warnings: Option<EngineWarnings>,
    /// Percent fuel remaining.
    /// unit: %
    pub fuel_level_pct: Option<f32>,
    /// Bitfield of pit service checkboxes.
    /// unit: irsdk_PitSvFlags
    pub pit_sv_flags: Option<PitSvFlags>,
    /// Pit service left front tire pressure.
    /// unit: kPa
    pub pit_sv_lfp: Option<f32>,
    /// Pit service right front tire pressure.
    /// unit: kPa
    pub pit_sv_rfp: Option<f32>,
    /// Pit service left rear tire pressure.
    /// unit: kPa
    pub pit_sv_lrp: Option<f32>,
    /// Pit service right rear tire pressure.
    /// unit: kPa
    pub pit_sv_rrp: Option<f32>,
    /// Pit service fuel add amount.
    /// unit: l or kWh
    pub pit_sv_fuel: Option<f32>,
    /// Pit service pending tire compound.
    pub pit_sv_tire_compound: Option<i32>,
    /// Push2Pass active or not.
    pub car_idx_p2p_status: Option<Vec<bool>>,
    /// Push2Pass count of usage (or remaining in Race).
    pub car_idx_p2p_count: Option<Vec<i32>>,
    /// Force feedback % max torque on steering shaft unsigned.
    /// unit: %
    pub steering_wheel_pct_torque: Option<f32>,
    /// Force feedback % max torque on steering shaft signed.
    /// unit: %
    pub steering_wheel_pct_torque_sign: Option<f32>,
    /// Force feedback % max torque on steering shaft signed stops.
    /// unit: %
    pub steering_wheel_pct_torque_sign_stops: Option<f32>,
    /// Force feedback % max smoothing.
    /// unit: %
    pub steering_wheel_pct_smoothing: Option<f32>,
    /// Force feedback % max damping.
    /// unit: %
    pub steering_wheel_pct_damper: Option<f32>,
    /// Force feedback limiter strength limits impacts and oscillation.
    /// unit: %
    pub steering_wheel_limiter: Option<f32>,
    /// Value of strength or max force slider in Nm for FFB.
    /// unit: N*m
    pub steering_wheel_max_force_nm: Option<f32>,
    /// Peak torque mapping to direct input units for FFB.
    /// unit: N*m
    pub steering_wheel_peak_force_nm: Option<f32>,
    /// True if steering wheel force is using linear mode.
    pub steering_wheel_use_linear: Option<bool>,
    /// DEPRECATED use DriverCarSLBlinkRPM instead.
    /// unit: %
    pub shift_indicator_pct: Option<f32>,
    /// Replay playback speed.
    pub replay_play_speed: Option<i32>,
    /// 0=not slow motion  1=replay is in slow motion.
    pub replay_play_slow_motion: Option<bool>,
    /// Seconds since replay session start.
    /// unit: s
    pub replay_session_time: Option<Time>,
    /// Replay session number.
    pub replay_session_num: Option<i32>,
    /// Players LF Tire Sound rumblestrip pitch.
    /// unit: Hz
    pub tire_lf_rumble_pitch: Option<f32>,
    /// Players RF Tire Sound rumblestrip pitch.
    /// unit: Hz
    pub tire_rf_rumble_pitch: Option<f32>,
    /// Players LR Tire Sound rumblestrip pitch.
    /// unit: Hz
    pub tire_lr_rumble_pitch: Option<f32>,
    /// Players RR Tire Sound rumblestrip pitch.
    /// unit: Hz
    pub tire_rr_rumble_pitch: Option<f32>,
    /// 1=Garage screen is visible.
    pub is_garage_visible: Option<bool>,
    /// Output torque on steering shaft at 360 Hz.
    /// unit: N*m
    pub steering_wheel_torque_st: Option<Vec<f32>>,
    /// Output torque on steering shaft.
    /// unit: N*m
    pub steering_wheel_torque: Option<f32>,
    /// Z velocity.
    /// unit: m/s at 360 Hz
    pub velocity_z_st: Option<Vec<f32>>,
    /// Y velocity.
    /// unit: m/s at 360 Hz
    pub velocity_y_st: Option<Vec<f32>>,
    /// X velocity.
    /// unit: m/s at 360 Hz
    pub velocity_x_st: Option<Vec<f32>>,
    /// Z velocity.
    /// unit: m/s
    pub velocity_z: Option<f32>,
    /// Y velocity.
    /// unit: m/s
    pub velocity_y: Option<f32>,
    /// X velocity.
    /// unit: m/s
    pub velocity_x: Option<f32>,
    /// Yaw rate at 360 Hz.
    /// unit: rad/s
    pub yaw_rate_st: Option<Vec<f32>>,
    /// Pitch rate at 360 Hz.
    /// unit: rad/s
    pub pitch_rate_st: Option<Vec<f32>>,
    /// Roll rate at 360 Hz.
    /// unit: rad/s
    pub roll_rate_st: Option<Vec<f32>>,
    /// Yaw rate.
    /// unit: rad/s
    pub yaw_rate: Option<f32>,
    /// Pitch rate.
    /// unit: rad/s
    pub pitch_rate: Option<f32>,
    /// Roll rate.
    /// unit: rad/s
    pub roll_rate: Option<f32>,
    /// Vertical acceleration (including gravity) at 360 Hz.
    /// unit: m/s^2
    pub vert_accel_st: Option<Vec<f32>>,
    /// Lateral acceleration (including gravity) at 360 Hz.
    /// unit: m/s^2
    pub lat_accel_st: Option<Vec<f32>>,
    /// Longitudinal acceleration (including gravity) at 360 Hz.
    /// unit: m/s^2
    pub long_accel_st: Option<Vec<f32>>,
    /// Vertical acceleration (including gravity).
    /// unit: m/s^2
    pub vert_accel: Option<f32>,
    /// Lateral acceleration (including gravity).
    /// unit: m/s^2
    pub lat_accel: Option<f32>,
    /// Longitudinal acceleration (including gravity).
    /// unit: m/s^2
    pub long_accel: Option<f32>,
    /// In car trigger car starter.
    pub dc_starter: Option<bool>,
    /// In car dash display page adjustment.
    pub dc_dash_page: Option<f32>,
    /// In car tear off visor film.
    pub dc_tear_off_visor: Option<bool>,
    /// Pitstop all tire change request.
    pub dp_tire_change: Option<f32>,
    /// Pitstop fuel fill flag.
    pub dp_fuel_fill: Option<f32>,
    /// Pitstop fuel add amount.
    /// unit: kg
    pub dp_fuel_add_kg: Option<f32>,
    /// Pitstop fast repair set.
    pub dp_fast_repair: Option<f32>,
    /// In car brake bias adjustment.
    pub dc_brake_bias: Option<f32>,
    /// Pitstop lf tire cold pressure adjustment.
    /// unit: Pa
    pub dp_lf_tire_cold_press: Option<f32>,
    /// Pitstop rf cold tire pressure adjustment.
    /// unit: Pa
    pub dp_rf_tire_cold_press: Option<f32>,
    /// Pitstop lr tire cold pressure adjustment.
    /// unit: Pa
    pub dp_lr_tire_cold_press: Option<f32>,
    /// Pitstop rr cold tire pressure adjustment.
    /// unit: Pa
    pub dp_rr_tire_cold_press: Option<f32>,
    /// RF brake line pressure.
    /// unit: bar
    pub r_fbrake_line_press: Option<f32>,
    /// RF tire cold pressure  as set in the garage.
    /// unit: kPa
    pub r_fcold_pressure: Option<f32>,
    /// RF tire left carcass temperature.
    /// unit: C
    pub r_ftemp_cl: Option<f32>,
    /// RF tire middle carcass temperature.
    /// unit: C
    pub r_ftemp_cm: Option<f32>,
    /// RF tire right carcass temperature.
    /// unit: C
    pub r_ftemp_cr: Option<f32>,
    /// RF tire left percent tread remaining.
    /// unit: %
    pub r_fwear_l: Option<f32>,
    /// RF tire middle percent tread remaining.
    /// unit: %
    pub r_fwear_m: Option<f32>,
    /// RF tire right percent tread remaining.
    /// unit: %
    pub r_fwear_r: Option<f32>,
    /// LF brake line pressure.
    /// unit: bar
    pub l_fbrake_line_press: Option<f32>,
    /// LF tire cold pressure  as set in the garage.
    /// unit: kPa
    pub l_fcold_pressure: Option<f32>,
    /// LF tire left carcass temperature.
    /// unit: C
    pub l_ftemp_cl: Option<f32>,
    /// LF tire middle carcass temperature.
    /// unit: C
    pub l_ftemp_cm: Option<f32>,
    /// LF tire right carcass temperature.
    /// unit: C
    pub l_ftemp_cr: Option<f32>,
    /// LF tire left percent tread remaining.
    /// unit: %
    pub l_fwear_l: Option<f32>,
    /// LF tire middle percent tread remaining.
    /// unit: %
    pub l_fwear_m: Option<f32>,
    /// LF tire right percent tread remaining.
    /// unit: %
    pub l_fwear_r: Option<f32>,
    /// Engine fuel used instantaneous.
    /// unit: kg/h
    pub fuel_use_per_hour: Option<f32>,
    /// Engine voltage.
    /// unit: V
    pub voltage: Option<f32>,
    /// Engine coolant temp.
    /// unit: C
    pub water_temp: Option<f32>,
    /// Engine coolant level.
    /// unit: l
    pub water_level: Option<f32>,
    /// Engine fuel pressure.
    /// unit: bar
    pub fuel_press: Option<f32>,
    /// Engine oil temperature.
    /// unit: C
    pub oil_temp: Option<f32>,
    /// Engine oil pressure.
    /// unit: bar
    pub oil_press: Option<f32>,
    /// Engine oil level.
    /// unit: l
    pub oil_level: Option<f32>,
    /// Engine manifold pressure.
    /// unit: bar
    pub manifold_press: Option<f32>,
    /// Liters of fuel remaining.
    /// unit: l
    pub fuel_level: Option<f32>,
    /// Engine0Engine rpm.
    /// unit: revs/min
    pub engine0_rpm: Option<f32>,
    /// RR brake line pressure.
    /// unit: bar
    pub r_rbrake_line_press: Option<f32>,
    /// RR tire cold pressure  as set in the garage.
    /// unit: kPa
    pub r_rcold_pressure: Option<f32>,
    /// RR tire left carcass temperature.
    /// unit: C
    pub r_rtemp_cl: Option<f32>,
    /// RR tire middle carcass temperature.
    /// unit: C
    pub r_rtemp_cm: Option<f32>,
    /// RR tire right carcass temperature.
    /// unit: C
    pub r_rtemp_cr: Option<f32>,
    /// RR tire left percent tread remaining.
    /// unit: %
    pub r_rwear_l: Option<f32>,
    /// RR tire middle percent tread remaining.
    /// unit: %
    pub r_rwear_m: Option<f32>,
    /// RR tire right percent tread remaining.
    /// unit: %
    pub r_rwear_r: Option<f32>,
    /// LR brake line pressure.
    /// unit: bar
    pub l_rbrake_line_press: Option<f32>,
    /// LR tire cold pressure  as set in the garage.
    /// unit: kPa
    pub l_rcold_pressure: Option<f32>,
    /// LR tire left carcass temperature.
    /// unit: C
    pub l_rtemp_cl: Option<f32>,
    /// LR tire middle carcass temperature.
    /// unit: C
    pub l_rtemp_cm: Option<f32>,
    /// LR tire right carcass temperature.
    /// unit: C
    pub l_rtemp_cr: Option<f32>,
    /// LR tire left percent tread remaining.
    /// unit: %
    pub l_rwear_l: Option<f32>,
    /// LR tire middle percent tread remaining.
    /// unit: %
    pub l_rwear_m: Option<f32>,
    /// LR tire right percent tread remaining.
    /// unit: %
    pub l_rwear_r: Option<f32>,
    /// CR shock deflection.
    /// unit: m
    pub c_rshock_defl: Option<f32>,
    /// CR shock deflection at 360 Hz.
    /// unit: m
    pub c_rshock_defl_st: Option<Vec<f32>>,
    /// CR shock velocity.
    /// unit: m/s
    pub c_rshock_vel: Option<f32>,
    /// CR shock velocity at 360 Hz.
    /// unit: m/s
    pub c_rshock_vel_st: Option<Vec<f32>>,
    /// LR shock deflection.
    /// unit: m
    pub l_rshock_defl: Option<f32>,
    /// LR shock deflection at 360 Hz.
    /// unit: m
    pub l_rshock_defl_st: Option<Vec<f32>>,
    /// LR shock velocity.
    /// unit: m/s
    pub l_rshock_vel: Option<f32>,
    /// LR shock velocity at 360 Hz.
    /// unit: m/s
    pub l_rshock_vel_st: Option<Vec<f32>>,
    /// RR shock deflection.
    /// unit: m
    pub r_rshock_defl: Option<f32>,
    /// RR shock deflection at 360 Hz.
    /// unit: m
    pub r_rshock_defl_st: Option<Vec<f32>>,
    /// RR shock velocity.
    /// unit: m/s
    pub r_rshock_vel: Option<f32>,
    /// RR shock velocity at 360 Hz.
    /// unit: m/s
    pub r_rshock_vel_st: Option<Vec<f32>>,
    /// LF shock deflection.
    /// unit: m
    pub l_fshock_defl: Option<f32>,
    /// LF shock deflection at 360 Hz.
    /// unit: m
    pub l_fshock_defl_st: Option<Vec<f32>>,
    /// LF shock velocity.
    /// unit: m/s
    pub l_fshock_vel: Option<f32>,
    /// LF shock velocity at 360 Hz.
    /// unit: m/s
    pub l_fshock_vel_st: Option<Vec<f32>>,
    /// RF shock deflection.
    /// unit: m
    pub r_fshock_defl: Option<f32>,
    /// RF shock deflection at 360 Hz.
    /// unit: m
    pub r_fshock_defl_st: Option<Vec<f32>>,
    /// RF shock velocity.
    /// unit: m/s
    pub r_fshock_vel: Option<f32>,
    /// RF shock velocity at 360 Hz.
    /// unit: m/s
    pub r_fshock_vel_st: Option<Vec<f32>>,
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
