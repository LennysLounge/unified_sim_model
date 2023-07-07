use std::collections::{BTreeMap, HashMap};

use regex::Regex;
use serde::{de::Visitor, Deserialize, Serialize};
use serde_value::Value;

use crate::{Angle, Distance, Pressure, Speed, Temperature, Time, Weight};

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StaticData {
    /// Incremented when an update ocures.
    #[serde(skip)]
    pub update_count: i32,
    pub missing_field: Option<i32>,
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

impl StaticData {
    pub fn get_unmapped(&self) -> BTreeMap<Value, Value> {
        let prefix = "StaticData.".to_owned();
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
    #[serde(deserialize_with = "time_of_day_deserializer")]
    pub time_of_day: Option<Time>,
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
    #[serde(deserialize_with = "unlimited_i32_deserializer")]
    pub incident_limit: Option<MaybeUnlimited<i32>>,
    #[serde(deserialize_with = "unlimited_i32_deserializer")]
    pub fast_repairs_limit: Option<MaybeUnlimited<i32>>,
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
    #[serde(deserialize_with = "unlimited_i32_deserializer")]
    pub session_laps: Option<MaybeUnlimited<i32>>,
    #[serde(deserialize_with = "unlimited_sec_deserializer")]
    pub session_time: Option<MaybeUnlimited<Time>>,
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

#[derive(Debug, Clone)]
pub enum MaybeUnlimited<T> {
    Unlimited,
    Value(T),
}

struct UnlimitedOrUnitVisitor {
    unit: &'static str,
}

impl<'de> Visitor<'de> for UnlimitedOrUnitVisitor {
    type Value = MaybeUnlimited<f32>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a value with the unit '{}' or 'unlimited'",
            self.unit
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == "unlimited" {
            return Ok(MaybeUnlimited::Unlimited);
        }
        if !v.ends_with(self.unit) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            ));
        }
        let value_str = v.trim_end_matches(self.unit).trim_end();
        Ok(MaybeUnlimited::Value(
            str::parse(value_str.trim()).map_err(|e| serde::de::Error::custom(e))?,
        ))
    }
}

fn unlimited_sec_deserializer<'de, D>(
    deserializer: D,
) -> Result<Option<MaybeUnlimited<Time>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnlimitedOrUnitVisitor { unit: "sec" })
        .map(|v| match v {
            MaybeUnlimited::Unlimited => Some(MaybeUnlimited::Unlimited),
            MaybeUnlimited::Value(t) => Some(MaybeUnlimited::Value(Time::from_secs(t))),
        })
}

fn unlimited_i32_deserializer<'de, D>(
    deserializer: D,
) -> Result<Option<MaybeUnlimited<i32>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnlimitedOrUnitVisitor { unit: "" })
        .map(|v| match v {
            MaybeUnlimited::Unlimited => Some(MaybeUnlimited::Unlimited),
            MaybeUnlimited::Value(v) => Some(MaybeUnlimited::Value(v as i32)),
        })
}

struct TimeOfDayVisitor;
impl<'de> Visitor<'de> for TimeOfDayVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a time in the format 'h:mm am/pm'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let time_of_day_regex = Regex::new(r"(?<hour>\d+):(?<minutes>\d{2}) (?<morning>am|pm)")
            .expect("Should be a valid regex");

        let invalid_value =
            || serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self);

        let Some(capture) = time_of_day_regex.captures(v) else {
            return Err(invalid_value());
        };
        let hour = capture
            .name("hour")
            .ok_or(invalid_value())
            .and_then(|hour_str| {
                str::parse::<i32>(hour_str.as_str()).map_err(|_| invalid_value())
            })?;
        let minutes = capture
            .name("minutes")
            .ok_or(invalid_value())
            .and_then(|hour_str| {
                str::parse::<i32>(hour_str.as_str()).map_err(|_| invalid_value())
            })?;
        let am = capture
            .name("morning")
            .map(|morning_str| morning_str.as_str() == "am")
            .ok_or(invalid_value())?;

        Ok(Time::from_secs(
            hour * 360 + minutes * 60 + if am { 0 } else { 43200 },
        ))
    }
}

fn time_of_day_deserializer<'de, D>(deserializer: D) -> Result<Option<Time>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(TimeOfDayVisitor)
        .map(|t| Some(t))
}
