use std::collections::HashMap;

use tracing::{debug, info};

use crate::{
    acc::{
        data::{
            BroadcastingEvent, CarLocation, EntryList, EntryListCar, LapInfo, RealtimeCarUpdate,
            RealtimeUpdate, RegistrationResult, SessionPhase, SessionType, TrackData,
        },
        AccProcessor, AccProcessorContext, ConnectionError, Result,
    },
    model::{self, Driver, DriverId, Entry, EntryId, Time},
};

/// A processor to transfer game data directly into the model.
/// Transfers only data that is available without doing any additional processing.
#[derive(Default, Debug)]
pub struct BaseProcessor {
    /// Index of the current session.
    current_session_index: i16,
    /// True if a new entry list should be requested.
    requested_entry_list: bool,
    /// State of the entries.
    entries: HashMap<EntryId, EntryState>,
}

/// The internal state of an entry.
#[derive(Debug)]
struct EntryState {
    connected: bool,
    laps: Option<i16>,
}

impl AccProcessor for BaseProcessor {
    fn registration_result(
        &mut self,
        result: &RegistrationResult,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Registration result");
        if !result.success {
            return Err(ConnectionError::ConnectionRefused {
                message: result.message.clone(),
            });
        }
        context.socket.connected = true;
        context.socket.connection_id = result.connection_id;
        context.socket.read_only = result.read_only;

        //context.socket.send_entry_list_request()?;
        context.socket.send_track_data_request()?;
        Ok(())
    }

    fn realtime_update(
        &mut self,
        update: &RealtimeUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Realtime Update");

        // Check if a new session has started.
        let session = context.model.current_session_mut();
        if session.is_none() || self.current_session_index != update.session_index {
            // Fast forward old session to end.
            if let Some(old_session) = session {
                while old_session.phase != model::SessionPhase::Finished {
                    info!("Session phase fast forwarded to {:?}", old_session.phase);
                    old_session.phase = old_session.phase.next();
                }
            }

            // Add new session to the model.
            let new_session = map_session(update);
            info!("New {:?} session detected", new_session.session_type);
            let session_id = context.model.add_session(new_session);
            context.model.current_session = session_id;

            self.current_session_index = update.session_index;
        }

        let session = context
            .model
            .current_session_mut()
            .expect("No Session available. If the list is empty then a new session should have been created");

        // Update session data
        let current_phase = map_session_phase(&update.session_phase);
        while current_phase > session.phase {
            session.phase = session.phase.next();
            info!("Session phase changed to {:?}", session.phase);
        }
        session.time_remaining = Time::from(update.session_end_time);
        session.time_of_day = Time::from(update.time_of_day * 1000.0);
        session.ambient_temp = update.ambient_temp as f32;
        session.track_temp = update.track_temp as f32;

        // Reset entry list flag
        self.requested_entry_list = false;

        Ok(())
    }

    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Realtime Car Update");

        let current_session = context
            .model
            .current_session_mut()
            .expect("There should have been a session update before a realtime update");

        let entry_id = EntryId(update.car_id as i32);

