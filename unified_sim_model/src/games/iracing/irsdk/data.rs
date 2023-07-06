use std::collections::{BTreeMap, HashMap};

use serde::{de::Visitor, Deserialize, Serialize};
use serde_value::Value;

use crate::{Angle, Distance, Pressure, Speed, Temperature, Weight};

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
    pub session_time: Option<f64>, // s
    pub session_tick: Option<i32>,
    pub session_num: Option<i32>,
    pub session_state: Option<i32>, // irsdk_SessionState
    pub session_unique_id: Option<i32>,
    pub session_flags: Option<i32>,      // irsdk_Flags
    pub session_time_remain: Option<f64>, // s
    pub session_laps_remain: Option<i32>,
    pub session_laps_remain_ex: Option<i32>,
    pub session_time_total: Option<f64>, // s
    pub session_laps_total: Option<i32>,
    pub session_joker_laps_remain: Option<i32>,
    pub session_on_joker_lap: Option<u8>,
    pub session_time_of_day: Option<f32>, // s
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
    pub frame_rate: Option<f32>,          // fps
    pub cpu_usage_fg: Option<f32>,         // %
    pub gpu_usage: Option<f32>,           // %
    pub chan_avg_latency: Option<f32>,     // s
    pub chan_latency: Option<f32>,        // s
    pub chan_quality: Option<f32>,        // %
    pub chan_partner_quality: Option<f32>, // %
    pub cpu_usage_bg: Option<f32>,         // %
    pub chan_clock_skew: Option<f32>,      // s
    pub mem_page_fault_sec: Option<f32>,
    pub mem_soft_page_fault_sec: Option<f32>,
    pub player_car_position: Option<i32>,
    pub player_car_class_position: Option<i32>,
    pub player_car_class: Option<i32>,
    pub player_track_surface: Option<i32>,         // irsdk_TrkLoc
    pub player_track_surface_material: Option<i32>, // irsdk_TrkSurf
    pub player_car_idx: Option<i32>,
    pub player_car_team_incident_count: Option<i32>,
    pub player_car_my_incident_count: Option<i32>,
    pub player_car_driver_incident_count: Option<i32>,
    pub player_car_weight_penalty: Option<f32>, // kg
    pub player_car_power_adjust: Option<f32>,   // %
    pub player_car_dry_tire_set_limit: Option<i32>,
    pub player_car_tow_time: Option<f32>, // s
    pub player_car_in_pit_stall: Option<u8>,
    pub player_car_pit_sv_status: Option<i32>, // irsdk_PitSvStatus
    pub player_tire_compound: Option<i32>,
    pub player_fast_repairs_used: Option<i32>,
    pub car_idx_lap: Option<Vec<i32>>,
    pub car_idx_lap_completed: Option<Vec<i32>>,
    pub car_idx_lap_dist_pct: Option<Vec<f32>>,           // %
    pub car_idx_track_surface: Option<Vec<i32>>,         // irsdk_TrkLoc
    pub car_idx_track_surface_material: Option<Vec<i32>>, // irsdk_TrkSurf
    pub car_idx_on_pit_road: Option<Vec<u8>>,
    pub car_idx_position: Option<Vec<i32>>,
    pub car_idx_class_position: Option<Vec<i32>>,
    pub car_idx_class: Option<Vec<i32>>,
    pub car_idx_f2_time: Option<Vec<f32>>,      // s
    pub car_idx_est_time: Option<Vec<f32>>,     // s
    pub car_idx_last_lap_time: Option<Vec<f32>>, // s
    pub car_idx_best_lap_time: Option<Vec<f32>>, // s
    pub car_idx_best_lap_num: Option<Vec<i32>>,
    pub car_idx_tire_compound: Option<Vec<i32>>,
    pub car_idx_qual_tire_compound: Option<Vec<i32>>,
    pub car_idx_qual_tire_compound_locked: Option<Vec<u8>>,
    pub car_idx_fast_repairs_used: Option<Vec<i32>>,
    pub car_idx_session_flags: Option<Vec<i32>>, // irsdk_Flags
    pub pace_mode: Option<i32>,                // irsdk_PaceMode
    pub car_idx_pace_line: Option<Vec<i32>>,
    pub car_idx_pace_row: Option<Vec<i32>>,
    pub car_idx_pace_flags: Option<Vec<i32>>, // irsdk_PaceFlags
    pub on_pit_road: Option<u8>,
    pub car_idx_steer: Option<Vec<f32>>, // rad
    pub car_idx_rpm: Option<Vec<f32>>,   // revs/min
    pub car_idx_gear: Option<Vec<i32>>,
    pub steering_wheel_angle: Option<f32>, // rad
    pub throttle: Option<f32>,           // %
    pub brake: Option<f32>,              // %
    pub clutch: Option<f32>,             // %
    pub gear: Option<i32>,
    pub rpm: Option<f32>, // revs/min
    pub lap: Option<i32>,
    pub lap_completed: Option<i32>,
    pub lap_dist: Option<f32>,    // m
    pub lap_dist_pct: Option<f32>, // %
    pub race_laps: Option<i32>,
    pub lap_best_lap: Option<i32>,
    pub lap_best_lap_time: Option<f32>,    // s
    pub lap_last_lap_time: Option<f32>,    // s
    pub lap_current_lap_time: Option<f32>, // s
    pub lap_las_n_lap_seq: Option<i32>,
    pub lap_last_n_lap_time: Option<f32>, // s
    pub lap_best_n_lap_lap: Option<i32>,
    pub lap_best_n_lap_time: Option<f32>,      // s
    pub lap_delta_to_best_lap: Option<f32>,    // s
    pub lap_delta_to_best_lap_dd: Option<f32>, // s/s
    pub lap_delta_to_best_lap_ok: Option<u8>,
    pub lap_delta_to_optimal_lap: Option<f32>,    // s
    pub lap_delta_to_optimal_lap_dd: Option<f32>, // s/s
    pub lap_delta_to_optimal_lap_ok: Option<u8>,
    pub lap_delta_to_session_best_lap: Option<f32>,    // s
    pub lap_delta_to_session_best_lap_dd: Option<f32>, // s/s
    pub lap_delta_to_session_best_lap_ok: Option<u8>,
    pub lap_delta_to_session_optimal_lap: Option<f32>,    // s
    pub lap_delta_to_session_optimal_lap_dd: Option<f32>, // s/s
    pub lap_delta_to_session_optimal_lap_ok: Option<u8>,
    pub lap_delta_to_session_lastl_lap: Option<f32>,    // s
    pub lap_delta_to_session_lastl_lap_dd: Option<f32>, // s/s
    pub lap_delta_to_session_lastl_lap_ok: Option<u8>,
    pub speed: Option<f32>,    // m/s
    pub yaw: Option<f32>,      // rad
    pub yaw_north: Option<f32>, // rad
    pub pitch: Option<f32>,    // rad
    pub roll: Option<f32>,     // rad
    pub enter_exit_reset: Option<i32>,
    pub track_temp: Option<f32>,     // C
    pub track_temp_crew: Option<f32>, // C
    pub air_temp: Option<f32>,       // C
    pub weather_type: Option<i32>,
    pub skies: Option<i32>,
    pub air_density: Option<f32>,       // kg/m^3
    pub air_pressure: Option<f32>,      // Hg
    pub wind_vel: Option<f32>,          // m/s
    pub wind_dir: Option<f32>,          // rad
    pub relative_humidity: Option<f32>, // %
    pub fog_level: Option<f32>,         // %
    pub solar_altitude: Option<f32>,    // rad
    pub solar_azimuth: Option<f32>,     // rad
    pub dc_lap_status: Option<i32>,
    pub dc_drivers_so_far: Option<i32>,
    pub ok_to_reload_textures: Option<u8>,
    pub load_num_textures: Option<u8>,
    pub car_left_right: Option<i32>, // irsdk_CarLeftRight
    pub pits_open: Option<u8>,
    pub vid_cap_enabled: Option<u8>,
    pub vid_cap_active: Option<u8>,
    pub pit_repair_left: Option<f32>,    // s
    pub pit_opt_repair_left: Option<f32>, // s
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
    pub cam_camera_state: Option<i32>, // irsdk_CameraState
    pub is_on_track_car: Option<u8>,
    pub is_in_garage: Option<u8>,
    pub steering_wheel_angle_max: Option<f32>, // rad
    pub shift_power_pct: Option<f32>,         // %
    pub shift_grind_rpm: Option<f32>,         // RPM
    pub throttle_raw: Option<f32>,           // %
    pub brake_raw: Option<f32>,              // %
    pub clutch_raw: Option<f32>,             // %
    pub handbrake_raw: Option<f32>,          // %
    pub brake_ab_sactive: Option<u8>,
    pub engine_warnings: Option<i32>, // irsdk_EngineWarnings
    pub fuel_level_pct: Option<f32>,   // %
    pub pit_sv_flags: Option<i32>,     // irsdk_PitSvFlags
    pub pit_sv_lfp: Option<f32>,       // kPa
    pub pit_sv_rfp: Option<f32>,       // kPa
    pub pit_sv_lrp: Option<f32>,       // kPa
    pub pit_sv_rrp: Option<f32>,       // kPa
    pub pit_sv_fuel: Option<f32>,      // l or kWh
    pub pit_sv_tire_compound: Option<i32>,
    pub car_idx_p2p_status: Option<Vec<u8>>,
    pub car_idx_p2p_count: Option<Vec<i32>>,
    pub steering_wheel_pct_torque: Option<f32>,          // %
    pub steering_wheel_pct_torque_sign: Option<f32>,      // %
    pub steering_wheel_pct_torque_sign_stops: Option<f32>, // %
    pub steering_wheel_pct_smoothing: Option<f32>,       // %
    pub steering_wheel_pct_damper: Option<f32>,          // %
    pub steering_wheel_limiter: Option<f32>,            // %
    pub steering_wheel_max_force_nm: Option<f32>,         // N*m
    pub steering_wheel_peak_force_nm: Option<f32>,        // N*m
    pub steering_wheel_use_linear: Option<u8>,
    pub shift_indicator_pct: Option<f32>, // %
    pub replay_play_speed: Option<i32>,
    pub replay_play_slow_motion: Option<u8>,
    pub replay_session_time: Option<f64>, // s
    pub replay_session_num: Option<i32>,
    pub tire_lf_rumble_pitch: Option<f32>, // Hz
    pub tire_rf_rumble_pitch: Option<f32>, // Hz
    pub tire_lr_rumble_pitch: Option<f32>, // Hz
    pub tire_rr_rumble_pitch: Option<f32>, // Hz
    pub is_garage_visible: Option<u8>,
    pub steering_wheel_torque_st: Option<f32>, // N*m
    pub steering_wheel_torque: Option<f32>,    // N*m
    pub velocity_z_st: Option<f32>,           // m/s at 360 Hz
    pub velocity_y_st: Option<f32>,           // m/s at 360 Hz
    pub velocity_x_st: Option<f32>,           // m/s at 360 Hz
    pub velocity_z: Option<f32>,              // m/s
    pub velocity_y: Option<f32>,              // m/s
    pub velocity_x: Option<f32>,              // m/s
    pub yaw_rate_st: Option<f32>,             // rad/s
    pub pitch_rate_st: Option<f32>,           // rad/s
    pub roll_rate_st: Option<f32>,            // rad/s
    pub yaw_rate: Option<f32>,                // rad/s
    pub pitch_rate: Option<f32>,              // rad/s
    pub roll_rate: Option<f32>,               // rad/s
    pub vert_accel_st: Option<f32>,           // m/s^2
    pub lat_accel_st: Option<f32>,            // m/s^2
    pub long_accel_st: Option<f32>,           // m/s^2
    pub vert_accel: Option<f32>,              // m/s^2
    pub lat_accel: Option<f32>,               // m/s^2
    pub long_accel: Option<f32>,              // m/s^2
    pub dc_starter: Option<u8>,
    pub dc_dash_page: Option<f32>,
    pub dc_tear_off_visor: Option<u8>,
    pub dp_tire_change: Option<f32>,
    pub dp_fuel_fill: Option<f32>,
    pub dp_fuel_add_kg: Option<f32>, // kg
    pub dp_fast_repair: Option<f32>,
    pub dc_brake_bias: Option<f32>,
    pub dp_lf_tire_cold_press: Option<f32>, // Pa
    pub dp_rf_tire_cold_press: Option<f32>, // Pa
    pub dp_lr_tire_cold_press: Option<f32>, // Pa
    pub dp_rr_tire_cold_press: Option<f32>, // Pa
    pub r_fbrake_line_press: Option<f32>,  // bar
    pub r_fcold_pressure: Option<f32>,    // kPa
    pub r_ftemp_cl: Option<f32>,          // C
    pub r_ftemp_cm: Option<f32>,          // C
    pub r_ftemp_cr: Option<f32>,          // C
    pub r_fwear_l: Option<f32>,           // %
    pub r_fwear_m: Option<f32>,           // %
    pub r_fwear_r: Option<f32>,           // %
    pub l_fbrake_line_press: Option<f32>,  // bar
    pub l_fcold_pressure: Option<f32>,    // kPa
    pub l_ftemp_cl: Option<f32>,          // C
    pub l_ftemp_cm: Option<f32>,          // C
    pub l_ftemp_cr: Option<f32>,          // C
    pub l_fwear_l: Option<f32>,           // %
    pub l_fwear_m: Option<f32>,           // %
    pub l_fwear_r: Option<f32>,           // %
    pub fuel_use_per_hour: Option<f32>,    // kg/h
    pub voltage: Option<f32>,           // V
    pub water_temp: Option<f32>,         // C
    pub water_level: Option<f32>,        // l
    pub fuel_press: Option<f32>,         // bar
    pub oil_temp: Option<f32>,           // C
    pub oil_press: Option<f32>,          // bar
    pub oil_level: Option<f32>,          // l
    pub manifold_press: Option<f32>,     // bar
    pub fuel_level: Option<f32>,         // l
    pub engine0_rpm: Option<f32>,       // revs/min
    pub r_rbrake_line_press: Option<f32>,  // bar
    pub r_rcold_pressure: Option<f32>,    // kPa
    pub r_rtemp_cl: Option<f32>,          // C
    pub r_rtemp_cm: Option<f32>,          // C
    pub r_rtemp_cr: Option<f32>,          // C
    pub r_rwear_l: Option<f32>,           // %
    pub r_rwear_m: Option<f32>,           // %
    pub r_rwear_r: Option<f32>,           // %
    pub l_rbrake_line_press: Option<f32>,  // bar
    pub l_rcold_pressure: Option<f32>,    // kPa
    pub l_rtemp_cl: Option<f32>,          // C
    pub l_rtemp_cm: Option<f32>,          // C
    pub l_rtemp_cr: Option<f32>,          // C
    pub l_rwear_l: Option<f32>,           // %
    pub l_rwear_m: Option<f32>,           // %
    pub l_rwear_r: Option<f32>,           // %
    pub c_rshock_defl: Option<f32>,       // m
    pub c_rshock_defl_st: Option<f32>,    // m
    pub c_rshock_vel: Option<f32>,        // m/s
    pub c_rshock_vel_st: Option<f32>,     // m/s
    pub l_rshock_defl: Option<f32>,       // m
    pub l_rshock_defl_st: Option<f32>,    // m
    pub l_rshock_vel: Option<f32>,        // m/s
    pub l_rshock_vel_st: Option<f32>,     // m/s
    pub r_rshock_defl: Option<f32>,       // m
    pub r_rshock_defl_st: Option<f32>,    // m
    pub r_rshock_vel: Option<f32>,        // m/s
    pub r_rshock_vel_st: Option<f32>,     // m/s
    pub l_fshock_defl: Option<f32>,       // m
    pub l_fshock_defl_st: Option<f32>,    // m
    pub l_fshock_vel: Option<f32>,        // m/s
    pub l_fshock_vel_st: Option<f32>,     // m/s
    pub r_fshock_defl: Option<f32>,       // m
    pub r_fshock_defl_st: Option<f32>,    // m
    pub r_fshock_vel: Option<f32>,        // m/s
    pub r_fshock_vel_st: Option<f32>,     // m/s
}
/*
name: SessionTime, desc: Seconds since session start, unit: s, type: 5, count: 1, count_as_time: false
name: SessionTick, desc: Current update number, unit: , type: 2, count: 1, count_as_time: false
name: SessionNum, desc: Session number, unit: , type: 2, count: 1, count_as_time: false
name: SessionState, desc: Session state, unit: irsdk_SessionState, type: 2, count: 1, count_as_time: false
name: SessionUniqueID, desc: Session ID, unit: , type: 2, count: 1, count_as_time: false
name: SessionFlags, desc: Session flags, unit: irsdk_Flags, type: 3, count: 1, count_as_time: false
name: SessionTimeRemain, desc: Seconds left till session ends, unit: s, type: 5, count: 1, count_as_time: false
name: SessionLapsRemain, desc: Old laps left till session ends use SessionLapsRemainEx, unit: , type: 2, count: 1, count_as_time: false
name: SessionLapsRemainEx, desc: New improved laps left till session ends, unit: , type: 2, count: 1, count_as_time: false
name: SessionTimeTotal, desc: Total number of seconds in session, unit: s, type: 5, count: 1, count_as_time: false
name: SessionLapsTotal, desc: Total number of laps in session, unit: , type: 2, count: 1, count_as_time: false
name: SessionJokerLapsRemain, desc: Joker laps remaining to be taken, unit: , type: 2, count: 1, count_as_time: false
name: SessionOnJokerLap, desc: Player is currently completing a joker lap, unit: , type: 1, count: 1, count_as_time: false
name: SessionTimeOfDay, desc: Time of day in seconds, unit: s, type: 4, count: 1, count_as_time: false
name: RadioTransmitCarIdx, desc: The car index of the current person speaking on the radio, unit: , type: 2, count: 1, count_as_time: false
name: RadioTransmitRadioIdx, desc: The radio index of the current person speaking on the radio, unit: , type: 2, count: 1, count_as_time: false
name: RadioTransmitFrequencyIdx, desc: The frequency index of the current person speaking on the radio, unit: , type: 2, count: 1, count_as_time: false
name: DisplayUnits, desc: Default units for the user interface 0 = english 1 = metric, unit: , type: 2, count: 1, count_as_time: false
name: DriverMarker, desc: Driver activated flag, unit: , type: 1, count: 1, count_as_time: false
name: PushToTalk, desc: Push to talk button state, unit: , type: 1, count: 1, count_as_time: false
name: PushToPass, desc: Push to pass button state, unit: , type: 1, count: 1, count_as_time: false
name: ManualBoost, desc: Hybrid manual boost state, unit: , type: 1, count: 1, count_as_time: false
name: ManualNoBoost, desc: Hybrid manual no boost state, unit: , type: 1, count: 1, count_as_time: false
name: IsOnTrack, desc: 1=Car on track physics running with player in car, unit: , type: 1, count: 1, count_as_time: false
name: IsReplayPlaying, desc: 0=replay not playing  1=replay playing, unit: , type: 1, count: 1, count_as_time: false
name: ReplayFrameNum, desc: Integer replay frame number (60 per second), unit: , type: 2, count: 1, count_as_time: false
name: ReplayFrameNumEnd, desc: Integer replay frame number from end of tape, unit: , type: 2, count: 1, count_as_time: false
name: IsDiskLoggingEnabled, desc: 0=disk based telemetry turned off  1=turned on, unit: , type: 1, count: 1, count_as_time: false
name: IsDiskLoggingActive, desc: 0=disk based telemetry file not being written  1=being written, unit: , type: 1, count: 1, count_as_time: false
name: FrameRate, desc: Average frames per second, unit: fps, type: 4, count: 1, count_as_time: false
name: CpuUsageFG, desc: Percent of available tim fg thread took with a 1 sec avg, unit: %, type: 4, count: 1, count_as_time: false
name: GpuUsage, desc: Percent of available tim gpu took with a 1 sec avg, unit: %, type: 4, count: 1, count_as_time: false
name: ChanAvgLatency, desc: Communications average latency, unit: s, type: 4, count: 1, count_as_time: false
name: ChanLatency, desc: Communications latency, unit: s, type: 4, count: 1, count_as_time: false
name: ChanQuality, desc: Communications quality, unit: %, type: 4, count: 1, count_as_time: false
name: ChanPartnerQuality, desc: Partner communications quality, unit: %, type: 4, count: 1, count_as_time: false
name: CpuUsageBG, desc: Percent of available tim bg thread took with a 1 sec avg, unit: %, type: 4, count: 1, count_as_time: false
name: ChanClockSkew, desc: Communications server clock skew, unit: s, type: 4, count: 1, count_as_time: false
name: MemPageFaultSec, desc: Memory page faults per second, unit: , type: 4, count: 1, count_as_time: false
name: MemSoftPageFaultSec, desc: Memory soft page faults per second, unit: , type: 4, count: 1, count_as_time: false
name: PlayerCarPosition, desc: Players position in race, unit: , type: 2, count: 1, count_as_time: false
name: PlayerCarClassPosition, desc: Players class position in race, unit: , type: 2, count: 1, count_as_time: false
name: PlayerCarClass, desc: Player car class id, unit: , type: 2, count: 1, count_as_time: false
name: PlayerTrackSurface, desc: Players car track surface type, unit: irsdk_TrkLoc, type: 2, count: 1, count_as_time: false
name: PlayerTrackSurfaceMaterial, desc: Players car track surface material type, unit: irsdk_TrkSurf, type: 2, count: 1, count_as_time: false
name: PlayerCarIdx, desc: Players carIdx, unit: , type: 2, count: 1, count_as_time: false
name: PlayerCarTeamIncidentCount, desc: Players team incident count for this session, unit: , type: 2, count: 1, count_as_time: false
name: PlayerCarMyIncidentCount, desc: Players own incident count for this session, unit: , type: 2, count: 1, count_as_time: false
name: PlayerCarDriverIncidentCount, desc: Teams current drivers incident count for this session, unit: , type: 2, count: 1, count_as_time: false
name: PlayerCarWeightPenalty, desc: Players weight penalty, unit: kg, type: 4, count: 1, count_as_time: false
name: PlayerCarPowerAdjust, desc: Players power adjust, unit: %, type: 4, count: 1, count_as_time: false
name: PlayerCarDryTireSetLimit, desc: Players dry tire set limit, unit: , type: 2, count: 1, count_as_time: false
name: PlayerCarTowTime, desc: Players car is being towed if time is greater than zero, unit: s, type: 4, count: 1, count_as_time: false
name: PlayerCarInPitStall, desc: Players car is properly in there pitstall, unit: , type: 1, count: 1, count_as_time: false
name: PlayerCarPitSvStatus, desc: Players car pit service status bits, unit: irsdk_PitSvStatus, type: 2, count: 1, count_as_time: false
name: PlayerTireCompound, desc: Players car current tire compound, unit: , type: 2, count: 1, count_as_time: false
name: PlayerFastRepairsUsed, desc: Players car number of fast repairs used, unit: , type: 2, count: 1, count_as_time: false
name: CarIdxLap, desc: Laps started by car index, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxLapCompleted, desc: Laps completed by car index, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxLapDistPct, desc: Percentage distance around lap by car index, unit: %, type: 4, count: 64, count_as_time: false
name: CarIdxTrackSurface, desc: Track surface type by car index, unit: irsdk_TrkLoc, type: 2, count: 64, count_as_time: false
name: CarIdxTrackSurfaceMaterial, desc: Track surface material type by car index, unit: irsdk_TrkSurf, type: 2, count: 64, count_as_time: false
name: CarIdxOnPitRoad, desc: On pit road between the cones by car index, unit: , type: 1, count: 64, count_as_time: false
name: CarIdxPosition, desc: Cars position in race by car index, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxClassPosition, desc: Cars class position in race by car index, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxClass, desc: Cars class id by car index, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxF2Time, desc: Race time behind leader or fastest lap time otherwise, unit: s, type: 4, count: 64, count_as_time: false
name: CarIdxEstTime, desc: Estimated time to reach current location on track, unit: s, type: 4, count: 64, count_as_time: false
name: CarIdxLastLapTime, desc: Cars last lap time, unit: s, type: 4, count: 64, count_as_time: false
name: CarIdxBestLapTime, desc: Cars best lap time, unit: s, type: 4, count: 64, count_as_time: false
name: CarIdxBestLapNum, desc: Cars best lap number, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxTireCompound, desc: Cars current tire compound, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxQualTireCompound, desc: Cars Qual tire compound, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxQualTireCompoundLocked, desc: Cars Qual tire compound is locked-in, unit: , type: 1, count: 64, count_as_time: false
name: CarIdxFastRepairsUsed, desc: How many fast repairs each car has used, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxSessionFlags, desc: Session flags for each player, unit: irsdk_Flags, type: 3, count: 64, count_as_time: false
name: PaceMode, desc: Are we pacing or not, unit: irsdk_PaceMode, type: 2, count: 1, count_as_time: false
name: CarIdxPaceLine, desc: What line cars are pacing in  or -1 if not pacing, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxPaceRow, desc: What row cars are pacing in  or -1 if not pacing, unit: , type: 2, count: 64, count_as_time: false
name: CarIdxPaceFlags, desc: Pacing status flags for each car, unit: irsdk_PaceFlags, type: 2, count: 64, count_as_time: false
name: OnPitRoad, desc: Is the player car on pit road between the cones, unit: , type: 1, count: 1, count_as_time: false
name: CarIdxSteer, desc: Steering wheel angle by car index, unit: rad, type: 4, count: 64, count_as_time: false
name: CarIdxRPM, desc: Engine rpm by car index, unit: revs/min, type: 4, count: 64, count_as_time: false
name: CarIdxGear, desc: -1=reverse  0=neutral  1..n=current gear by car index, unit: , type: 2, count: 64, count_as_time: false
name: SteeringWheelAngle, desc: Steering wheel angle, unit: rad, type: 4, count: 1, count_as_time: false
name: Throttle, desc: 0=off throttle to 1=full throttle, unit: %, type: 4, count: 1, count_as_time: false
name: Brake, desc: 0=brake released to 1=max pedal force, unit: %, type: 4, count: 1, count_as_time: false
name: Clutch, desc: 0=disengaged to 1=fully engaged, unit: %, type: 4, count: 1, count_as_time: false
name: Gear, desc: -1=reverse  0=neutral  1..n=current gear, unit: , type: 2, count: 1, count_as_time: false
name: RPM, desc: Engine rpm, unit: revs/min, type: 4, count: 1, count_as_time: false
name: Lap, desc: Laps started count, unit: , type: 2, count: 1, count_as_time: false
name: LapCompleted, desc: Laps completed count, unit: , type: 2, count: 1, count_as_time: false
name: LapDist, desc: Meters traveled from S/F this lap, unit: m, type: 4, count: 1, count_as_time: false
name: LapDistPct, desc: Percentage distance around lap, unit: %, type: 4, count: 1, count_as_time: false
name: RaceLaps, desc: Laps completed in race, unit: , type: 2, count: 1, count_as_time: false
name: LapBestLap, desc: Players best lap number, unit: , type: 2, count: 1, count_as_time: false
name: LapBestLapTime, desc: Players best lap time, unit: s, type: 4, count: 1, count_as_time: false
name: LapLastLapTime, desc: Players last lap time, unit: s, type: 4, count: 1, count_as_time: false
name: LapCurrentLapTime, desc: Estimate of players current lap time as shown in F3 box, unit: s, type: 4, count: 1, count_as_time: false
name: LapLasNLapSeq, desc: Player num consecutive clean laps completed for N average, unit: , type: 2, count: 1, count_as_time: false
name: LapLastNLapTime, desc: Player last N average lap time, unit: s, type: 4, count: 1, count_as_time: false
name: LapBestNLapLap, desc: Player last lap in best N average lap time, unit: , type: 2, count: 1, count_as_time: false
name: LapBestNLapTime, desc: Player best N average lap time, unit: s, type: 4, count: 1, count_as_time: false
name: LapDeltaToBestLap, desc: Delta time for best lap, unit: s, type: 4, count: 1, count_as_time: false
name: LapDeltaToBestLap_DD, desc: Rate of change of delta time for best lap, unit: s/s, type: 4, count: 1, count_as_time: false
name: LapDeltaToBestLap_OK, desc: Delta time for best lap is valid, unit: , type: 1, count: 1, count_as_time: false
name: LapDeltaToOptimalLap, desc: Delta time for optimal lap, unit: s, type: 4, count: 1, count_as_time: false
name: LapDeltaToOptimalLap_DD, desc: Rate of change of delta time for optimal lap, unit: s/s, type: 4, count: 1, count_as_time: false
name: LapDeltaToOptimalLap_OK, desc: Delta time for optimal lap is valid, unit: , type: 1, count: 1, count_as_time: false
name: LapDeltaToSessionBestLap, desc: Delta time for session best lap, unit: s, type: 4, count: 1, count_as_time: false
name: LapDeltaToSessionBestLap_DD, desc: Rate of change of delta time for session best lap, unit: s/s, type: 4, count: 1, count_as_time: false
name: LapDeltaToSessionBestLap_OK, desc: Delta time for session best lap is valid, unit: , type: 1, count: 1, count_as_time: false
name: LapDeltaToSessionOptimalLap, desc: Delta time for session optimal lap, unit: s, type: 4, count: 1, count_as_time: false
name: LapDeltaToSessionOptimalLap_DD, desc: Rate of change of delta time for session optimal lap, unit: s/s, type: 4, count: 1, count_as_time: false
name: LapDeltaToSessionOptimalLap_OK, desc: Delta time for session optimal lap is valid, unit: , type: 1, count: 1, count_as_time: false
name: LapDeltaToSessionLastlLap, desc: Delta time for session last lap, unit: s, type: 4, count: 1, count_as_time: false
name: LapDeltaToSessionLastlLap_DD, desc: Rate of change of delta time for session last lap, unit: s/s, type: 4, count: 1, count_as_time: false
name: LapDeltaToSessionLastlLap_OK, desc: Delta time for session last lap is valid, unit: , type: 1, count: 1, count_as_time: false
name: Speed, desc: GPS vehicle speed, unit: m/s, type: 4, count: 1, count_as_time: false
name: Yaw, desc: Yaw orientation, unit: rad, type: 4, count: 1, count_as_time: false
name: YawNorth, desc: Yaw orientation relative to north, unit: rad, type: 4, count: 1, count_as_time: false
name: Pitch, desc: Pitch orientation, unit: rad, type: 4, count: 1, count_as_time: false
name: Roll, desc: Roll orientation, unit: rad, type: 4, count: 1, count_as_time: false
name: EnterExitReset, desc: Indicate action the reset key will take 0 enter 1 exit 2 reset, unit: , type: 2, count: 1, count_as_time: false
name: TrackTemp, desc: Deprecated  set to TrackTempCrew, unit: C, type: 4, count: 1, count_as_time: false
name: TrackTempCrew, desc: Temperature of track measured by crew around track, unit: C, type: 4, count: 1, count_as_time: false
name: AirTemp, desc: Temperature of air at start/finish line, unit: C, type: 4, count: 1, count_as_time: false
name: WeatherType, desc: Weather type (0=constant  1=dynamic), unit: , type: 2, count: 1, count_as_time: false
name: Skies, desc: Skies (0=clear/1=p cloudy/2=m cloudy/3=overcast), unit: , type: 2, count: 1, count_as_time: false
name: AirDensity, desc: Density of air at start/finish line, unit: kg/m^3, type: 4, count: 1, count_as_time: false
name: AirPressure, desc: Pressure of air at start/finish line, unit: Hg, type: 4, count: 1, count_as_time: false
name: WindVel, desc: Wind velocity at start/finish line, unit: m/s, type: 4, count: 1, count_as_time: false
name: WindDir, desc: Wind direction at start/finish line, unit: rad, type: 4, count: 1, count_as_time: false
name: RelativeHumidity, desc: Relative Humidity, unit: %, type: 4, count: 1, count_as_time: false
name: FogLevel, desc: Fog level, unit: %, type: 4, count: 1, count_as_time: false
name: SolarAltitude, desc: Sun angle above horizon in radians, unit: rad, type: 4, count: 1, count_as_time: false
name: SolarAzimuth, desc: Sun angle clockwise from north in radians, unit: rad, type: 4, count: 1, count_as_time: false
name: DCLapStatus, desc: Status of driver change lap requirements, unit: , type: 2, count: 1, count_as_time: false
name: DCDriversSoFar, desc: Number of team drivers who have run a stint, unit: , type: 2, count: 1, count_as_time: false
name: OkToReloadTextures, desc: True if it is ok to reload car textures at this time, unit: , type: 1, count: 1, count_as_time: false
name: LoadNumTextures, desc: True if the car_num texture will be loaded, unit: , type: 1, count: 1, count_as_time: false
name: CarLeftRight, desc: Notify if car is to the left or right of driver, unit: irsdk_CarLeftRight, type: 3, count: 1, count_as_time: false
name: PitsOpen, desc: True if pit stop is allowed for the current player, unit: , type: 1, count: 1, count_as_time: false
name: VidCapEnabled, desc: True if video capture system is enabled, unit: , type: 1, count: 1, count_as_time: false
name: VidCapActive, desc: True if video currently being captured, unit: , type: 1, count: 1, count_as_time: false
name: PitRepairLeft, desc: Time left for mandatory pit repairs if repairs are active, unit: s, type: 4, count: 1, count_as_time: false
name: PitOptRepairLeft, desc: Time left for optional repairs if repairs are active, unit: s, type: 4, count: 1, count_as_time: false
name: PitstopActive, desc: Is the player getting pit stop service, unit: , type: 1, count: 1, count_as_time: false
name: FastRepairUsed, desc: How many fast repairs used so far, unit: , type: 2, count: 1, count_as_time: false
name: FastRepairAvailable, desc: How many fast repairs left  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: LFTiresUsed, desc: How many left front tires used so far, unit: , type: 2, count: 1, count_as_time: false
name: RFTiresUsed, desc: How many right front tires used so far, unit: , type: 2, count: 1, count_as_time: false
name: LRTiresUsed, desc: How many left rear tires used so far, unit: , type: 2, count: 1, count_as_time: false
name: RRTiresUsed, desc: How many right rear tires used so far, unit: , type: 2, count: 1, count_as_time: false
name: LeftTireSetsUsed, desc: How many left tire sets used so far, unit: , type: 2, count: 1, count_as_time: false
name: RightTireSetsUsed, desc: How many right tire sets used so far, unit: , type: 2, count: 1, count_as_time: false
name: FrontTireSetsUsed, desc: How many front tire sets used so far, unit: , type: 2, count: 1, count_as_time: false
name: RearTireSetsUsed, desc: How many rear tire sets used so far, unit: , type: 2, count: 1, count_as_time: false
name: TireSetsUsed, desc: How many tire sets used so far, unit: , type: 2, count: 1, count_as_time: false
name: LFTiresAvailable, desc: How many left front tires are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: RFTiresAvailable, desc: How many right front tires are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: LRTiresAvailable, desc: How many left rear tires are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: RRTiresAvailable, desc: How many right rear tires are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: LeftTireSetsAvailable, desc: How many left tire sets are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: RightTireSetsAvailable, desc: How many right tire sets are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: FrontTireSetsAvailable, desc: How many front tire sets are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: RearTireSetsAvailable, desc: How many rear tire sets are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: TireSetsAvailable, desc: How many tire sets are remaining  255 is unlimited, unit: , type: 2, count: 1, count_as_time: false
name: CamCarIdx, desc: Active camera's focus car index, unit: , type: 2, count: 1, count_as_time: false
name: CamCameraNumber, desc: Active camera number, unit: , type: 2, count: 1, count_as_time: false
name: CamGroupNumber, desc: Active camera group number, unit: , type: 2, count: 1, count_as_time: false
name: CamCameraState, desc: State of camera system, unit: irsdk_CameraState, type: 3, count: 1, count_as_time: false
name: IsOnTrackCar, desc: 1=Car on track physics running, unit: , type: 1, count: 1, count_as_time: false
name: IsInGarage, desc: 1=Car in garage physics running, unit: , type: 1, count: 1, count_as_time: false
name: SteeringWheelAngleMax, desc: Steering wheel max angle, unit: rad, type: 4, count: 1, count_as_time: false
name: ShiftPowerPct, desc: Friction torque applied to gears when shifting or grinding, unit: %, type: 4, count: 1, count_as_time: false
name: ShiftGrindRPM, desc: RPM of shifter grinding noise, unit: RPM, type: 4, count: 1, count_as_time: false
name: ThrottleRaw, desc: Raw throttle input 0=off throttle to 1=full throttle, unit: %, type: 4, count: 1, count_as_time: false
name: BrakeRaw, desc: Raw brake input 0=brake released to 1=max pedal force, unit: %, type: 4, count: 1, count_as_time: false
name: ClutchRaw, desc: Raw clutch input 0=disengaged to 1=fully engaged, unit: %, type: 4, count: 1, count_as_time: false
name: HandbrakeRaw, desc: Raw handbrake input 0=handbrake released to 1=max force, unit: %, type: 4, count: 1, count_as_time: false
name: BrakeABSactive, desc: true if abs is currently reducing brake force pressure, unit: , type: 1, count: 1, count_as_time: false
name: EngineWarnings, desc: Bitfield for warning lights, unit: irsdk_EngineWarnings, type: 3, count: 1, count_as_time: false
name: FuelLevelPct, desc: Percent fuel remaining, unit: %, type: 4, count: 1, count_as_time: false
name: PitSvFlags, desc: Bitfield of pit service checkboxes, unit: irsdk_PitSvFlags, type: 3, count: 1, count_as_time: false
name: PitSvLFP, desc: Pit service left front tire pressure, unit: kPa, type: 4, count: 1, count_as_time: false
name: PitSvRFP, desc: Pit service right front tire pressure, unit: kPa, type: 4, count: 1, count_as_time: false
name: PitSvLRP, desc: Pit service left rear tire pressure, unit: kPa, type: 4, count: 1, count_as_time: false
name: PitSvRRP, desc: Pit service right rear tire pressure, unit: kPa, type: 4, count: 1, count_as_time: false
name: PitSvFuel, desc: Pit service fuel add amount, unit: l or kWh, type: 4, count: 1, count_as_time: false
name: PitSvTireCompound, desc: Pit service pending tire compound, unit: , type: 2, count: 1, count_as_time: false
name: CarIdxP2P_Status, desc: Push2Pass active or not, unit: , type: 1, count: 64, count_as_time: false
name: CarIdxP2P_Count, desc: Push2Pass count of usage (or remaining in Race), unit: , type: 2, count: 64, count_as_time: false
name: SteeringWheelPctTorque, desc: Force feedback % max torque on steering shaft unsigned, unit: %, type: 4, count: 1, count_as_time: false
name: SteeringWheelPctTorqueSign, desc: Force feedback % max torque on steering shaft signed, unit: %, type: 4, count: 1, count_as_time: false
name: SteeringWheelPctTorqueSignStops, desc: Force feedback % max torque on steering shaft signed stops, unit: %, type: 4, count: 1, count_as_time: false
name: SteeringWheelPctSmoothing, desc: Force feedback % max smoothing, unit: %, type: 4, count: 1, count_as_time: false
name: SteeringWheelPctDamper, desc: Force feedback % max damping, unit: %, type: 4, count: 1, count_as_time: false
name: SteeringWheelLimiter, desc: Force feedback limiter strength limits impacts and oscillation, unit: %, type: 4, count: 1, count_as_time: false
name: SteeringWheelMaxForceNm, desc: Value of strength or max force slider in Nm for FFB, unit: N*m, type: 4, count: 1, count_as_time: false
name: SteeringWheelPeakForceNm, desc: Peak torque mapping to direct input units for FFB, unit: N*m, type: 4, count: 1, count_as_time: false
name: SteeringWheelUseLinear, desc: True if steering wheel force is using linear mode, unit: , type: 1, count: 1, count_as_time: false
name: ShiftIndicatorPct, desc: DEPRECATED use DriverCarSLBlinkRPM instead, unit: %, type: 4, count: 1, count_as_time: false
name: ReplayPlaySpeed, desc: Replay playback speed, unit: , type: 2, count: 1, count_as_time: false
name: ReplayPlaySlowMotion, desc: 0=not slow motion  1=replay is in slow motion, unit: , type: 1, count: 1, count_as_time: false
name: ReplaySessionTime, desc: Seconds since replay session start, unit: s, type: 5, count: 1, count_as_time: false
name: ReplaySessionNum, desc: Replay session number, unit: , type: 2, count: 1, count_as_time: false
name: TireLF_RumblePitch, desc: Players LF Tire Sound rumblestrip pitch, unit: Hz, type: 4, count: 1, count_as_time: false
name: TireRF_RumblePitch, desc: Players RF Tire Sound rumblestrip pitch, unit: Hz, type: 4, count: 1, count_as_time: false
name: TireLR_RumblePitch, desc: Players LR Tire Sound rumblestrip pitch, unit: Hz, type: 4, count: 1, count_as_time: false
name: TireRR_RumblePitch, desc: Players RR Tire Sound rumblestrip pitch, unit: Hz, type: 4, count: 1, count_as_time: false
name: IsGarageVisible, desc: 1=Garage screen is visible, unit: , type: 1, count: 1, count_as_time: false
name: SteeringWheelTorque_ST, desc: Output torque on steering shaft at 360 Hz, unit: N*m, type: 4, count: 6, count_as_time: true
name: SteeringWheelTorque, desc: Output torque on steering shaft, unit: N*m, type: 4, count: 1, count_as_time: false
name: VelocityZ_ST, desc: Z velocity, unit: m/s at 360 Hz, type: 4, count: 6, count_as_time: true
name: VelocityY_ST, desc: Y velocity, unit: m/s at 360 Hz, type: 4, count: 6, count_as_time: true
name: VelocityX_ST, desc: X velocity, unit: m/s at 360 Hz, type: 4, count: 6, count_as_time: true
name: VelocityZ, desc: Z velocity, unit: m/s, type: 4, count: 1, count_as_time: false
name: VelocityY, desc: Y velocity, unit: m/s, type: 4, count: 1, count_as_time: false
name: VelocityX, desc: X velocity, unit: m/s, type: 4, count: 1, count_as_time: false
name: YawRate_ST, desc: Yaw rate at 360 Hz, unit: rad/s, type: 4, count: 6, count_as_time: true
name: PitchRate_ST, desc: Pitch rate at 360 Hz, unit: rad/s, type: 4, count: 6, count_as_time: true
name: RollRate_ST, desc: Roll rate at 360 Hz, unit: rad/s, type: 4, count: 6, count_as_time: true
name: YawRate, desc: Yaw rate, unit: rad/s, type: 4, count: 1, count_as_time: false
name: PitchRate, desc: Pitch rate, unit: rad/s, type: 4, count: 1, count_as_time: false
name: RollRate, desc: Roll rate, unit: rad/s, type: 4, count: 1, count_as_time: false
name: VertAccel_ST, desc: Vertical acceleration (including gravity) at 360 Hz, unit: m/s^2, type: 4, count: 6, count_as_time: true
name: LatAccel_ST, desc: Lateral acceleration (including gravity) at 360 Hz, unit: m/s^2, type: 4, count: 6, count_as_time: true
name: LongAccel_ST, desc: Longitudinal acceleration (including gravity) at 360 Hz, unit: m/s^2, type: 4, count: 6, count_as_time: true
name: VertAccel, desc: Vertical acceleration (including gravity), unit: m/s^2, type: 4, count: 1, count_as_time: false
name: LatAccel, desc: Lateral acceleration (including gravity), unit: m/s^2, type: 4, count: 1, count_as_time: false
name: LongAccel, desc: Longitudinal acceleration (including gravity), unit: m/s^2, type: 4, count: 1, count_as_time: false
name: dcStarter, desc: In car trigger car starter, unit: , type: 1, count: 1, count_as_time: false
name: dcDashPage, desc: In car dash display page adjustment, unit: , type: 4, count: 1, count_as_time: false
name: dcTearOffVisor, desc: In car tear off visor film, unit: , type: 1, count: 1, count_as_time: false
name: dpTireChange, desc: Pitstop all tire change request, unit: , type: 4, count: 1, count_as_time: false
name: dpFuelFill, desc: Pitstop fuel fill flag, unit: , type: 4, count: 1, count_as_time: false
name: dpFuelAddKg, desc: Pitstop fuel add amount, unit: kg, type: 4, count: 1, count_as_time: false
name: dpFastRepair, desc: Pitstop fast repair set, unit: , type: 4, count: 1, count_as_time: false
name: dcBrakeBias, desc: In car brake bias adjustment, unit: , type: 4, count: 1, count_as_time: false
name: dpLFTireColdPress, desc: Pitstop lf tire cold pressure adjustment, unit: Pa, type: 4, count: 1, count_as_time: false
name: dpRFTireColdPress, desc: Pitstop rf cold tire pressure adjustment, unit: Pa, type: 4, count: 1, count_as_time: false
name: dpLRTireColdPress, desc: Pitstop lr tire cold pressure adjustment, unit: Pa, type: 4, count: 1, count_as_time: false
name: dpRRTireColdPress, desc: Pitstop rr cold tire pressure adjustment, unit: Pa, type: 4, count: 1, count_as_time: false
name: RFbrakeLinePress, desc: RF brake line pressure, unit: bar, type: 4, count: 1, count_as_time: false
name: RFcoldPressure, desc: RF tire cold pressure  as set in the garage, unit: kPa, type: 4, count: 1, count_as_time: false
name: RFtempCL, desc: RF tire left carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: RFtempCM, desc: RF tire middle carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: RFtempCR, desc: RF tire right carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: RFwearL, desc: RF tire left percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: RFwearM, desc: RF tire middle percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: RFwearR, desc: RF tire right percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: LFbrakeLinePress, desc: LF brake line pressure, unit: bar, type: 4, count: 1, count_as_time: false
name: LFcoldPressure, desc: LF tire cold pressure  as set in the garage, unit: kPa, type: 4, count: 1, count_as_time: false
name: LFtempCL, desc: LF tire left carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: LFtempCM, desc: LF tire middle carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: LFtempCR, desc: LF tire right carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: LFwearL, desc: LF tire left percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: LFwearM, desc: LF tire middle percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: LFwearR, desc: LF tire right percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: FuelUsePerHour, desc: Engine fuel used instantaneous, unit: kg/h, type: 4, count: 1, count_as_time: false
name: Voltage, desc: Engine voltage, unit: V, type: 4, count: 1, count_as_time: false
name: WaterTemp, desc: Engine coolant temp, unit: C, type: 4, count: 1, count_as_time: false
name: WaterLevel, desc: Engine coolant level, unit: l, type: 4, count: 1, count_as_time: false
name: FuelPress, desc: Engine fuel pressure, unit: bar, type: 4, count: 1, count_as_time: false
name: OilTemp, desc: Engine oil temperature, unit: C, type: 4, count: 1, count_as_time: false
name: OilPress, desc: Engine oil pressure, unit: bar, type: 4, count: 1, count_as_time: false
name: OilLevel, desc: Engine oil level, unit: l, type: 4, count: 1, count_as_time: false
name: ManifoldPress, desc: Engine manifold pressure, unit: bar, type: 4, count: 1, count_as_time: false
name: FuelLevel, desc: Liters of fuel remaining, unit: l, type: 4, count: 1, count_as_time: false
name: Engine0_RPM, desc: Engine0Engine rpm, unit: revs/min, type: 4, count: 1, count_as_time: false
name: RRbrakeLinePress, desc: RR brake line pressure, unit: bar, type: 4, count: 1, count_as_time: false
name: RRcoldPressure, desc: RR tire cold pressure  as set in the garage, unit: kPa, type: 4, count: 1, count_as_time: false
name: RRtempCL, desc: RR tire left carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: RRtempCM, desc: RR tire middle carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: RRtempCR, desc: RR tire right carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: RRwearL, desc: RR tire left percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: RRwearM, desc: RR tire middle percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: RRwearR, desc: RR tire right percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: LRbrakeLinePress, desc: LR brake line pressure, unit: bar, type: 4, count: 1, count_as_time: false
name: LRcoldPressure, desc: LR tire cold pressure  as set in the garage, unit: kPa, type: 4, count: 1, count_as_time: false
name: LRtempCL, desc: LR tire left carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: LRtempCM, desc: LR tire middle carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: LRtempCR, desc: LR tire right carcass temperature, unit: C, type: 4, count: 1, count_as_time: false
name: LRwearL, desc: LR tire left percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: LRwearM, desc: LR tire middle percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: LRwearR, desc: LR tire right percent tread remaining, unit: %, type: 4, count: 1, count_as_time: false
name: CRshockDefl, desc: CR shock deflection, unit: m, type: 4, count: 1, count_as_time: false
name: CRshockDefl_ST, desc: CR shock deflection at 360 Hz, unit: m, type: 4, count: 6, count_as_time: true
name: CRshockVel, desc: CR shock velocity, unit: m/s, type: 4, count: 1, count_as_time: false
name: CRshockVel_ST, desc: CR shock velocity at 360 Hz, unit: m/s, type: 4, count: 6, count_as_time: true
name: LRshockDefl, desc: LR shock deflection, unit: m, type: 4, count: 1, count_as_time: false
name: LRshockDefl_ST, desc: LR shock deflection at 360 Hz, unit: m, type: 4, count: 6, count_as_time: true
name: LRshockVel, desc: LR shock velocity, unit: m/s, type: 4, count: 1, count_as_time: false
name: LRshockVel_ST, desc: LR shock velocity at 360 Hz, unit: m/s, type: 4, count: 6, count_as_time: true
name: RRshockDefl, desc: RR shock deflection, unit: m, type: 4, count: 1, count_as_time: false
name: RRshockDefl_ST, desc: RR shock deflection at 360 Hz, unit: m, type: 4, count: 6, count_as_time: true
name: RRshockVel, desc: RR shock velocity, unit: m/s, type: 4, count: 1, count_as_time: false
name: RRshockVel_ST, desc: RR shock velocity at 360 Hz, unit: m/s, type: 4, count: 6, count_as_time: true
name: LFshockDefl, desc: LF shock deflection, unit: m, type: 4, count: 1, count_as_time: false
name: LFshockDefl_ST, desc: LF shock deflection at 360 Hz, unit: m, type: 4, count: 6, count_as_time: true
name: LFshockVel, desc: LF shock velocity, unit: m/s, type: 4, count: 1, count_as_time: false
name: LFshockVel_ST, desc: LF shock velocity at 360 Hz, unit: m/s, type: 4, count: 6, count_as_time: true
name: RFshockDefl, desc: RF shock deflection, unit: m, type: 4, count: 1, count_as_time: false
name: RFshockDefl_ST, desc: RF shock deflection at 360 Hz, unit: m, type: 4, count: 6, count_as_time: true
name: RFshockVel, desc: RF shock velocity, unit: m/s, type: 4, count: 1, count_as_time: false
name: RFshockVel_ST, desc: RF shock velocity at 360 Hz, unit: m/s, type: 4, count: 6, count_as_time: true
*/
