use std::collections::HashMap;

pub mod event;
pub mod nationality;
pub mod session;
pub mod time;

pub use event::Event;
use indexmap::IndexMap;
pub use nationality::Nationality;
pub use session::*;
pub use time::Time;

/// The unified sim model.
#[derive(Debug, Default)]
pub struct Model {
    /// List of sessions that have happend during the event.
    /// Sessions are orderd in the order they occur in the event.
    pub sessions: IndexMap<SessionId, Session>,
    /// Index of the current active session.
    pub current_session: SessionId,
    /// Name of the event.
    pub event_name: String,
    /// Name of the track.
    pub track_name: String,
    /// Length of the track in meter.
    pub track_length: i32,
    /// List of events that have happened since the last update.
    pub events: Vec<Event>,
}

impl Model {
    /// Add a session to the model.
    /// Sets the id of the session and returns it.
    pub fn add_session(&mut self, mut session: Session) -> SessionId {
        let id = SessionId(self.sessions.len());
        session.id = id;
        self.sessions.insert(id, session);
        id
    }

    pub fn current_session(&self) -> Option<&Session> {
        self.sessions.get(&self.current_session)
    }

    pub fn current_session_mut(&mut self) -> Option<&mut Session> {
        self.sessions.get_mut(&self.current_session)
    }
}

/// An id for an entry.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntryId(pub i32);

/// A team entry in the session.
#[derive(Debug, Default)]
pub struct Entry {
    pub id: EntryId,
    pub drivers: HashMap<DriverId, Driver>,
    pub current_driver: DriverId,
    pub team_name: String,
    pub car: Car,
    pub car_number: i32,
    pub nationality: Nationality,
    pub world_pos: [f32; 3],
    pub orientation: [f32; 3],
    pub position: i32,
    pub spline_pos: f32,
    pub lap_count: i32,
    pub laps: Vec<Lap>,
    pub current_lap: Lap,
    pub best_lap: Option<usize>,
    pub performance_delta: Time,
    pub time_behind_leader: Time,
    pub in_pits: bool,
    pub gear: i32,
    pub speed: f32,
    pub connected: bool,
    pub stint_time: Time,
    pub distance_driven: f32,
}

/// An id for a driver.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DriverId(pub i32);

/// A Driver in a team.
#[derive(Debug, Default)]
pub struct Driver {
    pub id: DriverId,
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
    pub nationality: Nationality,
    pub driving_time: Time,
    pub best_lap: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Lap {
    pub time: Time,
    pub splits: Vec<Time>,
    pub driver_id: DriverId,
    pub entry_id: EntryId,
    pub invalid: bool,
}

impl Default for Lap {
    fn default() -> Self {
        Self {
            time: Time::from(i32::MAX),
            splits: Default::default(),
            driver_id: Default::default(),
            entry_id: Default::default(),
            invalid: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CarCategory {
    pub name: &'static str,
}

impl CarCategory {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Car {
    pub name: &'static str,
    pub manufacturer: &'static str,
    pub category: CarCategory,
}

impl Car {
    pub const fn new(
        name: &'static str,
        manufacturer: &'static str,
        category: CarCategory,
    ) -> Self {
        Self {
            name,
            manufacturer,
            category,
        }
    }
}
