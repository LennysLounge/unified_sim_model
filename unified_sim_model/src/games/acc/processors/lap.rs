use std::collections::HashMap;

use tracing::info;

use crate::{
    games::acc::{
        data::{LapInfo, RealtimeCarUpdate},
        AccConnectionError, AccProcessor, AccProcessorContext, Result,
    },
    model::{DriverId, Entry, EntryId, Event, Lap, LapCompleted},
    time::Time,
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
            .ok_or(AccConnectionError::Other(
                "No current session on a realtime car update".to_owned(),
            ))?;

        let Some(entry) = session.entries.get_mut(&entry_id) else {return Ok(())};

        let lap = map_lap(&update.last_lap, entry.current_driver, entry.id);
        let lap_index = entry.laps.len();
        entry.laps.push(lap.clone());

        // Check personal best for driver
        fn current_driver_best_lap(entry: &Entry) -> Option<&Lap> {
            let driver = entry.drivers.get(&entry.current_driver)?;
            entry.laps.get((*driver.best_lap)?)
        }
        let personal_best = current_driver_best_lap(entry)
            .map_or(true, |best_lap| lap.time < best_lap.time)
            && !*lap.invalid;
        if personal_best {
            if let Some(driver) = entry.drivers.get_mut(&entry.current_driver) {
                driver.best_lap.set(Some(lap_index));
            }
        }

        // Check personal best for entry
        let entry_best = entry
            .best_lap
            .as_ref()
            .and_then(|best_lap| entry.laps.get(best_lap))
            .map_or(true, |best_lap| lap.time < best_lap.time)
            && !*lap.invalid;
        if entry_best {
            entry.best_lap.set(Some(lap_index));
        }

        // Check session best.
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

        context.events.push_back(Event::LapCompleted(LapCompleted {
            lap: lap.clone(),
            is_session_best: session_best,
            is_entry_best: entry_best,
            is_driver_best: personal_best,
        }));
        Ok(())
    }
}

fn map_lap(lap_info: &LapInfo, driver_id: DriverId, entry_id: EntryId) -> Lap {
    Lap {
        time: Time::from(lap_info.laptime_ms).into(),
        splits: lap_info
            .splits
            .clone()
            .iter()
            .map(|ms| Time::from(*ms))
            .collect::<Vec<_>>()
            .into(),
        invalid: lap_info.is_invaliud.into(),
        driver_id,
        entry_id,
    }
}
