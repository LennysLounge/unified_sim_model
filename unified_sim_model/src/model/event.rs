use super::{EntryId, Lap, SessionId, SessionPhase};

#[derive(Debug)]
pub enum Event {
    EntryConnected(EntryId),
    EntryReconnected(EntryId),
    EntryDisconnected(EntryId),
    SessionChanged(SessionId),
    SessionPhaseChanged(SessionId, SessionPhase),
    LapCompleted(LapCompleted),
}

#[derive(Debug)]
pub struct LapCompleted {
    pub lap: Lap,
    pub is_session_best: bool,
    pub is_entry_best: bool,
    pub is_driver_best: bool,
}