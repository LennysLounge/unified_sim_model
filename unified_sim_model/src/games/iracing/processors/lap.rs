use std::collections::HashMap;

use tracing::info;

use crate::{games::iracing::IRacingResult, model};

use super::{IRacingProcessor, IRacingProcessorContext};

pub struct LapProcessor {
    laps_before: HashMap<model::EntryId, i32>,
}

impl LapProcessor {
    pub fn new() -> Self {
        Self {
            laps_before: HashMap::new(),
        }
    }
}

impl IRacingProcessor for LapProcessor {
    fn live_data(&mut self, _context: &mut IRacingProcessorContext) -> IRacingResult<()> {
        Ok(())
    }

    fn static_data(&mut self, context: &mut super::IRacingProcessorContext) -> IRacingResult<()> {
        // Althoug we are using the live data to check if a lap was completed, the last lap time
        // updates quite a bit after the lap counter updates. This makes it difficult to know when to
        // read out the last lap time. Instead we wait for a static data update which should happen
        // atleast everytime a car finishes a lap.

        let Some(session) = context
            .model
            .current_session_mut() else {return Ok(())};

        for (entry_id, entry) in session.entries.iter_mut() {
            let lap_completed = self
                .laps_before
                .get(&entry.id)
                .is_some_and(|lap_count| lap_count != entry.lap_count.as_ref());
            self.laps_before.insert(entry.id, entry.lap_count.as_copy());
            if !lap_completed {
                continue;
            }

            let (last_lap_time, invalid) = {
                let Some(last_lap_time) = context.data.live_data
                    .car_idx_last_lap_time
                    .as_ref()
                    .and_then(|lap_times| lap_times.get(entry_id.0 as usize)) else {continue};
                if last_lap_time.ms == -1000.0 {
                    (last_lap_time.clone(), true)
                } else {
                    (last_lap_time.clone(), false)
                }
            };

            let Some(driver) = entry.drivers.get_mut(&entry.current_driver) else {continue};

            let lap = model::Lap {
                time: last_lap_time.into(),
                splits: Vec::new().into(),
                invalid: invalid.into(),
                driver_id: Some(driver.id),
                entry_id: Some(entry.id),
            };
            entry.laps.push(lap.clone());

            let personal_best = driver
                .best_lap
                .as_ref()
                .as_ref()
                .map_or(true, |best_lap| lap.time < best_lap.time)
                && !*lap.invalid;
            if personal_best {
                driver.best_lap.set(Some(lap.clone()));
            }

            let entry_best = entry
                .best_lap
                .as_ref()
                .as_ref()
                .map_or(true, |best_lap| lap.time < best_lap.time)
                && !*lap.invalid;
            if entry_best {
                entry.best_lap.set(Some(lap.clone()));
            }

            let session_best = session
                .best_lap
                .as_ref()
                .as_ref()
                .map_or(true, |best_lap| lap.time < best_lap.time)
                && !*lap.invalid;
            if session_best {
                session.best_lap.set(Some(lap.clone()));
            }

            info!(
                "Car #{} completed lap: {} {}{}{}",
                entry.car_number,
                lap.time,
                if personal_best { "P" } else { "" },
                if entry_best { "E" } else { "" },
                if session_best { "S" } else { "" },
            );

            context
                .events
                .push_back(model::Event::LapCompleted(model::LapCompleted {
                    lap,
                    is_session_best: session_best,
                    is_entry_best: entry_best,
                    is_driver_best: personal_best,
                }));
        }
        Ok(())
    }

    fn event(
        &mut self,
        context: &mut super::IRacingProcessorContext,
        event: &model::Event,
    ) -> IRacingResult<()> {
        match event {
            model::Event::SessionChanged(_) => {
                // clear data and initialise it again.
                self.laps_before.clear();
                self.static_data(context)?;
            }
            _ => (),
        }
        Ok(())
    }
}
