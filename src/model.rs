use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Model {
    pub sessions: Vec<Session>,
    pub current_session: usize,
    pub event_name: String,
    pub track_name: String,
    pub track_length: i32,
}

#[derive(Debug, Default)]
pub enum SessionType {
    #[default]
    None,
}

#[derive(Debug, Default)]
pub enum SessionPhase {
    #[default]
    None,
}

#[derive(Debug, Default)]
pub enum SessionDay {
    #[default]
    Sunday,
}

#[derive(Debug, Default)]
pub struct Session {
    pub id: i32,
    pub entries: HashMap<i32, Entry>,
    pub session_type: SessionType,
    pub session_time: i32,
    pub time_remaining: i32,
    pub laps: i32,
    pub laps_remaining: i32,
    pub phase: SessionPhase,
    pub time_of_day: i32,
    pub day: SessionDay,
    pub ambient_temp: f32,
    pub track_temp: f32,
}

#[derive(Debug, Default)]
pub enum CarModel {
    #[default]
    None,
}

#[derive(Debug, Default)]
pub enum Nationality {
    #[default]
    None,
}

#[derive(Debug, Default)]
pub struct Entry {
    pub id: i32,
    pub driver: HashMap<i32, Driver>,
    pub current_driver: i32,
    pub team_name: String,
    pub car: CarModel,
    pub car_number: i32,
    pub nationality: Nationality,
    pub world_pos: [f32; 3],
    pub orientation: [f32; 3],
    pub position: i32,
    pub spline_pos: f32,
    pub laps: Vec<Lap>,
    pub current_lap: Lap,
    pub best_lap: usize,
    pub performance_delta: i32,
    pub time_behind_leader: i32,
    pub in_pits: bool,
    pub gear: i32,
    pub speed: f32,
    pub connected: bool,
    pub stint_time: i32,
}

#[derive(Debug, Default)]
pub struct Driver {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
    pub nationality: Nationality,
    pub driving_time: i32,
}

#[derive(Debug, Default)]
pub struct Lap {
    pub time: i32,
    pub splits: Vec<i32>,
    pub driver_id: i32,
    pub invalid: bool,
}
