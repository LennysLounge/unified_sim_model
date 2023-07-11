use std::collections::{HashMap, VecDeque};

use tracing::{info, warn};

use crate::{
    games::{
        common::distance_driven,
        iracing::{
            irsdk::{
                defines::{SessionState, TrkLoc},
                static_data, Data,
            },
            IRacingError, IRacingResult,
        },
    },
    model, Temperature,
};

use super::{IRacingProcessor, IRacingProcessorContext};

pub struct BaseProcessor {}

impl IRacingProcessor for BaseProcessor {
    fn live_data(&mut self, context: &mut IRacingProcessorContext) -> IRacingResult<()> {
        // let IRacingProcessorContext {
        //     model,
        //     events: _,
        //     data,
        // } = context;
        // Set current session.
        let current_session_num = context
            .data
            .live_data
            .session_num
            .ok_or(IRacingError::Other(
                "No session number in live data".to_owned(),
            ))?;
        let current_session_id = model::SessionId(current_session_num as usize);

        if Some(current_session_id) != context.model.current_session {
            // Make sure the new session is a valid session
            if !context.model.sessions.contains_key(&current_session_id) {
                return Err(IRacingError::Other(
                    "Current session number is not a valid session".to_owned(),
                )
                .into());
            }
            // forward current session.
            if let Some(session) = context.model.current_session_mut() {
                while session.phase != model::SessionPhase::Finished {
                    info!(
                        "Session phase fast forwarded to {:?}",
                        session.phase.as_ref()
                    );
                    session.phase.set(session.phase.next());
                    context.events.push_back(model::Event::SessionPhaseChanged(
                        session.id,
                        session.phase.as_copy(),
                    ));
                }
            }

            // Set current session
            context.model.current_session = Some(current_session_id);

            // Create event
            let current_session = context
                .model
                .current_session()
                .expect("Current session should be valid");
            info!(
                "New {:?} session detected",
                current_session.session_type.as_ref()
            );
            context
                .events
                .push_back(model::Event::SessionChanged(current_session.id));
        }

        // Set the focused entry
        if let Some(ref cam_car_idx) = context.data.live_data.cam_car_idx {
            context.model.focused_entry = Some(model::EntryId(*cam_car_idx));
        } else {
            context.model.focused_entry = None;
        }

        // Update session.
        update_session_live(context);

        // Update entries
        let current_session = context
            .model
            .current_session_mut()
            .expect("The current session should be available");
        for (_entry_id, entry) in current_session.entries.iter_mut() {
            update_entry_live(entry, &context.data, &mut context.events);
            distance_driven::calc_distance_driven(entry);
        }
        Ok(())
    }

    fn static_data(&mut self, context: &mut IRacingProcessorContext) -> IRacingResult<()> {
        let IRacingProcessorContext {
            model,
            events: _,
            data,
        } = context;
        // Create sessions
        if model.sessions.len() != data.static_data.session_info.sessions.len() {
            for session_info in data.static_data.session_info.sessions.iter() {
                let session = init_session(session_info, data)?;
                model.sessions.insert(session.id, session);
            }
        }
        // // Create cameras
        // for group_def in context.data.static_data.camera_info.groups.iter() {
        //     let Some(ref group_num) = group_def.group_num else {continue};
        //     let Some(ref group_name) = group_def.group_name else {continue};
        //     for camera_def in group_def.cameras.iter() {
        //         let Some(ref camera_num) = camera_def.camera_num else {continue};
        //         let Some(ref camera_name) = camera_def.camera_name else {continue};
        //         // let camera =
        //         //     model::Camera::Game(model::GameCamera::IRacing(IRacingCamera::Generic {
        //         //         group_num: *group_num,
        //         //         group_name: group_name.clone(),
        //         //         camera_num: *camera_num,
        //         //         camera_name: camera_name.clone(),
        //         //     }));
        //         //context.model.available_cameras.insert(camera);
        //     }
        // }
        Ok(())
    }

    fn event(
        &mut self,
        _context: &mut IRacingProcessorContext,
        _event: &model::Event,
    ) -> IRacingResult<()> {
        Ok(())
    }
}

