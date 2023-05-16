use std::collections::HashMap;

use tracing::info;

use crate::{
    acc_adapter::{
        data::{LapInfo, RealtimeCarUpdate},
        AccProcessor, AccProcessorContext, Result,
    },
    model::{event::LapCompleted, DriverId, EntryId, Event, Lap, Time},
};

#[derive(Debug, Default)]
pub struct LapProcessor {
    laps_before: HashMap<EntryId, i16>,
}

impl AccProcessor for LapProcessor {
    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        let entry_id = EntryId(update.car_id as i32);

        let laps_before = self.laps_before.insert(entry_id, update.laps);

        if laps_before.unwrap_or(update.laps) == update.laps {
            // Lap was not completed
            return Ok(());
        }

        let session = context
            .model
            .current_session_mut()
            .expect("There should have been a session update before a realtime update");

        let entry = match session.entries.get_mut(&entry_id) {
            Some(e) => e,
            None => return Ok(()),
        };

        let lap = map_lap(&update.last_lap, entry.current_driver, entry.id);
        info!("Car #{} completed lap: {}", entry.car_number, lap.time);

        // Check personal best for driver
        let personal_best = match entry.drivers.get_mut(&entry.current_driver) {
            Some(driver) => match entry.laps.get(driver.best_lap) {
                Some(best_lap) => {
                    let lap_is_better = lap.time < best_lap.time;
                    if lap_is_better {
                        driver.best_lap = entry.laps.len();
                    }
                    lap_is_better
                }
                None => true,
            },
            None => false,
        };
        // Check personal best for entry
        let entry_best = match entry.laps.get(entry.best_lap) {
            Some(best_lap) => {
                let lap_is_better = lap.time < best_lap.time;
                if lap_is_better {
                    entry.best_lap = entry.laps.len();
                }
                lap_is_better
            }
            None => true,
        };
        // Check session best.
        let session_best = lap.time < session.best_lap.time;
        if session_best {
            session.best_lap = lap.clone();
        }

        context.events.push_back(Event::LapCompleted(LapCompleted {
            lap: lap.clone(),
            is_session_best: session_best,
            is_entry_best: entry_best,
            is_driver_best: personal_best,
        }));
        entry.laps.push(lap);
        Ok(())
    }
}

fn map_lap(lap_info: &LapInfo, driver_index: DriverId, entry_id: EntryId) -> Lap {
    Lap {
        time: lap_info.laptime_ms.into(),
        splits: lap_info
            .splits
            .clone()
            .iter()
            .map(|ms| Time::from(*ms))
            .collect(),
        invalid: lap_info.is_invaliud,
        driver_id: driver_index,
        entry_id,
    }
}
