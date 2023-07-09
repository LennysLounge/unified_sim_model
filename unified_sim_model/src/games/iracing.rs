use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, TryRecvError},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

use thiserror::Error;
use tracing::{error, info, warn};

use crate::{
    log_todo,
    model::{self, CarCategory, Model, SessionGameData},
    AdapterCommand, GameAdapter, Temperature, UpdateEvent,
};

use self::irsdk::{
    live_data::{self, TrkLoc},
    static_data::{self},
    Data, Irsdk,
};

use super::common::distance_driven;

pub mod irsdk;

/// A specialized result for Connection errors.
type Result<T> = std::result::Result<T, crate::AdapterError>;

#[derive(Debug, Error)]
pub enum IRacingError {
    #[error("The game is not running")]
    GameNotRunning,
    #[error("The game disconnected")]
    Disconnected,
    #[error("Missing required data: {0}")]
    MissingData(String),
    #[error("The adapter encountered an error: {0}")]
    Other(String),
}

impl From<IRacingError> for crate::AdapterError {
    fn from(value: IRacingError) -> Self {
        crate::AdapterError::IRacing(value)
    }
}

pub struct IRacingAdapter;
impl GameAdapter for IRacingAdapter {
    fn run(
        &mut self,
        model: Arc<RwLock<Model>>,
        command_rx: Receiver<AdapterCommand>,
        update_event: UpdateEvent,
    ) -> Result<()> {
        let sdk = Irsdk::new().map_err(|_| IRacingError::GameNotRunning)?;

        if let Ok(mut model) = model.write() {
            model.connected = true;
            model.event_name.set("iRacing".to_owned());
        }
        let mut connection = IRacingConnection::new(model.clone(), command_rx, update_event, sdk);
        let result = connection.run_loop();

        if let Ok(mut model) = model.write() {
            model.connected = false;
        }

        result
    }
}

struct IRacingConnection {
    model: Arc<RwLock<Model>>,
    command_rx: Receiver<AdapterCommand>,
    update_event: UpdateEvent,
    sdk: Irsdk,
    static_data_update_count: Option<i32>,
    lap_processor: LapProcessor,
}

impl IRacingConnection {
    fn new(
        model: Arc<RwLock<Model>>,
        command_rx: Receiver<AdapterCommand>,
        update_event: UpdateEvent,
        sdk: Irsdk,
    ) -> Self {
        Self {
            model,
            command_rx,
            update_event,
            sdk,
            static_data_update_count: None,
            lap_processor: LapProcessor::new(),
        }
    }

    fn run_loop(&mut self) -> Result<()> {
        loop {
            let should_close = self.handle_commands()?;
            if should_close {
                break;
            }

            let data = self.sdk.poll().map_err(|e| match e {
                irsdk::PollError::NotConnected => IRacingError::Disconnected,
            })?;

            self.update_model(&data)?;
            self.update_event.trigger();

            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    fn handle_commands(&self) -> Result<bool> {
        let should_close = match self.command_rx.try_recv() {
            Ok(command) => match command {
                AdapterCommand::Close => true,
                AdapterCommand::FocusOnCar(_) => {
                    log_todo(false, "Focus on car command not implemented yet")
                }
                AdapterCommand::ChangeCamera(_) => {
                    log_todo(false, "Change camera command not implemented yet")
                }
            },
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => {
                // This should only happen if all adapters have been dropped.
                // In which case it is impossible to interact with this adapter any more.
                // To avoid leaking memory we quit.
                error!("All adapter handle have been dropped it is impossible to communicate with this game adapter.");
                true
            }
        };

        Ok(should_close)
    }

    fn update_model(&mut self, data: &Data) -> Result<()> {
        let mut model = self
            .model
            .write()
            .map_err(|_| IRacingError::Other("Model was poisoned".into()))?;

        if let None = self.static_data_update_count {
            // Initialise model
            let session_infos = &data.static_data.session_info;
            for session_info in session_infos.sessions.iter() {
                let session = init_session(session_info, data)?;
                model.sessions.insert(session.id, session);
            }
        }

        // Set current session.
        let current_session_num = data.live_data.session_num.ok_or(IRacingError::Other(
            "No session number in live data".to_owned(),
        ))?;
        let current_session_id = model::SessionId(current_session_num as usize);
        if !model.sessions.contains_key(&current_session_id) {
            return Err(IRacingError::Other(
                "Current session number is not a valid session".to_owned(),
            )
            .into());
        }
        model.current_session = Some(current_session_id);

        // Set the focused entry
        if let Some(ref cam_car_idx) = data.live_data.cam_car_idx {
            model.focused_entry = Some(model::EntryId(*cam_car_idx));
        } else {
            model.focused_entry = None;
        }

        // Update session.
        let mut current_session = model
            .current_session_mut()
            .expect("The current session should be available");
        update_session_live(&mut current_session, &data.live_data);

        // Update entries
        for (_entry_id, entry) in current_session.entries.iter_mut() {
            update_entry_live(entry, &data);
            distance_driven::calc_distance_driven(entry);
        }

        // Update lap times.
        if self
            .static_data_update_count
            .map_or(true, |count| count != data.static_data.update_count)
        {
            self.static_data_update_count = Some(data.static_data.update_count);

            // Althoug we are using the live data to check if a lap was completed, the last lap time
            // updates quite a bit after the lap counter updates. This makes it difficult to know when to
            // read out the last lap time. Instead we wait for a static data update which should happen
            // atleast everytime a car finishes a lap.
            self.lap_processor.process(&mut model, data)?;
        }

        Ok(())
    }
}

fn init_session(session_info: &static_data::Session, data: &Data) -> Result<model::Session> {
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
        game_data: SessionGameData::None,
    })
}

