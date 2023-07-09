use tracing::{debug, info};

use crate::{
    games::acc::{
        data::{
            CarLocation, EntryListCar, RealtimeCarUpdate, RegistrationResult, SessionPhase,
            SessionType, SessionUpdate, TrackData,
        },
        model::{AccCamera, AccEntry, AccSession},
        AccConnectionError, AccProcessorContext, Result,
    },
    model::{
        self, Camera, Day, Driver, DriverId, Entry, EntryGameData, EntryId, Event, GameCamera, Lap,
        Nationality, Session, SessionGameData, Value,
    },
    time::Time,
    Distance, Temperature,
};

use super::AccProcessor;

/// A processor to transfer game data directly into the model.
/// Transfers only data that is available without doing any additional processing.
#[derive(Default, Debug)]
pub struct BaseProcessor {
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

        let current_session_index = context
            .model
            .current_session()
            .map(|session| {
                session
                    .game_data
                    .assert_acc()
                    .map(|data| data.session_index)
            })
            .transpose()?;

        let is_new_session =
            current_session_index.map_or(true, |index| update.session_index != index);

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
                game_data: SessionGameData::Acc(AccSession::default()),
                best_lap: Value::new(None),
                ..Default::default()
            };
            let id = context.model.add_session(session);
            context.model.current_session = Some(id);

            // Create event
            info!("New {:?} session detected", session_type);
            context.events.push_back(Event::SessionChanged(id));

            // Ask for track data.
            // I dont think that acc can change tracks between sessions right now. In principle
            // that means we only have to ask for track data once and could use that every time.
            // For simplicity we just request the track data for every session.
            context.socket.send_track_data_request()?;
        }

        let session = context
            .model
            .current_session_mut()
            .ok_or(AccConnectionError::Other(
                "No current session on a session update".to_owned(),
            ))?;

        // Update game data
        let game_data = session.game_data.assert_acc_mut()?;
        game_data.event_index = update.event_index;
        game_data.session_index = update.session_index;
        game_data.camera_set = update.active_camera_set.clone();
        game_data.camera = update.active_camera.clone();
        game_data.hud_page = update.current_hud_page.clone();
        game_data.cloud_level = update.cloud_level;
        game_data.rain_level = update.rain_level;
        game_data.wetness = update.wetness;

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
        session.time_remaining.set(update.session_end_time.into());
        session
            .time_of_day
            .set((update.time_of_day * 1000.0).into());
        session
            .ambient_temp
            .set(Temperature::from_celcius(update.ambient_temp as f32));
        session
            .track_temp
            .set(Temperature::from_celcius(update.track_temp as f32));

        // Set focused car.
        let focused_entry = EntryId(update.focused_car_id);
        for entry in session.entries.values_mut() {
            entry.focused = entry.id == focused_entry;
        }
        context.model.focused_entry = session
            .entries
            .contains_key(&focused_entry)
            .then_some(focused_entry);
        context.model.active_camera = map_camera(&update.active_camera_set, &update.active_camera)
            .unwrap_or(Camera::None)
            .into();

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

        let entry_id = EntryId(update.car_id as i32);

        let session = context
            .model
            .current_session_mut()
            .ok_or(AccConnectionError::Other(
                "No current session on a realtime car update".to_owned(),
            ))?;

        let entry = session.entries.get_mut(&entry_id);
        if entry.is_none() {
            debug!("Realtime update for unknown car id:{}", update.car_id);
            if !self.requested_entry_list {
                debug!("Requesting new entry list");
                context.socket.send_entry_list_request()?;
                self.requested_entry_list = true;
            }
            return Ok(());
        }

        let current_driver_id = DriverId(update.driver_id as i32);
        let entry = entry.unwrap();
        entry.current_driver = current_driver_id;
        entry
            .orientation
            .set([update.pitch, update.yaw, update.roll]);
        entry.position.set(update.position as i32);
        entry.spline_pos.set(update.spline_position);
        entry.lap_count.set(update.laps as i32);
        entry.current_lap.set(Lap {
            time: Time::from(update.current_lap.laptime_ms).into(),
            splits: Vec::new().into(),
            invalid: update.current_lap.is_invaliud.into(),
            driver_id: current_driver_id,
            entry_id,
        });
        entry.current_lap.set_available();
        entry.performance_delta.set(update.delta.into());
        entry
            .in_pits
            .set(update.car_location == CarLocation::Pitlane);
        entry.gear.set(update.gear as i32);
        entry.speed.set(update.kmh as f32);

        let game_data = entry.game_data.assert_acc_mut()?;
        game_data.car_location = update.car_location.clone();
        game_data.cup_position = update.cup_position;
        game_data.track_position = update.track_position;

        Ok(())
    }

    fn track_data(&mut self, track: &TrackData, context: &mut AccProcessorContext) -> Result<()> {
        debug!("Track data");
        if let Some(session) = context.model.current_session_mut() {
            session.track_name.set(track.track_name.clone());
            session
                .track_length
                .set(Distance::from_meter(track.track_meter as f32));
        }
        let available_cameras = &mut context.model.available_cameras;
        for (set, cameras) in track.camera_sets.iter() {
            for camera in cameras.iter() {
                if let Some(c) = map_camera(set, camera) {
                    available_cameras.insert(c);
                }
            }
        }
        Ok(())
    }

    fn entry_list_car(
        &mut self,
        car: &EntryListCar,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        debug!("Entry List Car");

        let Some(session) = context.model.current_session_mut() else {return Ok(())};

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
        // Entry ids should be unique in a session.
        // This only works because ids are unique in the game.
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
                    best_lap: None.into(),
                    ..Default::default()
                };
                (id, driver)
            })
            .collect(),
        current_driver: DriverId(car.current_driver_index as i32),
        team_name: Value::<String>::default().with_editable(),
        car: car.car_model_type.clone().into(),
        car_number: car.race_number.into(),
        nationality: Value::<Nationality>::default().with_editable(),
        connected: true.into(),
        best_lap: None.into(),
        game_data: EntryGameData::Acc(AccEntry {
            car_id: car.car_id,
            cup_category: car.cup_category,
            ..Default::default()
        }),
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