        match current_session.entries.get_mut(&entry_id) {
            Some(entry) => {
                entry.orientation = [update.pitch, update.yaw, update.roll];
                entry.position = update.position as i32;
                entry.spline_pos = update.spline_position;
                entry.current_lap.time = update.current_lap.laptime_ms.into();
                entry.current_lap.invalid = update.current_lap.is_invaliud;
                entry.performance_delta = update.delta.into();
                entry.in_pits = update.car_location == CarLocation::Pitlane;
                entry.gear = update.gear as i32;
                entry.speed = update.kmh as f32;

                let entry_state = self
                    .entries
                    .get_mut(&entry.id)
                    .expect("When an entry is in the model it should also be present here");
                // Update connected flag for this entry.
                entry_state.connected = true;

                // Check for lap completed.
                if let Some(laps) = entry_state.laps {
                    if update.laps != laps {
                        let lap = map_lap(&update.last_lap, entry.current_driver, entry.id);
                        info!("Car #{} completed lap: {}", entry.car_number, lap.time);

                        // Check personal best for entry
                        let new_entry_best = match entry.laps.get(entry.best_lap) {
                            Some(best_lap) => lap.time < best_lap.time,
                            None => true,
                        };
                        if new_entry_best {
                            info!(
                                "Car #{} achieved a new personal best lap nr.{} {}",
                                entry.car_number,
                                entry.laps.len(),
                                lap.time
                            );
                            entry.best_lap = entry.laps.len();
                        }
                        // Check personal best for driver
                        if let Some(driver) = entry.drivers.get_mut(&entry.current_driver) {
                            let new_personal_best = match entry.laps.get(driver.best_lap) {
                                Some(best_lap) => lap.time < best_lap.time,
                                None => true,
                            };
                            if new_personal_best {
                                info!(
                                        "Driver {} {} in car #{} achieved a new personal best lap nr.{}: {}",
                                        driver.first_name,
                                        driver.last_name,
                                        entry.car_number,
                                        entry.laps.len(),
                                        lap.time
                                    );
                                driver.best_lap = entry.laps.len();
                            }
                        }
                        // Check session best.
                        if lap.time < current_session.best_lap.time {
                            info!(
                                "Car #{} achieved a new session best lap: {}",
                                entry.car_number, lap.time,
                            );
                            current_session.best_lap = lap.clone();
                        }

                        entry.laps.push(lap);
                    }
                }
                entry_state.laps = Some(update.laps);
            }
            None => {
                debug!("Realtime update for unknown car id:{}", update.car_id);
                if !self.requested_entry_list {
                    debug!("Requesting new entry list");
                    context.socket.send_entry_list_request()?;
                    self.requested_entry_list = true;
                }
            }
        }
        Ok(())
    }

    fn track_data(&mut self, track: &TrackData, context: &mut AccProcessorContext) -> Result<()> {
        debug!("Track data");

        context.model.track_name = track.track_name.clone();
        context.model.track_length = track.track_meter;
        Ok(())
    }

    fn entry_list_car(
        &mut self,
        car: &EntryListCar,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Entry List Car");

        let session = match context.model.current_session_mut() {
            None => {
                debug!("No active session");
                return Ok(());
            }
            Some(s) => s,
        };

        if session.entries.contains_key(&EntryId(car.car_id as i32)) {
            return Ok(());
        }

        info!("Entry connected: #{}", car.race_number);
        let entry = map_entry(car);
        let event = model::Event::EntryConnected(entry.id);
        context.events.push_back(event);
        self.entries.insert(
            entry.id,
            EntryState {
                connected: true,
                laps: None,
            },
        );
        session.entries.insert(entry.id, entry);
        Ok(())
    }

    fn broadcast_event(
        &mut self,
        _event: &BroadcastingEvent,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Broadcasting event");
        Ok(())
    }

    fn entry_list(&mut self, _list: &EntryList, _context: &mut AccProcessorContext) -> Result<()> {
        debug!("Entry List");
        Ok(())
    }
}

fn map_entry(car: &EntryListCar) -> model::Entry {
    Entry {
        id: EntryId(car.car_id as i32),
        drivers: car
            .drivers
            .iter()
            .enumerate()
            .map(|(i, driver_info)| {
                let id = DriverId(i as i32);
                let driver = Driver {
                    id,
                    first_name: driver_info.first_name.clone(),
                    last_name: driver_info.last_name.clone(),
                    short_name: driver_info.short_name.clone(),
                    nationality: driver_info.nationality.clone(),
                    driving_time: Time::from(0),
                    best_lap: 0,
                };
                (id, driver)
            })
            .collect(),
        current_driver: DriverId(car.current_driver_index as i32),
        team_name: car.team_name.clone(),
        car: car.car_model_type.clone(),
        car_number: car.race_number,
        nationality: car.car_nationality.clone(),
        connected: true,
        ..Default::default()
    }
}

fn map_lap(lap_info: &LapInfo, driver_index: DriverId, entry_id: EntryId) -> model::Lap {
    model::Lap {
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
fn map_session(update: &RealtimeUpdate) -> model::Session {
    model::Session {
        session_type: map_session_type(&update.session_type),
        session_time: Time::from(update.session_time + update.session_end_time),
        phase: model::SessionPhase::Waiting,
        ..Default::default()
    }
}

fn map_session_phase(value: &SessionPhase) -> model::SessionPhase {
    match value {
        SessionPhase::None => model::SessionPhase::None,
        SessionPhase::Starting => model::SessionPhase::Preparing,
        SessionPhase::PreFormation => model::SessionPhase::Preparing,
        SessionPhase::FormationLap => model::SessionPhase::Formation,
        SessionPhase::PreSession => model::SessionPhase::Formation,
        SessionPhase::Session => model::SessionPhase::Active,
        SessionPhase::SessionOver => model::SessionPhase::Ending,
        SessionPhase::PostSession => model::SessionPhase::Finished,
        SessionPhase::ResultUi => model::SessionPhase::Finished,
    }
}

fn map_session_type(value: &SessionType) -> model::SessionType {
    match value {
        SessionType::Practice => model::SessionType::Practice,
        SessionType::Qualifying => model::SessionType::Qualifying,
        SessionType::Superpole => model::SessionType::Qualifying,
        SessionType::Race => model::SessionType::Race,
        SessionType::Hotlap => model::SessionType::Practice,
        SessionType::Hotstint => model::SessionType::Practice,
        SessionType::HotlapSuperpole => model::SessionType::Practice,
        SessionType::Replay => model::SessionType::None,
        SessionType::None => model::SessionType::None,
    }
}
