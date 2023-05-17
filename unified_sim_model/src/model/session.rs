use std::collections::HashMap;

use super::{Entry, EntryId, Lap, Time};

/// A session id.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// A practice session scored by best lap time.
    Practice,
    /// A qualifying session scored by best lap time.
    Qualifying,
    /// A Race session, scored by furthest distance.
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