fn init_entries(data: &Data) -> Result<HashMap<model::EntryId, model::Entry>> {
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

fn map_entry(driver_info: &static_data::Driver) -> Result<model::Entry> {
    let driver = map_driver(driver_info)?;

    let car_idx = driver_info
        .car_idx
        .ok_or_else(|| IRacingError::MissingData("car_idx".into()))?;

    let team_name = match driver_info.team_name {
        Some(ref name) => name.clone().into(),
        None => model::Value::default(),
    };

    let car = match driver_info.car_screen_name {
        Some(ref car_name) => {
            model::Car::new(car_name.to_owned(), "".to_owned(), CarCategory::new("")).into()
        }
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

fn map_driver(driver_info: &static_data::Driver) -> Result<model::Driver> {
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

fn update_session_live(session: &mut model::Session, data: &live_data::LiveData) {
    if let Some(ref session_state) = data.session_state {
        session.phase.set(map_session_phase(session_state));
    }

    if let Some(ref time_remaining) = data.session_time_remain {
        session.time_remaining.set(time_remaining.clone());
    }

    if let Some(ref laps_remaining) = data.session_laps_remain {
        session.laps_remaining.set(laps_remaining.clone());
    }

    if let Some(ambient_temp) = data.air_temp {
        session
            .ambient_temp
            .set(Temperature::from_celcius(ambient_temp));
    }

    if let Some(track_temp) = data.track_temp {
        session
            .track_temp
            .set(Temperature::from_celcius(track_temp));
    }

    if let Some(time_of_day) = data.session_time_of_day {
        session.time_of_day.set(time_of_day.clone());
    }
}

fn map_session_phase(session_state: &live_data::SessionState) -> model::SessionPhase {
    match session_state {
        live_data::SessionState::StateInvalid => model::SessionPhase::Waiting,
        live_data::SessionState::StateGetInCar => model::SessionPhase::Preparing,
        live_data::SessionState::StateWarmup => model::SessionPhase::Preparing,
        live_data::SessionState::StateParadeLaps => model::SessionPhase::Formation,
        live_data::SessionState::StateRacing => model::SessionPhase::Active,
        live_data::SessionState::StateCheckered => model::SessionPhase::Ending,
        live_data::SessionState::StateCoolDown => model::SessionPhase::Finished,
    }
}

fn update_entry_live(entry: &mut model::Entry, data: &Data) {
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
            if let TrkLoc::NotInWorld = track_location {
                entry.connected.set(false);
            } else {
                entry.connected.set(true);
            }
        }
    }
}

struct LapProcessor {
    laps_before: HashMap<model::EntryId, i32>,
}

impl LapProcessor {
    fn new() -> Self {
        Self {
            laps_before: HashMap::new(),
        }
    }

    fn process(&mut self, model: &mut model::Model, data: &Data) -> Result<()> {
        let session = model
            .current_session_mut()
            .expect("Current session should be valid");

        let mut new_events = Vec::new();
        for (entry_id, entry) in session.entries.iter_mut() {
            let lap_completed = self
                .laps_before
                .get(&entry.id)
                .is_some_and(|lap_count| lap_count != entry.lap_count.as_ref());
            self.laps_before.insert(entry.id, entry.lap_count.as_copy());
            if !lap_completed {
                continue;
            }

            let Some(last_lap_time) = data.live_data
                .car_idx_last_lap_time
                .as_ref()
                .and_then(|lap_times| lap_times.get(entry_id.0 as usize)) else {continue};

            let Some(driver) = entry.drivers.get_mut(&entry.current_driver) else {continue};

            let lap = model::Lap {
                time: last_lap_time.clone().into(),
                splits: Vec::new().into(),
                invalid: model::Value::default(),
                driver_id: driver.id,
                entry_id: entry.id,
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

            new_events.push(model::Event::LapCompleted(model::LapCompleted {
                lap,
                is_session_best: session_best,
                is_entry_best: entry_best,
                is_driver_best: personal_best,
            }));
        }

        model.events.extend(new_events);

        Ok(())
    }
}
