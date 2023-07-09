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
use tracing::{error, warn};

use crate::{
    log_todo,
    model::{self, CarCategory, Model, SessionGameData},
    AdapterCommand, GameAdapter, Temperature, UpdateEvent,
};

use self::irsdk::{
    live_data,
    static_data::{self},
    Data, Irsdk,
};

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
            self.static_data_update_count = Some(data.static_data.update_count);
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

        // Update session.
        let mut current_session = model
            .current_session_mut()
            .expect("The current session should be available");
        update_session_live(&mut current_session, &data.live_data);

        // Update entries
        for (_entry_id, entry) in current_session.entries.iter_mut() {
            update_entry_live(entry, &data);
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
        let Some(team_id) = driver_info.team_id else {
            Err(IRacingError::MissingData("team_id".into()))?
        };
        let entry_id = model::EntryId(team_id);
        if !entries.contains_key(&entry_id) {
            let entry = map_entry(driver_info)?;
            entries.insert(entry.id, entry);
        }

        let entry = entries
            .get_mut(&entry_id)
            .expect("entry should have been just created");
        let driver = map_driver(driver_info)?;
        entry.drivers.insert(driver.id, driver);
    }
    Ok(entries)
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

fn map_entry(driver_info: &static_data::Driver) -> Result<model::Entry> {
    let team_id = driver_info
        .team_id
        .ok_or_else(|| IRacingError::MissingData("team_id".into()))?;

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
        id: model::EntryId(team_id),
        drivers: HashMap::new(),
        current_driver: None,
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
    // TODO: Update current driver.
}
