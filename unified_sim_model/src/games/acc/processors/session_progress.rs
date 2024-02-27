//! This module deals with calculating the distance driven and the is_finished state
//! for each entry.
//!
//! The distance driven is simply calculated by adding the spline position to
//! the lap count. However, ACC has a bunch of inconsistencies that are unwanted and
//! need to be fixed.
//!
//! This implementation tries to do this by keeping track of the current state for each
//! entry and adjusting the calcultion base don the current state.
//! This way we can hopefully make sure that these fields work as intended.

use std::{cmp::Ordering, collections::HashMap};

use tracing::debug;

use crate::{
    games::acc::data::{RealtimeCarUpdate, SessionUpdate},
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
    /// If the user joins a session that is already in the ending state, it is
    /// impossible to know if the leader has already finished the race or not.
    /// In that case, all entries should immediately be considers finished even
    /// if that might finish some entries that have not actually finished.
    /// A regular session is one where this error does not happen.
    is_regular_session: bool,
}
impl AccProcessor for SessionProgressProcessor {
    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };

        let entry_id = EntryId(update.car_id as i32);
        if !session.entries.contains_key(&entry_id) {
            debug!("Got entry update for car that doesnt exists in session yet");
            return Ok(());
        }

        let entry_state = self.entries.entry(entry_id).or_insert_with(|| {
            let state = match *session.phase {
                SessionPhase::None
                | SessionPhase::Waiting
                | SessionPhase::Preparing
                | SessionPhase::Formation => EntryState::PreSession,
                SessionPhase::Active => EntryState::Active,
                SessionPhase::Ending => EntryState::Ending,
                SessionPhase::Finished => EntryState::Finished,
            };
            debug!("Insert entry state for entry {entry_id:?} with state: {state:?}");
            state
        });

        match session.session_type.scoring_type() {
            ScoringType::BestLapTime => entry_state.best_lap(entry_id, session),
            ScoringType::DistanceThenTime => entry_state.distance_then_time(entry_id, session),
        }

        Ok(())
    }

    fn session_update(
        &mut self,
        _update: &SessionUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        // Resolve the potentially finished state for entries.
        // Sort them by distance driven and if the leader is potentially finished then they become
        // an actuall finisher. all other entries are returned to the ending state.
        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };

        // If this processor is new and the user joins a session that is already in the ending state
        // we are not in a regular session.
        if session.phase != SessionPhase::Ending {
            self.is_regular_session = true;
        }
        // Irregular sessions end all entries instantly in the ending phase.
        if !self.is_regular_session && session.phase == SessionPhase::Ending {
            for entry_id in session.entries.keys() {
                if let Some(entry_state) = self.entries.get_mut(entry_id) {
                    *entry_state = EntryState::Finished;
                }
            }
        }

        let mut entries = session.entries.values().collect::<Vec<_>>();
        entries.sort_by(|e1, e2| {
            e2.distance_driven
                .partial_cmp(&e1.distance_driven)
                .unwrap_or(Ordering::Equal)
        });

        let has_first_just_place_finished = entries
            .first()
            .and_then(|first| self.entries.get(&first.id))
            .is_some_and(|first_state| matches!(first_state, EntryState::PotentialyFinished));
        if has_first_just_place_finished {
            // In case of a tie we want to set all entries with the same distance driven to finished
            // everyone else goes back to ending.
            if let Some(finished_distance) = entries.first().map(|first| *first.distance_driven) {
                for entry in entries {
                    if let Some(entry_state) = self.entries.get_mut(&entry.id) {
                        if matches!(entry_state, EntryState::PotentialyFinished) {
                            if entry.distance_driven >= finished_distance {
                                *entry_state = EntryState::Finished;
                            } else {
                                *entry_state = EntryState::Ending;
                            }
                        }
                    }
                }
            }
        } else {
            let has_first_place_finished = entries
                .first()
                .and_then(|first| self.entries.get(&first.id))
                .is_some_and(|first_state| matches!(first_state, EntryState::Finished));

            // Set all potential finishers to finished if the first place has also finished.
            for entry in entries {
                if let Some(entry_state) = self.entries.get_mut(&entry.id) {
                    if matches!(entry_state, EntryState::PotentialyFinished) {
                        if has_first_place_finished {
                            *entry_state = EntryState::Finished;
                        } else {
                            *entry_state = EntryState::Ending;
                        }
                    }
                }
            }
        }

        {}

        Ok(())
    }

    fn event(
        &mut self,
        event: &Event,
        _context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        if let Event::SessionChanged(_) = event {
            self.entries.clear();
            self.is_regular_session = true;
        }

        Ok(())
    }
}

