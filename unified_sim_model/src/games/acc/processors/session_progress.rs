use std::collections::HashMap;

use crate::{
    games::acc::data::RealtimeCarUpdate,
    model::{Entry, EntryId, Event, ScoringType, Session, SessionPhase},
};

use super::{AccProcessor, AccProcessorContext};

/// Processes the session progress for each entity in the session.
///
/// Processes these fields:
/// * [`distance_driven`]
/// * [`is_finished`]
#[derive(Default)]
pub struct SessionProgressProcessor {
    entries: HashMap<EntryId, EntryState>,
}
impl AccProcessor for SessionProgressProcessor {
    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        let entry_id = EntryId(update.car_id as i32);
        let entry_state = self
            .entries
            .entry(entry_id)
            .or_insert(EntryState::PreSession);

        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };

        match session.session_type.scoring_type() {
            ScoringType::BestLapTime => entry_state.best_lap(entry_id, session),
            ScoringType::DistanceThenTime => entry_state.distance_then_time(entry_id, session),
        }

        Ok(())
    }

    fn event(
        &mut self,
        event: &Event,
        _context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        if let Event::SessionChanged(_) = event {
            self.entries.clear();
        }

        Ok(())
    }
}

/// The state of an entry in the session.
enum EntryState {
    /// The entry is waiting for the session to start.
    PreSession,
    /// The session is active but this entry has not
    /// crossed the start finish line yet to start the race.
    ActiveButNotCrossedTheLine,
    /// The entry is active in the session.
    Active,
    /// The enrty is in the process of finishing the session.
    Ending,
    /// The entry has completed the session.
    Finished,
}

impl EntryState {
    fn best_lap(&mut self, entry_id: EntryId, session: &mut Session) {
        // For debugging
        if let Some(entry) = session.entries.get_mut(&entry_id) {
            entry.team_name.set(match self {
                EntryState::PreSession => String::from("PreSession"),
                EntryState::ActiveButNotCrossedTheLine => {
                    String::from("ActiveButNotCrossedTheLine")
                }
                EntryState::Active => String::from("Active"),
                EntryState::Ending => String::from("Ending"),
                EntryState::Finished => String::from("Finished"),
            });
        }

        let session_phase = *session.phase;
        let Some(entry) = session.entries.get_mut(&entry_id) else {
            return;
        };

        match session_phase {
            SessionPhase::None
            | SessionPhase::Waiting
            | SessionPhase::Preparing
            | SessionPhase::Formation => {
                *self = EntryState::PreSession;
                entry.distance_driven.set(0.0);
                entry.is_finished.set(false);
            }
            SessionPhase::Active => {
                *self = EntryState::Active;
                let distance = get_distance_driven(entry);
                entry.distance_driven.set(distance);
                entry.is_finished.set(false);
            }
            SessionPhase::Ending => {
                // All active entries to to the ending state.
                // If an entry finishes the lap, it is moved to the finished state where it will stay.
                match self {
                    EntryState::PreSession
                    | EntryState::ActiveButNotCrossedTheLine
                    | EntryState::Active => {
                        let distance = get_distance_driven(entry);
                        entry.distance_driven.set(distance);
                        entry.is_finished.set(false);
                        *self = EntryState::Ending;
                    }
                    EntryState::Ending => {
                        let new_distance = get_distance_driven(entry);
                        if entry.distance_driven.fract() > 0.95 && new_distance.fract() < 0.5 {
                            entry.distance_driven.set(new_distance.floor());
                            entry.is_finished.set(true);
                            *self = EntryState::Finished;
                        } else {
                            entry.distance_driven.set(new_distance);
                            entry.is_finished.set(false);
                        }
                    }
                    // Do nothing.
                    EntryState::Finished => (),
                }
            }
            SessionPhase::Finished => {
                // Move all entries that are not finished to the finished state.
                if !matches!(self, EntryState::Finished) {
                    *self = EntryState::Finished;
                    entry.is_finished.set(true);
                }
            }
        }
    }

    fn distance_then_time(&mut self, _entry_id: EntryId, _session: &mut Session) {}
}

fn get_distance_driven(entry: &mut Entry) -> f32 {
    let mut distance_driven = *entry.spline_pos + *entry.lap_count as f32;
    if (entry.spline_pos > 0.95 || entry.spline_pos < 0.05) && !*entry.in_pits {
        let diff_to_last_update = distance_driven - *entry.distance_driven;
        if diff_to_last_update < -0.5 {
            distance_driven += 1.0;
        }
        if diff_to_last_update > 0.5 {
            distance_driven -= 1.0;
        }
    }
    distance_driven
}