fn init_session(session_info: &static_data::Session, data: &Data) -> IRacingResult<model::Session> {
    let session_num = session_info
        .session_num
        .ok_or_else(|| IRacingError::MissingData("session_num".into()))?;
    let id = model::SessionId(session_num as usize);

    let session_type = match session_info.session_type {
        Some(ref type_str) => map_session_type(type_str).into(),
        None => Err(IRacingError::MissingData("session_type".into()))?,
    };

    let session_time = match session_info.session_time {
        Some(ref time) => match time {
            static_data::MaybeUnlimited::Unlimited => model::Value::default(),
            static_data::MaybeUnlimited::Value(t) => t.clone().into(),
        },
        None => Err(IRacingError::MissingData("session_time".into()))?,
    };

    let laps = match session_info.session_laps {
        Some(ref laps) => match laps {
            static_data::MaybeUnlimited::Unlimited => model::Value::default(),
            static_data::MaybeUnlimited::Value(laps) => laps.clone().into(),
        },
        None => Err(IRacingError::MissingData("session_laps".into()))?,
    };

    let time_of_day = match data.static_data.weekend_info.weekend_options {
        Some(static_data::WeekendOptions {
            time_of_day: Some(ref time_of_day),
            ..
        }) => time_of_day.clone().into(),
        _ => model::Value::default(),
    };

    let ambient_temp = match data.static_data.weekend_info.track_air_temp {
        Some(temp) => temp.clone().into(),
        None => model::Value::default(),
    };

    let track_temp = match data.static_data.weekend_info.track_surface_temp {
        Some(temp) => temp.clone().into(),
        None => model::Value::default(),
    };

    let track_name = match data.static_data.weekend_info.track_name {
        Some(ref track_name) => track_name.clone().into(),
        None => model::Value::default(),
    };

    let track_length = match data.static_data.weekend_info.track_length {
        Some(ref track_length) => track_length.clone().into(),
        None => model::Value::default(),
    };

    Ok(model::Session {
        id,
        entries: init_entries(data)?,
        session_type,
        phase: model::SessionPhase::Waiting.into(),
        session_time,
        time_remaining: model::Value::default(),
        laps,
        laps_remaining: model::Value::default(),
        time_of_day,
        day: model::Value::default(),
        ambient_temp,
        track_temp,
        best_lap: model::Value::default(),
        track_name,
        track_length,
        game_data: model::SessionGameData::None,
    })
}

fn init_entries(data: &Data) -> IRacingResult<HashMap<model::EntryId, model::Entry>> {
    let mut entries = HashMap::new();

    let driver_infos = &data.static_data.driver_info;
    for driver_info in driver_infos.drivers.iter() {
        if driver_info.car_is_pace_car.is_some_and(|v| v > 0) {
            // This is a pace car
            continue;
        }

        let Some(car_idx) = driver_info.car_idx else {
            Err(IRacingError::MissingData("car_idx".into()))?
        };
        let entry_id = model::EntryId(car_idx);
        if !entries.contains_key(&entry_id) {
            let entry = map_entry(driver_info)?;
            entries.insert(entry.id, entry);
        }
    }
    Ok(entries)
}

fn map_entry(driver_info: &static_data::Driver) -> IRacingResult<model::Entry> {
    let driver = map_driver(driver_info)?;

    let car_idx = driver_info
        .car_idx
        .ok_or_else(|| IRacingError::MissingData("car_idx".into()))?;

    let team_name = match driver_info.team_name {
        Some(ref name) => name.clone().into(),
        None => model::Value::default(),
    };

    let car = match driver_info.car_screen_name {
        Some(ref car_name) => model::Car::new(
            car_name.to_owned(),
            "".to_owned(),
            model::CarCategory::new(""),
        )
        .into(),
        None => model::Value::default(),
    };

    let car_number = match driver_info.car_number_raw {
        Some(number) => number.into(),
        None => model::Value::default(),
    };

    Ok(model::Entry {
        id: model::EntryId(car_idx),
        drivers: {
            let mut drivers = HashMap::new();
            drivers.insert(driver.id, driver.clone());
            drivers
        },
        current_driver: driver.id,
        team_name,
        car,
        car_number,
        nationality: model::Value::<model::Nationality>::default().with_editable(),
        world_pos: model::Value::default(),
        orientation: model::Value::default(),
        position: model::Value::default(),
        spline_pos: model::Value::default(),
        lap_count: model::Value::default(),
        laps: Vec::new(),
        current_lap: model::Value::default(),
        best_lap: model::Value::new(None),
        performance_delta: model::Value::default(),
        time_behind_leader: model::Value::default(),
        in_pits: model::Value::default(),
        gear: model::Value::default(),
        speed: model::Value::default(),
        connected: model::Value::default(),
        stint_time: model::Value::default(),
        distance_driven: model::Value::default(),
        focused: false,
        game_data: model::EntryGameData::None,
    })
}

fn map_driver(driver_info: &static_data::Driver) -> IRacingResult<model::Driver> {
    let (first_name, last_name) = {
        let split: Option<(String, String)> = driver_info.user_name.clone().and_then(|name| {
            name.split_once(" ")
                .map(|(l, r)| (l.to_owned(), r.to_owned()))
        });
        if let Some((first_name, last_name)) = split {
            (first_name.into(), last_name.into())
        } else {
            (model::Value::default(), model::Value::default())
        }
    };

    let car_idx = driver_info
        .car_idx
        .ok_or_else(|| IRacingError::MissingData("car_idx".into()))?;

    Ok(model::Driver {
        id: model::DriverId(car_idx),
        first_name,
        last_name,
        short_name: model::Value::default(),
        nationality: model::Value::default(),
        driving_time: model::Value::default(),
        best_lap: model::Value::default(),
    })
}

