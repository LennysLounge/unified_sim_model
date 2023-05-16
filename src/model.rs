use std::{
    collections::HashMap,
    slice::{Iter, IterMut},
};

pub mod nationality;
pub use nationality::Nationality;

pub mod time;
pub use time::Time;

pub mod event;
pub use event::Event;

/// The unified sim model.
#[derive(Debug, Default)]
pub struct Model {
    /// List of sessions that have happend during the event.
    /// Sessions are orderd in the order they occur in the event.
    sessions: Vec<Session>,
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
        self.sessions.push(session);
        id
    }

    pub fn current_session(&self) -> Option<&Session> {
        self.sessions.get(self.current_session.0)
    }

    pub fn current_session_mut(&mut self) -> Option<&mut Session> {
        self.sessions.get_mut(self.current_session.0)
    }

    pub fn get_session(&self, id: &SessionId) -> Option<&Session> {
        self.sessions.get(id.0)
    }

    pub fn get_session_mut(&mut self, id: &SessionId) -> Option<&mut Session> {
        self.sessions.get_mut(id.0)
    }

    pub fn get_sessions(&self) -> Iter<Session> {
        self.sessions.iter()
    }

    pub fn get_sessions_mut(&mut self) -> IterMut<Session> {
        self.sessions.iter_mut()
    }
}

/// A session id.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SessionId(pub usize);

/// A session.
#[derive(Debug, Default)]
pub struct Session {
    pub id: SessionId,
    pub entries: HashMap<EntryId, Entry>,
    pub session_type: SessionType,
    pub session_time: Time,
    pub time_remaining: Time,
    pub laps: i32,
    pub laps_remaining: i32,
    pub phase: SessionPhase,
    pub time_of_day: Time,
    pub day: Day,
    pub ambient_temp: f32,
    pub track_temp: f32,
    pub best_lap: Lap,
}

/// The type of the session.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum SessionType {
    Practice,
    Qualifying,
    Race,
    /// Session type is unknown or unavailable.
    #[default]
    None,
}

/// The phase of the current session.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SessionPhase {
    /// The session phase is unknown or unavailable
    #[default]
    None,
    /// The session is waiting to start while a different session is active.
    Waiting,
    /// The session is preparing to start.
    /// Drivers and teams are getting ready.
    Preparing,
    /// The session is forming befor the start.
    /// This is mostly in form of a formation lap.
    Formation,
    /// The session is active and running.
    Active,
    /// The session is ending. The end condition for the session has been met
    /// (either lap count reached or timer expired etc.) and the session is waiting
    /// for all drivers to finish the session.
    Ending,
    /// The session is finished. All drivers have finished their session and the
    /// results of the session are finalised.
    Finished,
}

impl SessionPhase {
    /// Returns the next phase in order.
    /// Once session is in the finished state it does not advance further.
    pub fn next(&self) -> Self {
        use SessionPhase::*;
        match self {
            None => Waiting,
            Waiting => Preparing,
            Preparing => Formation,
            Formation => Active,
            Active => Ending,
            Ending => Finished,
            Finished => Finished,
        }
    }
}

/// Describes the day a session takes part in.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Day {
    Monday,
    Thuesday,
    Wednesday,
    Thrusday,
    Friday,
    Saturday,
    #[default]
    Sunday,
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
    pub laps: Vec<Lap>,
    pub current_lap: Lap,
    pub best_lap: usize,
    pub performance_delta: Time,
    pub time_behind_leader: Time,
    pub in_pits: bool,
    pub gear: i32,
    pub speed: f32,
    pub connected: bool,
    pub stint_time: Time,
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
    pub best_lap: usize,
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
