use std::collections::HashMap;

use tracing::warn;

use crate::{
    games::iracing::IRacingResult,
    model::{EntryId, Event},
};

use super::{IRacingProcessor, IRacingProcessorContext};

pub struct SpeedProcessor {
    last_update_number: i32,
    entries: HashMap<EntryId, LastData>,
}

#[derive(Default)]
struct LastData {
    lap_dist: f32,
    speed: f32,
}

impl SpeedProcessor {
    pub fn new() -> Self {
        Self {
            last_update_number: 0,
            entries: HashMap::new(),
        }
    }
}

impl IRacingProcessor for SpeedProcessor {
    fn static_data(&mut self, _context: &mut IRacingProcessorContext) -> IRacingResult<()> {
        Ok(())
    }

    fn live_data(&mut self, context: &mut IRacingProcessorContext) -> IRacingResult<()> {
        let Some(track_length) = context.data.static_data.weekend_info.track_length else {
            warn!("No track length available");
            return Ok(());
        };
        let Some(update_number) = context.data.live_data.session_tick else {
            warn!("No session tick available");
            return Ok(());
        };

        let time_between = (update_number - self.last_update_number) as f32 / 60.0;

        context.model.current_session_mut().map(|s| {
            s.entries.iter_mut().for_each(|(entry_id, entry)| {
                let last_data = self.entries.remove(entry_id).unwrap_or_default();

                let distance_driven =
                    (entry.spline_pos.as_ref() - last_data.lap_dist) * track_length.as_meters();
                let mut speed = distance_driven / time_between;

                // 120 m/s is roughly 430 kph. No car can reasonable travel that fast.
                if speed.abs() > 120.0 {
                    speed = last_data.speed;
                }

                entry.speed.estimate(speed);

                self.entries.insert(
                    *entry_id,
                    LastData {
                        lap_dist: *entry.spline_pos,
                        speed,
                    },
                );
            })
        });

        self.last_update_number = update_number;

        Ok(())
    }

    fn event(
        &mut self,
        _context: &mut IRacingProcessorContext,
        _event: &Event,
    ) -> IRacingResult<()> {
        Ok(())
    }
}
