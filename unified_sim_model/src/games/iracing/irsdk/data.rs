use std::collections::{BTreeMap, HashMap};

use bitflags::bitflags;
use serde::{de::Visitor, Deserialize, Serialize};
use serde_value::Value;

use crate::{Angle, Distance, Pressure, Speed, Temperature, Time, Weight};

#[derive(Default, Clone)]
pub struct Data {
    pub session_data: SessionData,
    pub live_data: LiveData,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SessionData {
    pub weekend_info: Option<WeekendInfo>,
    pub session_info: Option<SessionInfo>,
    pub camera_info: Option<CameraInfo>,
    pub radio_info: Option<RadioInfo>,
    pub driver_info: Option<DriverInfo>,
    pub split_time_info: Option<SplitTimeInfo>,
    pub qualify_results_info: Option<QualifyResultsInfo>,
    pub car_setup: Option<CarSetup>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl SessionData {
    pub fn get_unmapped(&self) -> BTreeMap<Value, Value> {
        let prefix = "SessionData.".to_owned();
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.weekend_info
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.session_info
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.camera_info
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.radio_info
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.driver_info
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.split_time_info
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.qualify_results_info
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.car_setup
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WeekendInfo {
    pub track_name: Option<String>,
    #[serde(rename = "TrackID")]
    pub track_id: Option<i32>,
    #[serde(deserialize_with = "km_deserializer")]
    pub track_length: Option<Distance>,
    #[serde(deserialize_with = "km_deserializer")]
    pub track_length_official: Option<Distance>,
    pub track_display_name: Option<String>,
    pub track_display_short_name: Option<String>,
    pub track_config_name: Option<String>,
    pub track_city: Option<String>,
    pub track_country: Option<String>,
    #[serde(deserialize_with = "m_deserializer")]
    pub track_altitude: Option<Distance>,
    #[serde(deserialize_with = "decimal_degrees_deserializer")]
    pub track_latitude: Option<Angle>,
    #[serde(deserialize_with = "decimal_degrees_deserializer")]
    pub track_longitude: Option<Angle>,
    #[serde(deserialize_with = "rad_deserializer")]
    pub track_north_offset: Option<Angle>,
    pub track_num_turns: Option<i32>,
    #[serde(deserialize_with = "kph_deserializer")]
    pub track_pit_speed_limit: Option<Speed>,
    pub track_type: Option<String>,
    pub track_direction: Option<String>,
    pub track_weather_type: Option<String>,
    pub track_skies: Option<String>,
    #[serde(deserialize_with = "celcius_deserializer")]
    pub track_surface_temp: Option<Temperature>,
    #[serde(deserialize_with = "celcius_deserializer")]
    pub track_air_temp: Option<Temperature>,
    #[serde(deserialize_with = "air_pressure_deserializer")]
    pub track_air_pressure: Option<f32>,
    #[serde(deserialize_with = "ms_deserializer")]
    pub track_wind_vel: Option<Speed>,
    #[serde(deserialize_with = "rad_deserializer")]
    pub track_wind_dir: Option<Angle>,
    #[serde(deserialize_with = "percent_deserializer")]
    pub track_relative_humidity: Option<f32>,
    #[serde(deserialize_with = "percent_deserializer")]
    pub track_fog_level: Option<f32>,
    pub track_cleanup: Option<i32>,
    pub track_dynamic_track: Option<i32>,
    pub track_version: Option<String>,
    #[serde(rename = "SeriesID")]
    pub series_id: Option<i32>,
    #[serde(rename = "SeasonID")]
    pub season_id: Option<i32>,
    #[serde(rename = "SessionID")]
    pub session_id: Option<i32>,
    #[serde(rename = "SubSessionID")]
    pub sub_session_id: Option<i32>,
    #[serde(rename = "LeagueID")]
    pub league_id: Option<i32>,
    pub official: Option<i32>,
    pub race_week: Option<i32>,
    pub event_type: Option<String>,
    pub category: Option<String>,
    pub sim_mode: Option<String>,
    pub team_racing: Option<i32>,
    pub min_drivers: Option<i32>,
    pub max_drivers: Option<i32>,
    #[serde(rename = "DCRuleSet")]
    pub dc_rule_set: Option<String>,
    pub qualifier_must_start_race: Option<i32>,
    pub num_car_classes: Option<i32>,
    pub num_car_types: Option<i32>,
    pub heat_racing: Option<i32>,
    pub build_type: Option<String>,
    pub build_target: Option<String>,
    pub build_version: Option<String>,
    pub weekend_options: Option<WeekendOptions>,
    pub telemetry_options: Option<TelemetryOptions>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl WeekendInfo {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}WeekendInfo.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.weekend_options
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.telemetry_options
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeekendOptions {
    pub num_starters: Option<i32>,
    pub starting_grid: Option<String>,
    pub qualify_scoring: Option<String>,
    pub course_cautions: Option<String>,
    pub standing_start: Option<i32>,
    pub short_parade_lap: Option<i32>,
    pub restarts: Option<String>,
    pub weather_type: Option<String>,
    pub skies: Option<String>,
    pub wind_direction: Option<String>,
    #[serde(deserialize_with = "kmh_deserializer")]
    pub wind_speed: Option<Speed>,
    #[serde(deserialize_with = "celcius_deserializer")]
    pub weather_temp: Option<Temperature>,
    #[serde(deserialize_with = "percent_deserializer")]
    pub relative_humidity: Option<f32>,
    #[serde(deserialize_with = "percent_deserializer")]
    pub fog_level: Option<f32>,
    pub time_of_day: Option<String>,
    pub date: Option<String>,
    pub earth_rotation_speedup_factor: Option<i32>,
    pub unofficial: Option<i32>,
    pub commercial_mode: Option<String>,
    pub night_mode: Option<String>,
    pub is_fixed_setup: Option<i32>,
    pub strict_laps_checking: Option<String>,
    pub has_open_registration: Option<i32>,
    pub hardcore_level: Option<i32>,
    pub num_joker_laps: Option<i32>,
    pub incident_limit: Option<String>,
    pub fast_repairs_limit: Option<String>,
    pub green_white_checkered_limit: Option<i32>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl WeekendOptions {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}WeekendOptions.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TelemetryOptions {
    pub telemetry_disk_file: Option<String>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl TelemetryOptions {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}TelemetryOptions.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    pub sessions: Vec<Session>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl SessionInfo {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}SessionInfo.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.sessions
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Session {
    pub session_num: Option<i32>,
    pub session_laps: Option<String>,
    pub session_time: Option<String>,
    pub session_num_laps_to_avg: Option<i32>,
    pub session_type: Option<String>,
    pub session_track_rubber_state: Option<String>,
    pub session_name: Option<String>,
    pub session_sub_type: Option<String>,
    pub session_skipped: Option<i32>,
    pub session_run_groups_used: Option<i32>,
    pub session_enforce_tire_compound_change: Option<i32>,
    pub results_positions: Vec<ResultsPosition>,
    pub results_fastest_lap: Vec<ResultFastedLap>,
    pub results_average_lap_time: Option<f32>,
    pub results_num_caution_flags: Option<i32>,
    pub results_num_caution_laps: Option<i32>,
    pub results_num_lead_changes: Option<i32>,
    pub results_laps_complete: Option<i32>,
    pub results_official: Option<i32>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl Session {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}Session.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.results_positions
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        self.results_fastest_lap
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResultsPosition {
    pub position: Option<i32>,
    pub class_position: Option<i32>,
    pub car_idx: Option<i32>,
    pub lap: Option<i32>,
    pub time: Option<f32>,
    pub fastest_lap: Option<i32>,
    pub fastest_time: Option<f32>,
    pub last_time: Option<f32>,
    pub laps_led: Option<i32>,
    pub laps_complete: Option<i32>,
    pub joker_laps_complete: Option<i32>,
    pub laps_driven: Option<f32>,
    pub incidents: Option<i32>,
    pub reason_out_id: Option<i32>,
    pub reason_out_str: Option<String>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl ResultsPosition {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}ResultsPosition.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResultFastedLap {
    pub car_idx: Option<i32>,
    pub fastest_lap: Option<i32>,
    pub fastest_time: Option<f32>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl ResultFastedLap {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}.ResultFastedLap.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CameraInfo {
    pub groups: Vec<CameraGroup>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl CameraInfo {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}CameraInfo.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.groups
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CameraGroup {
    pub group_num: Option<i32>,
    pub group_name: Option<String>,
    pub is_scenic: Option<bool>,
    pub cameras: Vec<Camera>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl CameraGroup {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}CameraGroup.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.cameras
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Camera {
    pub camera_num: Option<i32>,
    pub camera_name: Option<String>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl Camera {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}Camera.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioInfo {
    pub selected_radio_num: Option<i32>,
    pub radios: Vec<Radio>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl RadioInfo {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}RadioInfo.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.radios
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Radio {
    pub radio_num: Option<i32>,
    pub hop_count: Option<i32>,
    pub num_frequencies: Option<i32>,
    pub tuned_to_frequency_num: Option<i32>,
    pub scanning_is_on: Option<i32>,
    pub frequencies: Vec<RadioFrequency>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl Radio {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}Radio.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.frequencies
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioFrequency {
    pub frequency_num: Option<i32>,
    pub frequency_name: Option<String>,
    pub priority: Option<i32>,
    pub car_idx: Option<i32>,
    pub entry_idx: Option<i32>,
    #[serde(rename = "ClubID")]
    pub club_id: Option<i32>,
    pub can_scan: Option<i32>,
    pub can_squawk: Option<i32>,
    pub muted: Option<i32>,
    pub is_mutable: Option<i32>,
    pub is_deletable: Option<i32>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl RadioFrequency {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}RadioFrequency.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DriverInfo {
    pub driver_car_idx: Option<i32>,
    #[serde(rename = "DriverUserID")]
    pub driver_user_id: Option<i32>,
    pub pace_car_idx: Option<i32>,
    pub driver_head_pos_x: Option<f32>,
    pub driver_head_pos_y: Option<f32>,
    pub driver_head_pos_z: Option<f32>,
    pub driver_car_is_electric: Option<i32>,
    #[serde(rename = "DriverCarIdleRPM")]
    pub driver_car_idle_rpm: Option<f32>,
    pub driver_car_red_line: Option<f32>,
    pub driver_car_eng_cylinder_count: Option<i32>,
    pub driver_car_fuel_kg_per_ltr: Option<f32>,
    pub driver_car_fuel_max_ltr: Option<f32>,
    pub driver_car_max_fuel_pct: Option<f32>,
    pub driver_car_gear_num_forward: Option<i32>,
    pub driver_car_gear_neutral: Option<i32>,
    pub driver_car_gear_reverse: Option<i32>,
    #[serde(rename = "DriverCarSLFirstRPM")]
    pub driver_car_sl_first_rpm: Option<f32>,
    #[serde(rename = "DriverCarSLShiftRPM")]
    pub driver_car_sl_shift_rpm: Option<f32>,
    #[serde(rename = "DriverCarSLLastRPM")]
    pub driver_car_sl_last_rpm: Option<f32>,
    #[serde(rename = "DriverCarSLBlinkRPM")]
    pub driver_car_sl_blink_rpm: Option<f32>,
    pub driver_car_version: Option<String>,
    pub driver_pit_trk_pct: Option<f32>,
    pub driver_car_est_lap_time: Option<f32>,
    pub driver_setup_name: Option<String>,
    pub driver_setup_is_modified: Option<i32>,
    pub driver_setup_load_type_name: Option<String>,
    pub driver_setup_passed_tech: Option<i32>,
    pub driver_incident_count: Option<i32>,
    pub drivers: Vec<Driver>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl DriverInfo {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}DriverInfo.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.drivers
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Driver {
    pub car_idx: Option<i32>,
    pub user_name: Option<String>,
    pub abbrev_name: Option<String>,
    pub initials: Option<String>,
    #[serde(rename = "UserID")]
    pub user_id: Option<i32>,
    #[serde(rename = "TeamID")]
    pub team_id: Option<i32>,
    pub team_name: Option<String>,
    pub car_number: Option<String>,
    pub car_number_raw: Option<String>,
    pub car_path: Option<String>,
    #[serde(rename = "CarClassID")]
    pub car_class_id: Option<i32>,
    #[serde(rename = "CarID")]
    pub car_id: Option<i32>,
    pub car_is_pace_car: Option<i32>,
    #[serde(rename = "CarIsAI")]
    pub car_is_ai: Option<i32>,
    pub car_is_electric: Option<i32>,
    pub car_screen_name: Option<String>,
    pub car_screen_name_short: Option<String>,
    pub car_class_short_name: Option<String>,
    pub car_class_rel_speed: Option<i32>,
    pub car_class_license_level: Option<i32>,
    #[serde(deserialize_with = "percent_deserializer")]
    pub car_class_max_fuel_pct: Option<f32>,
    #[serde(deserialize_with = "kg_deserializer")]
    pub car_class_weight_penalty: Option<Weight>,
    #[serde(deserialize_with = "percent_deserializer")]
    pub car_class_power_adjust: Option<f32>,
    #[serde(deserialize_with = "percent_deserializer")]
    pub car_class_dry_tire_set_limit: Option<f32>,
    pub car_class_color: Option<String>,
    pub car_class_est_lap_time: Option<f32>,
    pub i_rating: Option<i32>,
    pub lic_level: Option<i32>,
    pub lic_sub_level: Option<i32>,
    pub lic_string: Option<String>,
    pub lic_color: Option<String>,
    pub is_spectator: Option<i32>,
    pub car_design_str: Option<String>,
    pub helmet_design_str: Option<String>,
    pub suit_design_str: Option<String>,
    pub car_number_design_str: Option<String>,
    #[serde(rename = "CarSponsor_1")]
    pub car_sponsor_1: Option<i32>,
    #[serde(rename = "CarSponsor_2")]
    pub car_sponsor_2: Option<i32>,
    pub cur_driver_incident_count: Option<i32>,
    pub team_incident_count: Option<i32>,
    pub body_type: Option<i32>,
    pub face_type: Option<i32>,
    pub helmet_type: Option<i32>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl Driver {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}Driver.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SplitTimeInfo {
    pub sectors: Vec<Sector>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl SplitTimeInfo {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}SplitTimeInfo.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.sectors
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sector {
    pub sector_num: Option<i32>,
    pub sector_start_pct: Option<f32>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl Sector {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}Sector.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QualifyResultsInfo {
    pub results: Vec<QualifyResult>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl QualifyResultsInfo {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}QualifyResultsInfo.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.results
            .iter()
            .for_each(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QualifyResult {
    pub position: Option<i32>,
    pub class_position: Option<i32>,
    pub car_idx: Option<i32>,
    pub fastest_lap: Option<i32>,
    pub fastest_time: Option<f32>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl QualifyResult {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}QualifyResult.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarSetup {
    pub update_count: Option<i32>,
    pub tires: Option<Tires>,
    pub chassis: Option<Chassis>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl CarSetup {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}CarSetup.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.tires
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.chassis
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tires {
    pub left_front: Option<TireLeft>,
    pub left_rear: Option<TireLeft>,
    pub right_front: Option<TireRight>,
    pub right_rear: Option<TireRight>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl Tires {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}Tires.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.left_front
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.left_rear
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.right_front
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.right_rear
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TireLeft {
    #[serde(deserialize_with = "kpa_deserializer")]
    pub cold_pressure: Option<Pressure>,
    #[serde(deserialize_with = "kpa_deserializer")]
    pub last_hot_pressure: Option<Pressure>,
    #[serde(rename = "LastTempsOMI")]
    #[serde(deserialize_with = "tripple_celcius_deserializer_reverse")]
    pub last_temps_imo: Option<InnerMiddleOutside<Temperature>>,
    #[serde(deserialize_with = "tripple_percent_deserializer")]
    pub tread_remaining: Option<InnerMiddleOutside<f32>>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl TireLeft {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}TireLeft.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TireRight {
    #[serde(deserialize_with = "kpa_deserializer")]
    pub cold_pressure: Option<Pressure>,
    #[serde(deserialize_with = "kpa_deserializer")]
    pub last_hot_pressure: Option<Pressure>,
    #[serde(deserialize_with = "tripple_celcius_deserializer")]
    #[serde(rename = "LastTempsIMO")]
    pub last_temps_imo: Option<InnerMiddleOutside<Temperature>>,
    #[serde(deserialize_with = "tripple_percent_deserializer")]
    pub tread_remaining: Option<InnerMiddleOutside<f32>>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl TireRight {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}TireRight.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone)]
pub struct InnerMiddleOutside<T> {
    pub inner: T,
    pub middle: T,
    pub outside: T,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Chassis {
    pub front: Option<ChassisFront>,
    pub left_front: Option<ChassisCornerFront>,
    pub left_rear: Option<ChassisCornerRear>,
    pub rear: Option<ChassisRear>,
    pub right_front: Option<ChassisCornerFront>,
    pub right_rear: Option<ChassisCornerRear>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl Chassis {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}Chassis.");
        let mut map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        self.front
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.left_front
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.right_front
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.left_rear
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.right_rear
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        self.rear
            .as_ref()
            .map(|v| map.extend(v.get_unmapped(&prefix)));
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisFront {
    #[serde(deserialize_with = "mm_deserializer")]
    pub arb_diameter: Option<Distance>,
    #[serde(deserialize_with = "turns_deserializer")]
    pub spring_preload: Option<f32>,
    #[serde(deserialize_with = "percent_deserializer")]
    pub brake_pressure_bias: Option<f32>,
    pub screen_color: Option<String>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl ChassisFront {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}ChassisFront.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisCornerFront {
    #[serde(deserialize_with = "n_deserializer")]
    pub corner_weight: Option<f32>,
    #[serde(deserialize_with = "mm_deserializer")]
    pub ride_height: Option<Distance>,
    #[serde(deserialize_with = "clicks_deserializer")]
    pub shock_setting: Option<f32>,
    #[serde(deserialize_with = "deg_deserializer")]
    pub camber: Option<Angle>,
    #[serde(deserialize_with = "mm_deserializer")]
    pub toe_in: Option<Distance>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl ChassisCornerFront {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}ChassisCornerFront.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisRear {
    #[serde(deserialize_with = "mm_deserializer")]
    pub ride_height: Option<Distance>,
    #[serde(deserialize_with = "mm_deserializer")]
    pub pushrod_offset: Option<Distance>,
    #[serde(deserialize_with = "mm_deserializer")]
    pub spring_perch_offset: Option<Distance>,
    #[serde(deserialize_with = "n_per_mm_deserializer")]
    pub spring_rate: Option<f32>,
    #[serde(deserialize_with = "clicks_deserializer")]
    pub shock_setting: Option<f32>,
    #[serde(deserialize_with = "l_deserializer")]
    pub fuel_level: Option<f32>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl ChassisRear {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}ChassisRear.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisCornerRear {
    #[serde(deserialize_with = "n_deserializer")]
    pub corner_weight: Option<f32>,
    #[serde(deserialize_with = "deg_deserializer")]
    pub camber: Option<Angle>,
    #[serde(deserialize_with = "mm_deserializer")]
    pub toe_in: Option<Distance>,
    #[serde(flatten)]
    pub unmapped: HashMap<String, Value>,
}

impl ChassisCornerRear {
    pub(crate) fn get_unmapped(&self, prefix: &String) -> BTreeMap<Value, Value> {
        let prefix = format!("{prefix}ChassisCornerRear.");
        let map: BTreeMap<Value, Value> = self
            .unmapped
            .iter()
            .map(|(key, value)| (Value::String(format!("{prefix}{key}")), value.clone()))
            .collect();
        map
    }
}

struct UnitVisitor {
    unit: &'static str,
}
impl<'de> Visitor<'de> for UnitVisitor {
    type Value = f32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a value with the unit '{}'", self.unit)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if !v.ends_with(self.unit) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            ));
        }
        let value_str = v.trim_end_matches(self.unit).trim_end();
        Ok(str::parse(value_str.trim()).map_err(|e| serde::de::Error::custom(e))?)
    }
}

struct TrippleUnitVisitor {
    unit: &'static str,
}

impl<'de> Visitor<'de> for TrippleUnitVisitor {
    type Value = (f32, f32, f32);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "three comma seperated values with the unit '{}'",
            self.unit
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let values: Vec<f32> = v
            .split(",")
            .map(|v| {
                if !v.ends_with(self.unit) {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &self,
                    ));
                }
                let value_str = v.trim_end_matches(self.unit).trim_end();
                str::parse(value_str.trim()).map_err(|e| serde::de::Error::custom(e))
            })
            .collect::<Result<Vec<f32>, E>>()?;
        if values.len() < 3 {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            ));
        }

        Ok((values[0], values[1], values[2]))
    }
}

fn km_deserializer<'de, D>(deserializer: D) -> Result<Option<Distance>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "km" })
        .map(|km| Some(Distance::from_kilometers(km)))
}

fn m_deserializer<'de, D>(deserializer: D) -> Result<Option<Distance>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "m" })
        .map(|km| Some(Distance::from_kilometers(km)))
}

fn mm_deserializer<'de, D>(deserializer: D) -> Result<Option<Distance>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "mm" })
        .map(|km| Some(Distance::from_kilometers(km)))
}

fn decimal_degrees_deserializer<'de, D>(deserializer: D) -> Result<Option<Angle>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        // For some reason the latitude and longitude have m as their unit.
        .deserialize_str(UnitVisitor { unit: "m" })
        .map(|angle| Some(Angle::from_deg(angle)))
}

fn rad_deserializer<'de, D>(deserializer: D) -> Result<Option<Angle>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "rad" })
        .map(|angle| Some(Angle::from_rad(angle)))
}

fn deg_deserializer<'de, D>(deserializer: D) -> Result<Option<Angle>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "deg" })
        .map(|angle| Some(Angle::from_deg(angle)))
}

fn kph_deserializer<'de, D>(deserializer: D) -> Result<Option<Speed>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "kph" })
        .map(|kmh| Some(Speed::from_kmh(kmh)))
}

fn kmh_deserializer<'de, D>(deserializer: D) -> Result<Option<Speed>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "km/h" })
        .map(|kmh| Some(Speed::from_kmh(kmh)))
}

fn ms_deserializer<'de, D>(deserializer: D) -> Result<Option<Speed>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "m/s" })
        .map(|ms| Some(Speed::from_ms(ms)))
}

fn celcius_deserializer<'de, D>(deserializer: D) -> Result<Option<Temperature>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "C" })
        .map(|c| Some(Temperature::from_celcius(c)))
}

fn kg_deserializer<'de, D>(deserializer: D) -> Result<Option<Weight>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "kg" })
        .map(|kg| Some(Weight::from_kg(kg)))
}

fn percent_deserializer<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "%" })
        .map(|v| Some(v / 100.0))
}

fn air_pressure_deserializer<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "Hg" })
        .map(|v| Some(v))
}

fn kpa_deserializer<'de, D>(deserializer: D) -> Result<Option<Pressure>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "kPa" })
        .map(|v| Some(Pressure::from_kpa(v)))
}

fn turns_deserializer<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "Turns" })
        .map(|v| Some(v))
}

fn clicks_deserializer<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "clicks" })
        .map(|v| Some(v))
}

