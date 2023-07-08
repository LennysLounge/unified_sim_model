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
    live_data::{self},
    static_data::{self, DriverInfo, MaybeUnlimited, WeekendInfo, WeekendOptions},
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
    last_static_data_update: i32,
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
            last_static_data_update: -1,
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

        // Create sessions
        if data
            .static_data
            .session_info
            .as_ref()
            .is_some_and(|s| s.sessions.len() != model.sessions.len())
        {
            let sessions = &data.static_data.session_info.as_ref().unwrap().sessions;
            for session_info in sessions {
                let Some(session_num) = session_info.session_num else {break;};
                let session_id = model::SessionId(session_num as usize);
                model.sessions.insert(
                    session_id,
                    model::Session {
                        id: session_id,
                        game_data: SessionGameData::None,
                        ..Default::default()
                    },
                );
            }
        }

        // Update static data
        if data.static_data.update_count != self.last_static_data_update {
            self.last_static_data_update = data.static_data.update_count;

            let sessions = &data.static_data.session_info.as_ref().unwrap().sessions;
            for session_info in sessions {
                let Some(session_num) = session_info.session_num else {break;};
                let session_id = model::SessionId(session_num as usize);
                let Some(ref mut session) = model.sessions.get_mut(&session_id) else {break;};

                update_session_static(session, &data.static_data, session_info);
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

        // Update session.
        let mut current_session = model
            .current_session_mut()
            .expect("The current session should be available");
        update_session_live(&mut current_session, &data.live_data);

        Ok(())
    }
}

fn update_session_static(
    session: &mut model::Session,
    data: &static_data::StaticData,
    session_info: &static_data::Session,
) {
    if let Some(ref session_type_str) = session_info.session_type {
        session.session_type.set(map_session_type(session_type_str));
    }

    if let Some(ref session_time) = session_info.session_time {
        match session_time {
            MaybeUnlimited::Unlimited => session.session_time.set_unavailable(),
            MaybeUnlimited::Value(t) => session.session_time.set(t.clone()),
        }
    }

    if let Some(ref session_laps) = session_info.session_laps {
        match session_laps {
            MaybeUnlimited::Unlimited => session.laps.set_unavailable(),
            MaybeUnlimited::Value(laps) => session.laps.set(laps.clone()),
        }
    }

    if let Some(WeekendInfo {
        weekend_options:
            Some(WeekendOptions {
                time_of_day: Some(ref time_of_day),
                ..
            }),
        ..
    }) = data.weekend_info
    {
        session.time_of_day.set(time_of_day.clone());
    }

    if let Some(WeekendInfo {
        track_name: Some(ref track_name),
        ..
    }) = data.weekend_info
    {
        session.track_name.set(track_name.clone());
    }

    if let Some(WeekendInfo {
        track_length: Some(ref track_length),
        ..
    }) = data.weekend_info
    {
        session.track_length.set(track_length.clone());
    }

    if let Some(WeekendInfo {
        track_surface_temp: Some(ref track_temp),
        ..
    }) = data.weekend_info
    {
        session.track_temp.set(track_temp.clone());
    }

    if let Some(WeekendInfo {
        track_air_temp: Some(ref ambient_temp),
        ..
    }) = data.weekend_info
    {
        session.ambient_temp.set(ambient_temp.clone());
    }

    // Create entries
    if let static_data::StaticData {
        weekend_info:
            Some(WeekendInfo {
                weekend_options:
                    Some(WeekendOptions {
                        num_starters: Some(ref num_starters),
                        ..
                    }),
                ..
            }),
        driver_info:
            Some(DriverInfo {
                drivers: ref driver_infos,
                ..
            }),
        ..
    } = data
    {
        if session.entries.len() != *num_starters as usize {
            for driver_info in driver_infos {
                let Some(team_id) = driver_info.team_id else {break;};
                let entry_id = model::EntryId(team_id);
                if !session.entries.contains_key(&entry_id) {
                    let Some(entry) = map_entry(driver_info) else {break;};
                    session.entries.insert(entry.id, entry);
                }

                let Some(entry) = session.entries.get_mut(&entry_id) else {break};
                let Some(driver) = map_driver(driver_info) else {break;};
                entry.drivers.insert(driver.id, driver);
            }
        }
    }
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

fn map_driver(driver_info: &static_data::Driver) -> Option<model::Driver> {
    let driver_name = driver_info.user_name.clone()?;
    let (first_name, last_name) = driver_name.split_once(" ")?;
    Some(model::Driver {
        id: model::DriverId(driver_info.car_idx?),
        first_name: first_name.to_owned().into(),
        last_name: last_name.to_owned().into(),
        short_name: model::Value::default(),
        nationality: model::Value::default(),
        driving_time: model::Value::default(),
        best_lap: model::Value::default(),
    })
}

fn map_entry(driver_info: &static_data::Driver) -> Option<model::Entry> {
    Some(model::Entry {
        id: model::EntryId(driver_info.team_id?),
        drivers: HashMap::new(),
        current_driver: None,
        team_name: driver_info.team_name.clone()?.into(),
        car: model::Car::new(
            driver_info.car_screen_name.to_owned()?,
            "".to_owned(),
            CarCategory::new(""),
        )
        .into(),
        car_number: driver_info.car_number_raw?.into(),
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
