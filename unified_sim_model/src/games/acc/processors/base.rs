use tracing::{debug, info};

use crate::{
    games::acc::{
        data::{
            CarLocation, EntryListCar, RealtimeCarUpdate, RegistrationResult, SessionPhase,
            SessionType, SessionUpdate, TrackData,
        },
        AccConnectionError, AccProcessor, AccProcessorContext, Result,
    },
    model::{self, Day, Driver, DriverId, Entry, EntryId, Event, Session, Value},
    time::Time,
};

/// A processor to transfer game data directly into the model.
/// Transfers only data that is available without doing any additional processing.
#[derive(Default, Debug)]
pub struct BaseProcessor {
    /// Index of the current session.
    current_session_index: Option<i16>,
    /// True if a new entry list should be requested.
    requested_entry_list: bool,
}

impl AccProcessor for BaseProcessor {
    fn registration_result(
        &mut self,
        result: &RegistrationResult,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Registration result");
        if !result.success {
            return Err(AccConnectionError::ConnectionRefused {
                message: result.message.clone(),
            }
            .into());
        }
        context.socket.connected = true;
        context.socket.connection_id = result.connection_id;
        context.socket.read_only = result.read_only;

        //context.socket.send_entry_list_request()?;
        context.socket.send_track_data_request()?;
        Ok(())
    }

    fn session_update(
        &mut self,
        update: &SessionUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Session Update");

        let is_new_session = self
            .current_session_index
            .map_or(true, |index| update.session_index != index);
        self.current_session_index = Some(update.session_index);

        if is_new_session {
            if let Some(session) = context.model.current_session_mut() {
                while session.phase != model::SessionPhase::Finished {
                    info!("Session phase fast forwarded to {:?}", session.phase);
                    session.phase.set(session.phase.next());
                    context.events.push_back(Event::SessionPhaseChanged(
                        session.id,
                        session.phase.as_copy(),
                    ));
                }
            }

            // Make new session
            let session_type = map_session_type(&update.session_type);
            let session = Session {
                session_type: session_type.into(),
                session_time: Time::from(update.session_time + update.session_end_time).into(),
                phase: model::SessionPhase::Waiting.into(),
                day: Value::with_default(Day::Sunday).with_editable(),
                ..Default::default()
            };
            let id = context.model.add_session(session);
            context.model.current_session = Some(id);

            // Create event
            info!("New {:?} session detected", session_type);
            context.events.push_back(Event::SessionChanged(id));
        }

        let session = context
            .model
            .current_session_mut()
            .expect("No Session available. If the list is empty then a new session should have been created");

        // Update session data
        let current_phase = map_session_phase(&update.session_phase);
        while current_phase > *session.phase {
            session.phase.set(session.phase.next());
            info!("Session phase changed to {:?}", session.phase);
            context.events.push_back(Event::SessionPhaseChanged(
                session.id,
                session.phase.as_copy(),
            ));
        }
        session
            .time_remaining
            .set(Time::from(update.session_end_time));
        session
            .time_of_day
            .set(Time::from(update.time_of_day * 1000.0));
        session.ambient_temp.set(update.ambient_temp as f32);
        session.track_temp.set(update.track_temp as f32);

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
                entry
                    .orientation
                    .set([update.pitch, update.yaw, update.roll]);
                entry.position.set(update.position as i32);
                entry.spline_pos.set(update.spline_position);
                entry.lap_count.set(update.laps as i32);
                entry
                    .current_lap
                    .time
                    .set(update.current_lap.laptime_ms.into());
                entry
                    .current_lap
                    .invalid
                    .set(update.current_lap.is_invaliud);
                entry.performance_delta.set(update.delta.into());
                entry
                    .in_pits
                    .set(update.car_location == CarLocation::Pitlane);
                entry.gear.set(update.gear as i32);
                entry.speed.set(update.kmh as f32);
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
        if let Some(session) = context.model.current_session_mut() {
            session.track_name.set(track.track_name.clone());
            session.track_length.set(track.track_meter);
        }
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

        let entry = map_entry(car);
        if session.entries.contains_key(&entry.id) {
            return Ok(());
        }

        info!("Entry connected: #{}", car.race_number);
        context
            .events
            .push_back(model::Event::EntryConnected(entry.id));
        session.entries.insert(entry.id, entry);
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
                    first_name: driver_info.first_name.clone().into(),
                    last_name: driver_info.last_name.clone().into(),
                    short_name: driver_info.short_name.clone().into(),
                    nationality: driver_info.nationality.clone().into(),
                    driving_time: Time::from(0).into(),
                    best_lap: None.into(),
                };
                (id, driver)
            })
            .collect(),
        current_driver: DriverId(car.current_driver_index as i32),
        team_name: car.team_name.clone().into(),
        car: car.car_model_type.clone().into(),
        car_number: car.race_number.into(),
        nationality: car.car_nationality.clone().into(),
        connected: true.into(),
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
