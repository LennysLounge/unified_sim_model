use std::collections::HashMap;

use tracing::{debug, info};

use crate::{
    games::acc::{
        data::{LapInfo, RealtimeCarUpdate},
        AccProcessorContext, Result,
    },
    model::{DriverId, EntryId, Event, Lap, LapCompleted, Session},
    types::Time,
};

use super::AccProcessor;

/// This processors observes lap changes and publishes `LapCompleted` events.
///
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
        let Some(session) = context.model.current_session_mut() else {
            return Ok(());
        };
        let entry_id = EntryId(update.car_id as i32);
        if !session.entries.contains_key(&entry_id) {
            return Ok(());
        }

        if let Some(laps_completed) = self.laps_before.get(&entry_id) {
            if laps_completed != &update.laps {
                context
                    .events
                    .push_back(lap_completed(session, entry_id, update));
            }
        } else {
            initialize_laps(session, entry_id, update)?;
        }
        self.laps_before.insert(entry_id, update.laps);

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
        driver_id: Some(driver_id),
        entry_id: Some(entry_id),
    }
}

fn initialize_laps(
    session: &mut Session,
    entry_id: EntryId,
    update: &RealtimeCarUpdate,
) -> Result<()> {
    let entry = session
        .entries
        .get_mut(&entry_id)
        .expect("Entry must be present here");
    let current_driver = entry.current_driver;

    // Initialize best lap and last lap
    debug!("Initialize lap times for entry {:?}", entry_id);
    let best_lap = (update.best_session_lap.laptime_ms != i32::MAX).then_some(map_lap(
        &update.best_session_lap,
        current_driver,
        entry.id,
    ));
    let last_lap = (update.last_lap.laptime_ms != i32::MAX).then_some(map_lap(
        &update.last_lap,
        current_driver,
        entry.id,
    ));

    if let Some(best_lap) = best_lap {
        debug!("Set best lap: {:?}", best_lap.time.ms);
        entry.laps.push(best_lap.clone());
        entry.best_lap = Some(best_lap.clone()).into();

        let session_best = session
            .best_lap
            .as_ref()
            .as_ref()
            .map_or(true, |session_best_lap| {
                best_lap.time < session_best_lap.time
            });
        if session_best {
            session.best_lap = Some(best_lap).into();
        }
    }
    if let Some(last_lap) = last_lap {
        // if the last lap has the same time as the best lap then they are probably the same
        // lap and we dont have to add them twice.
        if update.best_session_lap.laptime_ms != update.last_lap.laptime_ms {
            debug!("Set last lap: {:?}", last_lap.time.ms);
            entry.laps.push(last_lap);
        }
    }
    Ok(())
}

fn lap_completed(session: &mut Session, entry_id: EntryId, update: &RealtimeCarUpdate) -> Event {
    let entry = session
        .entries
        .get_mut(&entry_id)
        .expect("Entry must be present in session");

    let current_driver = entry.current_driver;

    let lap = map_lap(&update.last_lap, current_driver, entry.id);
    entry.laps.push(lap.clone());

    let personal_best = entry
        .drivers
        .get(&entry.current_driver)
        .and_then(|driver| driver.best_lap.as_ref().as_ref())
        .map_or(true, |best_lap| lap.time < best_lap.time)
        && !*lap.invalid;
    if personal_best {
        if let Some(driver) = entry.drivers.get_mut(&current_driver) {
            driver.best_lap.set(Some(lap.clone()));
        }
    }

    // Check personal best for entry
    let entry_best = entry
        .best_lap
        .as_ref()
        .as_ref()
        .map_or(true, |best_lap| lap.time < best_lap.time)
        && !*lap.invalid;
    if entry_best {
        entry.best_lap.set(Some(lap.clone()));
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

    Event::LapCompleted(LapCompleted {
        lap,
        is_session_best: session_best,
        is_entry_best: entry_best,
        is_driver_best: personal_best,
    })
}
