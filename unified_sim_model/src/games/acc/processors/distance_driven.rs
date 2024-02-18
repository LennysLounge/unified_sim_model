//! Calculate the distance driven for an entity.
//!
//! The general approach to this is to add the spline position to the lap count to
//! get the distance driven. However for ACC this has two problems.
//!
//! 1)
//! The point in the lap where the lap count is updated and where the spline position
//! wrap are not the same. The lap count might update before or after the spline position
//! wraps. This causes a jump forwards or backwards by one lap
//!
//! 2) A race starts behind the start finish line with 0 laps on the counter.
//! Crossing the start finish line would usually increment the lap counter but not if the
//! race has just started. As a result the spline pos wraps but the lap count stays the same.
//! The distance driven then goes up to one and back down to 0 for the start.
//!
//! This processor tries to solve both of these problems.
//!
//!
//! 1) This problem can be solved by defining an area around the start finish line
//! where big jumps (more than half a lap) are not allowed. Any detected jumps can easily
//! be filtered out.
//! Cars that return to garage within the area should not become a problem since the jump
//! from the start finish line to the bix boxes is generally less than half the track.
//! Cars that RTG from outside the area might cause a jump that is bigger than 0.5.
//! In that case the area should only effect cars that were inside the area the previous frame.
//!
//!
//! 2) This problem can be solved by a state machine that keeps track of the state of each entry.
//! At the beginning of a race, each entry is in a pre race state where the distance driven is always
//! set to zero.
//! An entry leaves the pre race state once the session is active and their spline position drops from
//! high (> 0.5) to low (<0.5).
//! Cars that RTG before the session goes active may jump to a low spline position and jump ahead once the
//! session goes active. As an additional check, cars must not be in the pits to leave the pre race state.
//! Once in the active state the distance is calculated like normal.
//!
//! For the case there the user enters an already active session there is really nothing we can do about that.
//! We just have to asume that the normal method to calculate the distance is correct.
//! For the slim change that the user enters right after the session goes active and some drivers are sill in the
//! pre race state some anomalies are expected. There really is nothing we can do about that
//! (yes we can but its an unreasonable amount of work). Those anomalies will last one lap at most anyway.

use std::collections::HashMap;

use crate::model::{EntryId, Event, Session, SessionPhase};

use super::{AccProcessor, AccProcessorContext};

#[derive(Default)]
pub struct DistanceDrivenProcessor {
    entries: HashMap<EntryId, EntryState>,
}
impl AccProcessor for DistanceDrivenProcessor {
    fn realtime_car_update(
        &mut self,
        update: &crate::games::acc::data::RealtimeCarUpdate,
        context: &mut super::AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };
        let session_active = is_session_active(session);

        let entry_id = EntryId(update.car_id as i32);
        let entry_state = self.entries.entry(entry_id).or_insert_with(|| {
            if session_active {
                EntryState::Active
            } else {
                EntryState::PreRace
            }
        });

        match entry_state {
            EntryState::PreRace => {
                // Solve problem (2)
                if let Some(entry) = session.entries.get_mut(&entry_id) {
                    entry.distance_driven.set(0.0);
                    if entry.spline_pos < 0.5 && session_active && !entry.in_pits.as_ref() {
                        *entry_state = EntryState::Active;
                    }
                }
            }
            EntryState::Active => {
                if let Some(entry) = session.entries.get_mut(&entry_id) {
                    let mut distance_driven = *entry.spline_pos + *entry.lap_count as f32;

                    // Solve problem (1)
                    if (entry.spline_pos > 0.95 || entry.spline_pos < 0.05) && !*entry.in_pits {
                        let diff_to_last_update = distance_driven - *entry.distance_driven;
                        if diff_to_last_update < -0.5 {
                            distance_driven += 1.0;
                        }
                        if diff_to_last_update > 0.5 {
                            distance_driven -= 1.0;
                        }
                    }
                    entry.distance_driven.set(distance_driven);
                }
            }
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

enum EntryState {
    PreRace,
    Active,
}

fn is_session_active(session: &Session) -> bool {
    match session.phase.as_ref() {
        SessionPhase::None
        | SessionPhase::Waiting
        | SessionPhase::Preparing
        | SessionPhase::Formation => false,
        SessionPhase::Active | SessionPhase::Ending | SessionPhase::Finished => true,
    }
}
