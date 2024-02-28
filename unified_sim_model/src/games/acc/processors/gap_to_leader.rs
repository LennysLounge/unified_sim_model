//! This processor tries to solve for the time behind leader and time behind position ahead
//! properties in the model.
//!
//! There are multiple different ways to achieve this, all with their own set of advantages
//! and disadvantages.
//!
//! This implementation uses a checkpoint pased approach to measure the time between the leader
//! reaching a checkpoing and any other entry reaching that checkpoint.
//! the `time_behind_position_ahead` property can be easily calculated using the `time_behind_leader`.
//!
//! The checkpoint update is run in the session update for all entries all at once. Doing it this way
//! technically introduces a one update delay to the value but i have not found a way to do it in
//! `RealtimeCarUpdate` without creating a bigger delay in a different position.
//! Doing the update for each car directly in `RealtimeCarUpdate` does not work very well since we dont
//! know who is leading the race at that point. Only after the `distance_driven` value is full updated for
//! all entries can we find the leader of the race. We cannot reliably use the position reported by the game
//! since that only updates every sector.
//!

use std::{collections::HashMap, time::Instant};

use crate::{
    games::acc::data::SessionUpdate,
    model::{Entry, EntryId, Event, ScoringType, Value},
    Time,
};

use super::{AccProcessor, AccProcessorContext};

pub enum GapToLeaderProcessor {
    BestLapTime(BestLapTime),
    DistanceThenTime(DistanceThenTime),
}
impl Default for GapToLeaderProcessor {
    fn default() -> Self {
        Self::BestLapTime(BestLapTime::default())
    }
}
impl AccProcessor for GapToLeaderProcessor {
    fn session_update(
        &mut self,
        update: &SessionUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        match self {
            GapToLeaderProcessor::BestLapTime(p) => p.session_update(update, context),
            GapToLeaderProcessor::DistanceThenTime(p) => p.session_update(update, context),
        }
    }

    fn event(
        &mut self,
        event: &Event,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        if let Event::SessionChanged(session_id) = event {
            let session = context
                .model
                .sessions
                .get(session_id)
                .expect("The session was just changed to this");
            match session.session_type.scoring_type() {
                ScoringType::BestLapTime => {
                    *self = GapToLeaderProcessor::BestLapTime(BestLapTime::default())
                }
                ScoringType::DistanceThenTime => {
                    *self = GapToLeaderProcessor::DistanceThenTime(DistanceThenTime::default())
                }
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct BestLapTime;
impl AccProcessor for BestLapTime {
    fn session_update(
        &mut self,
        _update: &SessionUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };

        let Some(session_best_lap_time) = session.best_lap.as_ref().as_ref().map(|lap| lap.time.ms)
        else {
            session.entries.values_mut().for_each(|e| {
                e.time_behind_leader = Value::default();
                e.time_behind_position_ahead = Value::default();
            });
            return Ok(());
        };

        let mut entries: Vec<&Entry> = session.entries.values().collect();
        entries.sort_by_key(|e| *e.position);
        let entries = entries.iter().map(|e| e.id).collect::<Vec<_>>();

        let mut prev_position_best_lap_time = session_best_lap_time;
        for entry_id in entries {
            let Some(entry) = session.entries.get_mut(&entry_id) else {
                continue;
            };
            if let Some(best_lap_time) = entry.best_lap.as_ref().as_ref().map(|lap| lap.time.ms) {
                entry
                    .time_behind_leader
                    .set((best_lap_time - session_best_lap_time).into());
                entry
                    .time_behind_position_ahead
                    .set((best_lap_time - prev_position_best_lap_time).into());
                prev_position_best_lap_time = best_lap_time;
            } else {
                entry.time_behind_leader = Value::default();
                entry.time_behind_position_ahead = Value::default();
            }
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct DistanceThenTime {
    first_time: HashMap<usize, Instant>,
    last_time: HashMap<usize, Instant>,
    sectors_positions: HashMap<EntryId, usize>,
}
impl AccProcessor for DistanceThenTime {
    fn session_update(
        &mut self,
        _update: &SessionUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };

        let mut entries: Vec<&Entry> = session.entries.values().collect();
        entries.sort_by_key(|e| *e.position);
        let entries = entries.iter().map(|e| e.id).collect::<Vec<_>>();

        let now = Instant::now();
        for entry_id in entries {
            let entry = session.entries.get_mut(&entry_id).unwrap();
            let sector_number = (*entry.distance_driven * session.track_length.as_meters() / 200.0)
                .floor() as usize;

            // Check if this entry has crossed the next sector line.
            if let Some(prev_sector_number) = self.sectors_positions.get(&entry_id) {
                if *prev_sector_number == sector_number {
                    continue;
                }
            } else {
                self.sectors_positions.insert(entry_id, sector_number);
                continue;
            }
            self.sectors_positions.insert(entry_id, sector_number);

            // calculate time to leader
            if let Some(leader_instant) = self.first_time.get(&sector_number) {
                let time_diff = now - *leader_instant;
                entry
                    .time_behind_leader
                    .set(Time::from_secs(time_diff.as_secs_f64()));
            } else {
                // insert if this entry is the leader
                if entry.position == 1 {
                    entry.time_behind_leader.set(Time::from(0));
                    self.first_time.insert(sector_number, now);
                } else {
                    entry.time_behind_leader = Value::default();
                }
            }

            // calculate time to position ahead.
            if let Some(ahead_instant) = self.last_time.get(&sector_number) {
                let time_diff = now - *ahead_instant;
                entry
                    .time_behind_position_ahead
                    .set(Time::from_secs(time_diff.as_secs_f64()));
            } else {
                entry.time_behind_position_ahead = Value::default();
            }
            self.last_time.insert(sector_number, now);
        }

        Ok(())
    }
}
