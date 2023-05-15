use super::EntryId;

#[derive(Debug)]
pub enum Event {
    EntryConnected(EntryId),
    EntryReconnected(EntryId),
    EntryDisconnected(EntryId),
    SessionChanged,
    SessionPhaseChanged,
    LapCompleted,
}
