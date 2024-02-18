use std::{collections::HashMap, time::Instant};

use crate::{
    games::acc::data::{RealtimeCarUpdate, TrackData},
    model::{EntryId, Event},
    Time,
};

use super::{AccProcessor, AccProcessorContext};

#[derive(Default)]
pub struct GapToLeaderProcessor {
    markers: Vec<Vec<Marker>>,
    prev_marker_idx: HashMap<EntryId, usize>,
}
impl AccProcessor for GapToLeaderProcessor {
    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        if self.markers.is_empty() {
            // We dont have any track data yet so we dont know how many markers we have either.
            return Ok(());
        }
        let Some(entry) = context
            .model
            .current_session_mut()
            .and_then(|s| s.entries.get_mut(&EntryId(update.car_id as i32)))
        else {
            // A realtime update for a car that does not exsists is not an error directly.
            // The base processor should request a new entry list and then add it to the session.
            // We cannot continue though.
            return Ok(());
        };

        let lap = entry.distance_driven.floor() as u32;
        let marker_idx =
            (entry.distance_driven.fract() * self.markers.len() as f32).floor() as usize;

        // Check if this car has moved to a new marker.
        if self
            .prev_marker_idx
            .get(&entry.id)
            .is_some_and(|idx| *idx == marker_idx)
        {
            // This entry is still at the same marker as last update so there is no need to
            // update again.
            return Ok(());
        }
        self.prev_marker_idx.insert(entry.id, marker_idx);

        // Update the time behind leader.
        if let Some(marker) = self.markers[marker_idx].iter().find(|m| m.lap == lap) {
            entry.time_behind_leader.set(Time {
                ms: (Instant::now() - marker.time).as_millis() as f64,
            });
        } else {
            entry.time_behind_leader.set(Time { ms: 0.0 });
            // a marker for this lap does not exisist; create one.
            self.markers[marker_idx].push(Marker {
                lap,
                time: Instant::now(),
            });
        }

        Ok(())
    }
    fn track_data(
        &mut self,
        track: &TrackData,
        _context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        // Create marker container. Markers are spaced every 200 meters.
        let num_checkpoints = (track.track_meter / 200) as usize;
        self.markers.clear();
        for _ in 0..num_checkpoints {
            self.markers.push(Vec::new());
        }
        Ok(())
    }
    fn event(
        &mut self,
        event: &Event,
        _context: &mut AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        // Clear all known markers when the session changes.
        if let Event::SessionChanged(_) = event {
            self.markers.iter_mut().for_each(Vec::clear);
        }
        Ok(())
    }
}

struct Marker {
    lap: u32,
    time: Instant,
}