fn map_camera(set: &str, camera: &str) -> Option<Camera> {
    match set {
        "Helicam" => Some(Camera::Hellicopter),
        "pitlane" => Some(Camera::Game(GameCamera::Acc(AccCamera::Pitlane))),
        "set1" => Some(Camera::TV),
        "set2" => Some(Camera::Game(GameCamera::Acc(AccCamera::Tv2))),
        "Drivable" => match camera {
            "Chase" => Some(Camera::Chase),
            "FarChase" => Some(Camera::Game(GameCamera::Acc(AccCamera::FarChase))),
            "Bonnet" => Some(Camera::Game(GameCamera::Acc(AccCamera::Bonnet))),
            "DashPro" => Some(Camera::Game(GameCamera::Acc(AccCamera::DashPro))),
            "Cockpit" => Some(Camera::FirstPerson),
            "Dash" => Some(Camera::Game(GameCamera::Acc(AccCamera::Dash))),
            "Helmet" => Some(Camera::Game(GameCamera::Acc(AccCamera::Helmet))),
            _ => None,
        },
        "Onboard" => match camera {
            "Onboard0" => Some(Camera::Game(GameCamera::Acc(AccCamera::Onboard0))),
            "Onboard1" => Some(Camera::Game(GameCamera::Acc(AccCamera::Onboard1))),
            "Onboard2" => Some(Camera::Game(GameCamera::Acc(AccCamera::Onboard2))),
            "Onboard3" => Some(Camera::Game(GameCamera::Acc(AccCamera::Onboard3))),
            _ => None,
        },
        _ => None,
    }
}
