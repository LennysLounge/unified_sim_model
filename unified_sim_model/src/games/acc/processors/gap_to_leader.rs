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

use crate::{
    games::acc::data::SessionUpdate,
    model::{Entry, Event, ScoringType, Value},
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
// fn get_lap_time_diff(session: &Session, entry_id: &EntryId) -> Option<Time> {
//     let Some(first_place_best_lap) = session
//         .entries
//         .values()
//         .min_by(|a, b| a.position.cmp(&b.position))
//         .map(|first| first.best_lap.as_ref().as_ref())
//         .flatten()
//     else {
//         return None;
//     };

//     let Some(this_car_best_lap) = session
//         .entries
//         .get(entry_id)
//         .map(|e| e.best_lap.as_ref().as_ref())
//         .flatten()
//     else {
//         return None;
//     };

//     let laptime_diff = this_car_best_lap.time.ms - first_place_best_lap.time.ms;
//     Some(laptime_diff.into())
// }

#[derive(Default)]
pub struct DistanceThenTime;
impl AccProcessor for DistanceThenTime {}

// #[derive(Default)]
// pub struct GapToLeaderProcessor {
//     markers: Vec<Vec<Marker>>,
//     prev_marker_idx: HashMap<EntryId, usize>,
// }
// impl AccProcessor for GapToLeaderProcessor {
//     fn session_update(
//         &mut self,
//         _update: &SessionUpdate,
//         context: &mut AccProcessorContext,
//     ) -> crate::games::acc::Result<()> {
//         let Some(session) = context.model.current_session_mut() else {
//             return Ok(());
//         };

//         let mut entries: Vec<&Entry> = session.entries.values().collect();
//         entries.sort_by_key(|e| *e.position);
//         let entries = entries.iter().map(|e| e.id).collect::<Vec<_>>();

//         for (index, entry_id) in entries.iter().enumerate() {
//             match session.session_type.scoring_type() {
//                 ScoringType::BestLapTime => self.qualifying(*entry_id, session),
//                 ScoringType::DistanceThenTime => self.race(*entry_id, session, index == 0),
//             }
//         }

//         let mut prev_gap = 0.0;
//         for entry_id in entries.iter() {
//             if let Some(entry) = session.entries.get_mut(&entry_id) {
//                 if entry.time_behind_leader.is_avaliable() {
//                     let time = entry.time_behind_leader.ms - prev_gap;
//                     if time < 0.0 {
//                         entry.time_behind_position_ahead = Value::default();
//                     } else {
//                         entry.time_behind_position_ahead.set(time.into());
//                     }
//                     prev_gap = entry.time_behind_leader.ms;
//                 } else {
//                     entry.time_behind_position_ahead = Value::default();
//                 }
//             }
//         }

//         Ok(())
//     }
//     fn track_data(
//         &mut self,
//         track: &TrackData,
//         _context: &mut AccProcessorContext,
//     ) -> crate::games::acc::Result<()> {
//         // Create marker container. Markers are spaced every 200 meters.
//         let num_checkpoints = (track.track_meter / 200) as usize;
//         self.markers.clear();
//         for _ in 0..num_checkpoints {
//             self.markers.push(Vec::new());
//         }
//         Ok(())
//     }
//     fn event(
//         &mut self,
//         event: &Event,
//         _context: &mut AccProcessorContext,
//     ) -> crate::games::acc::Result<()> {
//         // Clear all known markers when the session changes.
//         if let Event::SessionChanged(_) = event {
//             self.markers.iter_mut().for_each(Vec::clear);
//         }
//         Ok(())
//     }
// }
// impl GapToLeaderProcessor {
//     fn race(&mut self, entry_id: EntryId, session: &mut Session, is_leader: bool) {
//         if self.markers.is_empty() {
//             // We dont have any track data yet so we dont know how many markers we have either.
//             return;
//         }
//         let Some(entry) = session.entries.get_mut(&entry_id) else {
//             // A realtime update for a car that does not exsists is not an error directly.
//             // The base processor should request a new entry list and then add it to the session.
//             // We cannot continue though.
//             return;
//         };

//         let lap = entry.distance_driven.floor() as u32;
//         let marker_idx =
//             (entry.distance_driven.fract() * self.markers.len() as f32).floor() as usize;

//         // Check if this car has moved to a new marker.
//         if !self.prev_marker_idx.contains_key(&entry.id) {
//             self.prev_marker_idx.insert(entry.id, marker_idx);
//         }
//         if self
//             .prev_marker_idx
//             .get(&entry.id)
//             .is_some_and(|idx| *idx == marker_idx)
//         {
//             // This entry is still at the same marker as last update so there is no need to
//             // update again.
//             return;
//         }
//         self.prev_marker_idx.insert(entry.id, marker_idx);

//         // Update the time behind leader.
//         if let Some(marker) = self.markers[marker_idx].iter().find(|m| m.lap == lap) {
//             entry.time_behind_leader.set(Time {
//                 ms: (Instant::now() - marker.time).as_millis() as f64,
//             });
//         } else if is_leader {
//             // a marker for this lap does not exisist; create one.
//             entry.time_behind_leader.set(Time { ms: 0.0 });
//             self.markers[marker_idx].push(Marker {
//                 lap,
//                 time: Instant::now(),
//             });
//         }
//     }

//     fn qualifying(&mut self, entry_id: EntryId, session: &mut Session) {
//         let gap_to_leader = Self::get_lap_time_diff(session, entry_id);
//         if let Some(entry) = session.entries.get_mut(&entry_id) {
//             if let Some(gap_to_leader) = gap_to_leader {
//                 entry.time_behind_leader.set(gap_to_leader);
//             } else {
//                 entry.time_behind_leader = Value::default();
//             }
//         }
//     }

//     fn get_lap_time_diff(session: &Session, entry_id: EntryId) -> Option<Time> {
//         let Some(first_place_best_lap) = session
//             .entries
//             .values()
//             .min_by(|a, b| a.position.cmp(&b.position))
//             .map(|first| first.best_lap.as_ref().as_ref())
//             .flatten()
//         else {
//             return None;
//         };

//         let Some(this_car_best_lap) = session
//             .entries
//             .get(&entry_id)
//             .map(|e| e.best_lap.as_ref().as_ref())
//             .flatten()
//         else {
//             return None;
//         };

//         let laptime_diff = this_car_best_lap.time.ms - first_place_best_lap.time.ms;
//         Some(laptime_diff.into())
//     }
// }

// struct Marker {
//     lap: u32,
//     time: Instant,
// }