fn map_session_type(session_type_str: &str) -> model::SessionType {
    match session_type_str {
        "Race" => model::SessionType::Race,
        "Practice" => model::SessionType::Practice,
        "Open Qualify" => model::SessionType::Qualifying,
        _ => {
            warn!("Unknown session type: {}", session_type_str);
            model::SessionType::None
        }
    }
}

fn update_session_live(context: &mut IRacingProcessorContext) {
    let session = context
        .model
        .current_session_mut()
        .expect("Current session should be valid");

    if let Some(ref session_state) = context.data.live_data.session_state {
        let new_phase = map_session_phase(session_state);
        if session.phase != new_phase {
            info!("Session phase changed to {:?}", new_phase);
            session.phase.set(new_phase);
            context.events.push_back(model::Event::SessionPhaseChanged(
                session.id,
                session.phase.as_copy(),
            ));
        }
    }

    if let Some(ref time_remaining) = context.data.live_data.session_time_remain {
        session.time_remaining.set(time_remaining.clone());
    }

    if let Some(ref laps_remaining) = context.data.live_data.session_laps_remain {
        session.laps_remaining.set(laps_remaining.clone());
    }

    if let Some(ambient_temp) = context.data.live_data.air_temp {
        session
            .ambient_temp
            .set(Temperature::from_celcius(ambient_temp));
    }

    if let Some(track_temp) = context.data.live_data.track_temp {
        session
            .track_temp
            .set(Temperature::from_celcius(track_temp));
    }

    if let Some(time_of_day) = context.data.live_data.session_time_of_day {
        session.time_of_day.set(time_of_day.clone());
    }
}

fn map_session_phase(session_state: &SessionState) -> model::SessionPhase {
    match session_state {
        SessionState::StateInvalid => model::SessionPhase::Waiting,
        SessionState::StateGetInCar => model::SessionPhase::Preparing,
        SessionState::StateWarmup => model::SessionPhase::Preparing,
        SessionState::StateParadeLaps => model::SessionPhase::Formation,
        SessionState::StateRacing => model::SessionPhase::Active,
        SessionState::StateCheckered => model::SessionPhase::Ending,
        SessionState::StateCoolDown => model::SessionPhase::Finished,
    }
}

fn update_entry_live(entry: &mut model::Entry, data: &Data, events: &mut VecDeque<model::Event>) {
    let car_idx = entry.id.0 as usize;

    // TODO: Update current driver for team races.

    if let Some(ref car_idx_position) = data.live_data.car_idx_position {
        if let Some(position) = car_idx_position.get(car_idx) {
            entry.position.set(*position);
        }
    }

    if let Some(ref car_idx_lap_dist_pct) = data.live_data.car_idx_lap_dist_pct {
        if let Some(spline_pos) = car_idx_lap_dist_pct.get(car_idx) {
            entry.spline_pos.set(*spline_pos);
        }
    }

    if let Some(ref car_idx_laps) = data.live_data.car_idx_lap_completed {
        if let Some(laps) = car_idx_laps.get(car_idx) {
            entry.lap_count.set((*laps).max(0));
        }
    }

    if let Some(ref lap_time_est) = data.live_data.car_idx_est_time {
        if let Some(time) = lap_time_est.get(car_idx) {
            entry.current_lap.set(model::Lap {
                time: time.clone().into(),
                splits: Vec::new().into(),
                invalid: model::Value::default(),
                driver_id: entry.current_driver,
                entry_id: entry.id,
            });
        }
    }

    if let Some(ref car_idx_f2_time) = data.live_data.car_idx_f2_time {
        if let Some(time) = car_idx_f2_time.get(car_idx) {
            entry.time_behind_leader.set(time.clone());
        }
    }

    if let Some(ref car_idx_on_pit_road) = data.live_data.car_idx_on_pit_road {
        if let Some(on_pit_road) = car_idx_on_pit_road.get(car_idx) {
            entry.in_pits.set(*on_pit_road);
        }
    }

    if let Some(ref car_idx_gear) = data.live_data.car_idx_gear {
        if let Some(gear) = car_idx_gear.get(car_idx) {
            entry.gear.set(*gear);
        }
    }

    if let Some(ref cam_car_idx) = data.live_data.cam_car_idx {
        entry.focused = *cam_car_idx as usize == car_idx;
    }

    if let Some(ref car_idx_track_surface) = data.live_data.car_idx_track_surface {
        if let Some(track_location) = car_idx_track_surface.get(car_idx) {
            let connected = !matches!(track_location, TrkLoc::NotInWorld);
            let was_connected = entry.connected.as_copy();
            entry.connected.set(connected);
            match (connected, was_connected) {
                (true, false) => {
                    info!("Entry reconnected: #{}", *entry.car_number);
                    events.push_back(model::Event::EntryConnected {
                        id: entry.id,
                        reconnect: true,
                    });
                }
                (false, true) => {
                    info!("Entry disconnected: #{}", *entry.car_number);
                    events.push_back(model::Event::EntryDisconnected(entry.id));
                }
                _ => (),
            }
            entry.connected.set(connected);
        }
    }
}