fn n_deserializer<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "N" })
        .map(|v| Some(v))
}

fn n_per_mm_deserializer<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "N/mm" })
        .map(|v| Some(v))
}

fn l_deserializer<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "L" })
        .map(|v| Some(v))
}

fn tripple_celcius_deserializer<'de, D>(
    deserializer: D,
) -> Result<Option<InnerMiddleOutside<Temperature>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(TrippleUnitVisitor { unit: "C" })
        .map(|(c1, c2, c3)| {
            Some(InnerMiddleOutside {
                inner: Temperature::from_celcius(c1),
                middle: Temperature::from_celcius(c2),
                outside: Temperature::from_celcius(c3),
            })
        })
}

fn tripple_celcius_deserializer_reverse<'de, D>(
    deserializer: D,
) -> Result<Option<InnerMiddleOutside<Temperature>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(TrippleUnitVisitor { unit: "C" })
        .map(|(c1, c2, c3)| {
            Some(InnerMiddleOutside {
                inner: Temperature::from_celcius(c3),
                middle: Temperature::from_celcius(c2),
                outside: Temperature::from_celcius(c1),
            })
        })
}

fn tripple_percent_deserializer<'de, D>(
    deserializer: D,
) -> Result<Option<InnerMiddleOutside<f32>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(TrippleUnitVisitor { unit: "%" })
        .map(|(c1, c2, c3)| {
            Some(InnerMiddleOutside {
                inner: c1,
                middle: c2,
                outside: c3,
            })
        })
}

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

#[derive(Clone)]
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
