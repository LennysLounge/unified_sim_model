use serde::{de::Visitor, Deserialize, Serialize};

use crate::{Angle, Distance, Speed, Temperature};

#[derive(Default, Clone)]
pub struct Data {
    pub session_data: SessionData,
    pub gear: i32,
    pub session_time: f64,
    pub car_idx_lap: Vec<i32>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct SessionData {
    #[serde(rename = "WeekendInfo")]
    pub weekend_info: WeekendInfo,
    #[serde(rename = "DriverInfo")]
    pub driver_info: DriverInfo,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct WeekendInfo {
    #[serde(rename = "TrackName")]
    pub track_name: String,
    #[serde(rename = "TrackID")]
    pub track_id: i32,
    #[serde(rename = "TrackLength")]
    #[serde(deserialize_with = "km_deserializer")]
    pub track_length: Distance,
    #[serde(rename = "TrackLengthOfficial")]
    #[serde(deserialize_with = "km_deserializer")]
    pub track_length_official: Distance,
    #[serde(rename = "TrackDisplayName")]
    pub track_display_name: String,
    #[serde(rename = "TrackDisplayShortName")]
    pub track_display_short_name: String,
    #[serde(rename = "TrackConfigName")]
    pub track_config_name: String,
    #[serde(rename = "TrackCity")]
    pub track_city: String,
    #[serde(rename = "TrackCountry")]
    pub track_country: String,
    #[serde(rename = "TrackAltitude")]
    #[serde(deserialize_with = "m_deserializer")]
    pub track_altitude: Distance,
    #[serde(rename = "TrackLatitude")]
    #[serde(deserialize_with = "decimal_degrees_deserializer")]
    pub track_latitude: Angle,
    #[serde(rename = "TrackLongitude")]
    #[serde(deserialize_with = "decimal_degrees_deserializer")]
    pub track_longitude: Angle,
    #[serde(rename = "TrackNorthOffset")]
    #[serde(deserialize_with = "rad_deserializer")]
    pub track_north_offset: Angle,
    #[serde(rename = "TrackNumTurns")]
    pub track_num_turns: i32,
    #[serde(rename = "TrackPitSpeedLimit")]
    #[serde(deserialize_with = "kph_deserializer")]
    pub track_pit_speed_limit: Speed,
    #[serde(rename = "TrackType")]
    pub track_type: String,
    #[serde(rename = "TrackDirection")]
    pub track_direction: String,
    #[serde(rename = "TrackWeatherType")]
    pub track_weather_type: String,
    #[serde(rename = "TrackSkies")]
    pub track_skies: String,
    #[serde(rename = "TrackSurfaceTemp")]
    #[serde(deserialize_with = "celcius_deserializer")]
    pub track_surface_temp: Temperature,
    #[serde(rename = "TrackAirTemp")]
    #[serde(deserialize_with = "celcius_deserializer")]
    pub track_air_temp: Temperature,
    #[serde(rename = "TrackAirPressure")]
    pub track_air_pressure: f32,
    #[serde(rename = "TrackWindVel")]
    #[serde(deserialize_with = "ms_deserializer")]
    pub track_wind_vel: Speed,
    #[serde(rename = "TrackWindDir")]
    #[serde(deserialize_with = "rad_deserializer")]
    pub track_wind_dir: Angle,
    #[serde(rename = "TrackRelativeHumidity")]
    pub track_relative_humidity: f32,
    #[serde(rename = "TrackFogLevel")]
    pub track_fog_level: f32,
    #[serde(rename = "TrackCleanup")]
    pub track_cleanup: i32,
    #[serde(rename = "TrackDynamicTrack")]
    pub track_dynamic_track: i32,
    #[serde(rename = "TrackVersion")]
    pub track_version: String,
    #[serde(rename = "SeriesID")]
    pub series_id: i32,
    #[serde(rename = "SeasonID")]
    pub season_id: i32,
    #[serde(rename = "SessionID")]
    pub session_id: i32,
    #[serde(rename = "SubSessionID")]
    pub sub_session_id: i32,
    #[serde(rename = "LeagueID")]
    pub league_id: i32,
    #[serde(rename = "Official")]
    pub official: i32,
    #[serde(rename = "RaceWeek")]
    pub race_week: i32,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "SimMode")]
    pub sim_mode: String,
    #[serde(rename = "TeamRacing")]
    pub team_racing: i32,
    #[serde(rename = "MinDrivers")]
    pub min_drivers: i32,
    #[serde(rename = "MaxDrivers")]
    pub max_drivers: i32,
    #[serde(rename = "DCRuleSet")]
    pub dc_rule_set: String,
    #[serde(rename = "QualifierMustStartRace")]
    pub qualifier_must_start_race: i32,
    #[serde(rename = "NumCarClasses")]
    pub num_car_classes: i32,
    #[serde(rename = "NumCarTypes")]
    pub num_car_type: i32,
    #[serde(rename = "HeatRacing")]
    pub heat_racing: i32,
    #[serde(rename = "BuildType")]
    pub build_type: String,
    #[serde(rename = "BuildTarget")]
    pub build_target: String,
    #[serde(rename = "BuildVersion")]
    pub build_version: String,
    #[serde(rename = "WeekendOptions")]
    pub weekend_options: WeekendOptions,
    #[serde(rename = "TelemetryOptions")]
    pub telemetry_options: TelemetryOptions,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct WeekendOptions {
    #[serde(rename = "NumStarters")]
    pub num_starters: i32,
    #[serde(rename = "StartingGrid")]
    pub starting_grid: String,
    #[serde(rename = "QualifyScoring")]
    pub qualify_scoring: String,
    #[serde(rename = "CourseCautions")]
    pub course_cautions: String,
    #[serde(rename = "StandingStart")]
    pub standing_start: i32,
    #[serde(rename = "ShortParadeLap")]
    pub short_parade_lap: i32,
    #[serde(rename = "Restarts")]
    pub restarts: String,
    #[serde(rename = "WeatherType")]
    pub weather_type: String,
    #[serde(rename = "Skies")]
    pub skies: String,
    #[serde(rename = "WindDirection")]
    pub wind_direction: String,
    #[serde(rename = "WindSpeed")]
    #[serde(deserialize_with = "kmh_deserializer")]
    pub wind_speed: Speed,
    #[serde(rename = "WeatherTemp")]
    #[serde(deserialize_with = "celcius_deserializer")]
    pub weather_temp: Temperature,
    #[serde(rename = "RelativeHumidity")]
    pub relative_humidity: f32,
    #[serde(rename = "FogLevel")]
    pub fog_level: f32,
    #[serde(rename = "TimeOfDay")]
    pub time_of_data: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "EarthRotationSpeedupFactor")]
    pub earth_rotation_speedup_factor: i32,
    #[serde(rename = "Unofficial")]
    pub unofficial: i32,
    #[serde(rename = "CommercialMode")]
    pub commercial_mode: String,
    #[serde(rename = "NightMode")]
    pub night_mode: String,
    #[serde(rename = "IsFixedSetup")]
    pub is_fixed_setup: i32,
    #[serde(rename = "StrictLapsChecking")]
    pub strict_laps_checking: String,
    #[serde(rename = "HasOpenRegistration")]
    pub has_open_registration: i32,
    #[serde(rename = "HardcoreLevel")]
    pub hardcore_level: i32,
    #[serde(rename = "NumJokerLaps")]
    pub num_joker_laps: i32,
    #[serde(rename = "IncidentLimit")]
    pub incident_limit: String,
    #[serde(rename = "FastRepairsLimit")]
    pub fast_repairs_limit: String,
    #[serde(rename = "GreenWhiteCheckeredLimit")]
    pub green_white_checkered_limit: i32,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TelemetryOptions {
    #[serde(rename = "TelemetryDiskFile")]
    pub telemetry_disk_file: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SessionInfo {}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CameraInfo {}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RadioInfo {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct DriverInfo {
    #[serde(rename = "DriverUserID")]
    pub driver_user_id: i32,
    #[serde(rename = "Drivers")]
    pub drivers: Vec<Driver>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Driver {
    #[serde(rename = "CarIdx")]
    pub car_index: i32,
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SplitTimeInfo {}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CarSetup {}

/// A visitor for deserializing values with units as string into a number
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

fn km_deserializer<'de, D>(deserializer: D) -> Result<Distance, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "km" })
        .map(|km| Distance::from_kilometers(km))
}

fn m_deserializer<'de, D>(deserializer: D) -> Result<Distance, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "m" })
        .map(|km| Distance::from_kilometers(km))
}

fn decimal_degrees_deserializer<'de, D>(deserializer: D) -> Result<Angle, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        // For some reason the latitude and longitude have m as their unit.
        .deserialize_str(UnitVisitor { unit: "m" })
        .map(|angle| Angle::from_deg(angle))
}

fn rad_deserializer<'de, D>(deserializer: D) -> Result<Angle, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "rad" })
        .map(|angle| Angle::from_rad(angle))
}

fn kph_deserializer<'de, D>(deserializer: D) -> Result<Speed, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "kph" })
        .map(|kmh| Speed::from_kmh(kmh))
}

fn kmh_deserializer<'de, D>(deserializer: D) -> Result<Speed, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "km/h" })
        .map(|kmh| Speed::from_kmh(kmh))
}

fn ms_deserializer<'de, D>(deserializer: D) -> Result<Speed, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "m/s" })
        .map(|ms| Speed::from_ms(ms))
}

fn celcius_deserializer<'de, D>(deserializer: D) -> Result<Temperature, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer
        .deserialize_str(UnitVisitor { unit: "C" })
        .map(|c| Temperature::from_celcius(c))
}

// fn unit_value_de<'de, D>(deserializer: D) -> Result<Value, D::Error>
// where
//     D: serde::de::Deserializer<'de>,
// {
//     deserializer.deserialize_str(UnitVisitor { unit: "dinglebobs" })
// }
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