/// The state of an entry in the session.
#[derive(Debug)]
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
    /// This entry might have finished the session but we are not sure yet.
    PotentialyFinished,
    /// The entry has completed the session.
    Finished,
}

impl EntryState {
    fn best_lap(&mut self, entry_id: EntryId, session: &mut Session) {
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
                        *self = EntryState::Ending;
                        self.best_lap(entry_id, session);
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
                    EntryState::PotentialyFinished => unreachable!("For a best lap session an entry cannot be in the potentially finished state"),
                    // Do nothing
                    EntryState::Finished => {
                        entry.is_finished.set(true);
                    },
                }
            }
            SessionPhase::Finished => {
                // Move all entries that are not finished to the finished state.
                *self = EntryState::Finished;
                entry.is_finished.set(true);
            }
        }
    }

    fn distance_then_time(&mut self, entry_id: EntryId, session: &mut Session) {
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
            SessionPhase::Active => match self {
                EntryState::PreSession => {
                    *self = EntryState::ActiveButNotCrossedTheLine;
                    self.distance_then_time(entry_id, session);
                }
                EntryState::ActiveButNotCrossedTheLine => {
                    if entry.spline_pos < 0.5 && !entry.in_pits.as_ref() {
                        *self = EntryState::Active;
                        self.distance_then_time(entry_id, session);
                    } else {
                        entry.distance_driven.set(0.0);
                        entry.is_finished.set(false);
                    }
                }
                EntryState::Active => {
                    let new_distance = get_distance_driven(entry);
                    entry.distance_driven.set(new_distance);
                    entry.is_finished.set(false);
                }
                EntryState::Ending | EntryState::PotentialyFinished | EntryState::Finished => {
                    unreachable!("No entry should be in the ending or finished state at this point")
                }
            },
            SessionPhase::Ending => match self {
                EntryState::PreSession
                | EntryState::ActiveButNotCrossedTheLine
                | EntryState::Active => {
                    *self = EntryState::Ending;
                    self.distance_then_time(entry_id, session);
                }
                EntryState::Ending => {
                    let new_distance = get_distance_driven(entry);
                    if entry.distance_driven.fract() > 0.95 && new_distance.fract() < 0.5 {
                        // We cannot be sure that this entry has finished the race since the leader
                        // of the race has to finish before any other entry. Therefore
                        // this entry becomes potentially finished until we can figure that out.
                        entry.distance_driven.set(new_distance.floor());
                        entry.is_finished.set(false);
                        *self = EntryState::PotentialyFinished;
                    } else {
                        entry.distance_driven.set(new_distance);
                        entry.is_finished.set(false);
                    }
                }
                EntryState::PotentialyFinished => {
                    unreachable!("The potentially finished state should have been resolved by now")
                }
                // Do nothing.
                EntryState::Finished => {
                    entry.is_finished.set(true);
                }
            },
            // Do nothing.
            SessionPhase::Finished => {
                *self = EntryState::Finished;
                entry.is_finished.set(true);
            }
        }
    }
}

fn get_distance_driven(entry: &mut Entry) -> f32 {
    let mut distance_driven = *entry.spline_pos + *entry.lap_count as f32;
    if entry.distance_driven.fract() > 0.95 || entry.distance_driven.fract() < 0.05 {
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
