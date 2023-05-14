use std::collections::HashMap;

use tracing::{debug, info};

use crate::{
    acc::{data::CarLocation, ConnectionError},
    model::{self, Driver, Entry, Time},
};

use super::{
    data::{
        BroadcastingEvent, EntryListCar, RealtimeCarUpdate, RealtimeUpdate, RegistrationResult,
        SessionPhase, SessionType, TrackData,
    },
    AccProcessor, AccProcessorContext, Result,
};

#[derive(Default, Debug)]
pub struct BaseProcessor {
    /// Index of the current session.
    current_session_index: i16,
    /// True if a new entry list should be requested.
    requested_entry_list: bool,
    /// State of the entries.
    entries: HashMap<i32, EntryState>,
}

/// The internal state of an entry.
#[derive(Debug)]
struct EntryState {
    connected: bool,
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

        if self.current_session_index != update.session_index || context.model.sessions.is_empty() {
            // A new session has started.
            let session = map_session(update, context.model.sessions.len() as i32);
            info!("New session detected {}", session.session_type);
            context.model.current_session = context.model.sessions.len();
            context.model.sessions.push(session);
            self.current_session_index = update.session_index;
        }

        let session = context
            .model
            .current_session_mut()
            .expect("No Session available. If the list is empty then a new session should have been created");

        let current_phase = map_session_phase(&update.session_phase);
        if current_phase != session.phase {
            info!("Session phase changed to {:?}", current_phase);
            session.phase = current_phase;
        }
        session.time_remaining = Time::from(update.session_end_time);
        session.time_of_day = Time::from(update.time_of_day * 1000.0);
        session.ambient_temp = update.ambient_temp as f32;
        session.track_temp = update.track_temp as f32;

        // Reset entry list flag
        self.requested_entry_list = false;

        // Check disconnects
        for entry in session.entries.values_mut() {
            let state = self
                .entries
                .get_mut(&entry.id)
                .expect("Entry states should always contain the same entries as the model");
            match (entry.connected, state.connected) {
                (true, false) => {
                    info!("Entry disconnected #{}", entry.car_number);
                }
                (false, true) => {
                    info!("Entry reconnected #{}", entry.car_number);
                }
                _ => (),
            }
            entry.connected = state.connected;
            // Reset connection state for next update.
            state.connected = false;
        }

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

        if let Some(entry) = current_session.entries.get_mut(&(update.car_id as i32)) {
            entry.orientation = [update.pitch, update.yaw, update.roll];
            entry.position = update.position as i32;
            entry.spline_pos = update.spline_position;
            entry.current_lap.time = update.current_lap.laptime_ms.into();
            entry.current_lap.invalid = update.current_lap.is_invaliud;
            entry.performance_delta = update.delta.into();
            entry.in_pits = update.car_location == CarLocation::Pitlane;
            entry.gear = update.gear as i32;
            entry.speed = update.kmh as f32;

            // Update connected flag for this entry.
            let entry_state = self
                .entries
                .get_mut(&entry.id)
                .expect("When an entry is in the model it should also be present here");
            entry_state.connected = true;
        } else {
            debug!("Realtime update for unknown car id:{}", update.car_id);
            if !self.requested_entry_list {
                debug!("Requesting new entry list");
                context.socket.send_entry_list_request()?;
                self.requested_entry_list = true;
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
            None => return Ok(()),
            Some(s) => s,
        };

        if session.entries.contains_key(&(car.car_id as i32)) {
            return Ok(());
        }

        info!("New entry has connected: #{}", car.race_number);
        let entry = Entry {
            id: car.car_id as i32,
            drivers: {
                let mut drivers = Vec::new();
                for (i, driver_info) in car.drivers.iter().enumerate() {
                    drivers.push(Driver {
                        id: i,
                        first_name: driver_info.first_name.clone(),
                        last_name: driver_info.last_name.clone(),
                        short_name: driver_info.short_name.clone(),
                        nationality: driver_info.nationality.clone(),
                        driving_time: Time::from(0),
                    });
                }
                drivers
            },
            current_driver: car.current_driver_index as usize,
            team_name: car.team_name.clone(),
            car: car.car_model_type.clone(),
            car_number: car.race_number,
            nationality: car.car_nationality.clone(),
            connected: true,
            ..Default::default()
        };

        self.entries
            .insert(entry.id, EntryState { connected: true });
        session.entries.insert(entry.id, entry);
        Ok(())
    }

    fn broadcast_even(
        &mut self,
        _event: &BroadcastingEvent,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Broadcasting event");
        Ok(())
    }

    fn entry_list(
        &mut self,
        _list: &super::data::EntryList,
        _context: &mut AccProcessorContext,
    ) -> super::Result<()> {
        debug!("Entry List");
        Ok(())
    }
}

fn map_session(update: &RealtimeUpdate, id: i32) -> model::Session {
    model::Session {
        id,
        session_type: map_session_type(&update.session_type),
        session_time: Time::from(update.session_time + update.session_end_time),
        time_remaining: Time::from(update.session_end_time),
        phase: map_session_phase(&update.session_phase),
        time_of_day: Time::from(update.time_of_day * 1000.0),
        ambient_temp: update.ambient_temp as f32,
        track_temp: update.track_temp as f32,
        ..Default::default()
    }
}

fn map_session_phase(value: &SessionPhase) -> model::SessionPhase {
    match value {
        SessionPhase::None => model::SessionPhase::None,
        SessionPhase::Starting => model::SessionPhase::PreSession,
        SessionPhase::PreFormation => model::SessionPhase::PreSession,
        SessionPhase::FormationLap => model::SessionPhase::PostSession,
        SessionPhase::PreSession => model::SessionPhase::PreSession,
        SessionPhase::Session => model::SessionPhase::Session,
        SessionPhase::SessionOver => model::SessionPhase::PostSession,
        SessionPhase::PostSession => model::SessionPhase::PostSession,
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
