use std::collections::HashMap;

use tracing::info;

use crate::{
    adapter::acc::{
        data::{LapInfo, RealtimeCarUpdate},
        AccProcessor, AccProcessorContext, Result,
    },
    model::{event::LapCompleted, DriverId, Entry, EntryId, Event, Lap, Time},
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
        let lap_index = entry.laps.len();
        entry.laps.push(lap.clone());
        info!("Car #{} completed lap: {}", entry.car_number, lap.time);

        // Check personal best for driver
        fn current_driver_best_lap(entry: &Entry) -> Option<&Lap> {
            let driver = entry.drivers.get(&entry.current_driver)?;
            entry.laps.get(driver.best_lap?)
        }
        let personal_best = current_driver_best_lap(entry)
            .map_or(false, |best_lap| lap.time < best_lap.time)
            && !lap.invalid;
        if personal_best {
            if let Some(driver) = entry.drivers.get_mut(&entry.current_driver) {
                driver.best_lap = Some(lap_index);
            }
        }

        // Check personal best for entry
        fn entry_best_lap(entry: &Entry) -> Option<&Lap> {
            entry.laps.get(entry.best_lap?)
        }
        let entry_best = entry_best_lap(entry).map_or(false, |best_lap| lap.time < best_lap.time)
            && !lap.invalid;
        if entry_best {
            entry.best_lap = Some(lap_index);
        }

        // Check session best.
        let session_best = lap.time < session.best_lap.time && !lap.invalid;
        if session_best {
            session.best_lap = lap.clone();
        }

        context.events.push_back(Event::LapCompleted(LapCompleted {
            lap: lap.clone(),
            is_session_best: session_best,
            is_entry_best: entry_best,
            is_driver_best: personal_best,
        }));
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
