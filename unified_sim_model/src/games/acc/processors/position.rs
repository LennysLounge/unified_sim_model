use std::{cmp::Ordering, collections::HashMap, time::Instant};

use tracing::debug;

use crate::{
    games::acc::data::{RealtimeCarUpdate, SessionUpdate},
    model::{EntryId, Event, ScoringType, Session},
};

use super::{AccProcessor, AccProcessorContext};

/// Processes the position for each entry.
#[derive(Default)]
pub struct PositionProcessor {
    entries: HashMap<EntryId, PositionState>,
}
impl AccProcessor for PositionProcessor {
    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };
        let entry_id = EntryId(update.car_id as i32);
        let Some(entry) = session.entries.get_mut(&EntryId(update.car_id as i32)) else {
            return Ok(());
        };

        let position_state = self.entries.entry(entry_id).or_insert_with(|| {
            debug!(
                "Init position state for {entry_id:?} at position: {}",
                update.position
            );
            PositionState {
                last_pos: update.position as i32,
                distance: 0.0,
                finish_time: None,
            }
        });

        //entry.team_name.set(format!("{:?}", position_state));
        position_state.distance = *entry.distance_driven;
        if position_state.finish_time.is_none() && *entry.is_finished {
            position_state.finish_time = Some(Instant::now());
        }

        Ok(())
    }

    fn session_update(
        &mut self,
        _update: &SessionUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };

        match session.session_type.scoring_type() {
            ScoringType::BestLapTime => self.best_lap(session),
            ScoringType::DistanceThenTime => self.distance_then_time(session),
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
impl PositionProcessor {
    fn best_lap(&mut self, session: &mut Session) {
        let mut entries = session.entries.values_mut().collect::<Vec<_>>();
        entries.sort_by(|e1, e2| {
            let best_lap_time = match (e1.best_lap.as_ref(), e2.best_lap.as_ref()) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(t1), Some(t2)) => t1
                    .time
                    .ms
                    .partial_cmp(&t2.time.ms)
                    .unwrap_or(Ordering::Equal),
            };

            let Some(p1) = self.entries.get(&e1.id) else {
                return Ordering::Greater;
            };
            let Some(p2) = self.entries.get(&e2.id) else {
                return Ordering::Less;
            };

            best_lap_time.then(p1.cmp_last_pos(p2))
        });

        for (index, entry) in entries.into_iter().enumerate() {
            entry.position.set(index as i32 + 1);
            if let Some(position_state) = self.entries.get_mut(&entry.id) {
                position_state.last_pos = index as i32 + 1;
            }
        }
    }

    fn distance_then_time(&mut self, session: &mut Session) {
        let mut entries = session.entries.values_mut().collect::<Vec<_>>();
        entries.sort_by(|e1, e2| {
            let connected_or_finished =
                (*e2.connected || *e2.is_finished).cmp(&(*e1.connected || *e1.is_finished));

            let Some(p1) = self.entries.get(&e1.id) else {
                return Ordering::Greater;
            };
            let Some(p2) = self.entries.get(&e2.id) else {
                return Ordering::Less;
            };

            let position = p1.compare(p2);

            connected_or_finished.then(position)
        });

        for (index, entry) in entries.into_iter().enumerate() {
            entry.position.set(index as i32 + 1);

            if let Some(position_state) = self.entries.get_mut(&entry.id) {
                position_state.last_pos = index as i32 + 1;
            }
        }
    }
}

#[derive(Debug)]
struct PositionState {
    last_pos: i32,
    distance: f32,
    finish_time: Option<Instant>,
}
impl PositionState {
    fn cmp_distance(&self, other: &PositionState) -> Ordering {
        let diff = self.distance - other.distance;
        if diff.abs() < 0.001 {
            Ordering::Equal
        } else if diff > 0.0 {
            Ordering::Less
        } else if diff < 0.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
    fn cmp_finish_time(&self, other: &PositionState) -> Ordering {
        match (&self.finish_time, &other.finish_time) {
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(t1), Some(t2)) => t1.cmp(t2),
            (None, None) => Ordering::Equal,
        }
    }
    fn cmp_last_pos(&self, other: &PositionState) -> Ordering {
        self.last_pos.cmp(&other.last_pos)
    }
    fn compare(&self, other: &PositionState) -> Ordering {
        self.cmp_distance(other)
            .then(self.cmp_finish_time(other))
            .then(self.cmp_last_pos(other))
    }
}
