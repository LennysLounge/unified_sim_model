use crate::{
    log_todo,
    model::{self, Entry, Model, Time},
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    io::ErrorKind,
    net::UdpSocket,
    result,
    sync::{Arc, RwLock, RwLockWriteGuard},
    thread::{self, JoinHandle},
    time::Duration,
};
use tracing::{debug, info};

use self::data::{
    EntryListCar, IncompleteTypeError, Message, RealtimeUpdate, RegistrationResult, SessionPhase,
    SessionType,
};

pub mod data;

/// A specialized result for Connection errors.
pub type Result<T> = result::Result<T, ConnectionError>;

#[derive(Debug)]
pub enum ConnectionError {
    CannotSend(std::io::Error),
    CannotReceive(std::io::Error),
    CannotParse(IncompleteTypeError),
    TimedOut,
    SocketUnavailable,
    ConnectionRefused { message: String },
    Other(String),
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::CannotSend(e) => write!(f, "Error writing to udp socket: {}", e),
            ConnectionError::CannotReceive(e) => write!(f, "Error receiving data: {}", e),
            ConnectionError::CannotParse(e) => write!(f, "Cannot parse message: {}", e),
            ConnectionError::TimedOut => write!(f, "Connection to game timed out"),
            ConnectionError::SocketUnavailable => write!(f, "Game connection is not available"),
            ConnectionError::ConnectionRefused { message } => {
                write!(f, "Game refused the connection. Reason: {}", message)
            }
            ConnectionError::Other(message) => {
                write!(f, "Connection encountered an error: {}", message)
            }
        }
    }
}

impl Error for ConnectionError {}

/// An adapter for Assetto corsa competizione.
pub struct AccAdapter {
    pub join_handle: JoinHandle<Result<()>>,
    pub model: Arc<RwLock<Model>>,
    // channel
}

impl AccAdapter {
    pub fn new() -> result::Result<AccAdapter, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect("127.0.0.1:9000")?;
        socket
            .set_read_timeout(Some(Duration::from_millis(2000)))
            .expect("Read timeout duration should be larger than 0");
        let model = Arc::new(RwLock::new(Model::default()));

        let mut connection = AccConnection {
            socket: AccSocket { socket },
            model: model.clone(),
            inner_model: InnerModel::default(),
        };

        Ok(AccAdapter {
            join_handle: thread::Builder::new()
                .name("Acc connection".into())
                .spawn(move || connection.run_connection())
                .expect("should be able to spawn thread"),
            model,
        })
    }
}

struct AccConnection {
    socket: AccSocket,
    model: Arc<RwLock<Model>>,
    inner_model: InnerModel,
}

impl AccConnection {
    fn run_connection(&mut self) -> Result<()> {
        self.socket.send_registration_request(1000, "", "")?;

        loop {
            debug!("Read messages");
            let messages = self.socket.read_availabe_messages()?;

            debug!("process messages");
            self.process_messages(messages)?;
        }
    }

    fn process_messages(&mut self, messages: Vec<Message>) -> Result<()> {
        let mut model = self
            .model
            .write()
            .map_err(|_| ConnectionError::Other("Model was poisoned".into()))?;

        for message in messages {
            match message {
                Message::Unknown(t) => {
                    return Err(ConnectionError::Other(format!(
                        "Unknown message type: {}",
                        t
                    )));
                }
                Message::RegistrationResult(result) => self
                    .inner_model
                    .process_registration_result(result, &self.socket)?,
                Message::RealtimeUpdate(update) => self
                    .inner_model
                    .process_realtime_update(update, &mut model)?,
                Message::RealtimeCarUpdate(_) => info!("RealtimeCarUpdate"),
                Message::EntryList(_) => info!("EntryList"),
                Message::TrackData(track_data) => {
                    model.track_name = track_data.track_name;
                    model.track_length = track_data.track_meter;
                }
                Message::EntryListCar(message) => self
                    .inner_model
                    .process_entry_list_car(message, &mut model)?,
                Message::BroadcastingEvent(_) => info!("BroadcastingEvent"),
            }
        }

        debug!("Additional processing now");
        if !model.sessions.is_empty() {
            let current_session_index = model.current_session;
            let session = &model.sessions[current_session_index];

            info!(
                "Session time: {}, Session time remaining: {}",
                session.session_time, session.time_remaining
            );
        }

        //addition processing
        Ok(())
    }
}

#[derive(Default)]
struct InnerModel {
    connected: bool,
    connection_id: i32,
    read_only: bool,
    current_session_index: i16,
}

impl InnerModel {
    fn process_registration_result(
        &mut self,
        result: RegistrationResult,
        socket: &AccSocket,
    ) -> Result<()> {
        debug!("Registration result");
        if !result.success {
            return Err(ConnectionError::ConnectionRefused {
                message: result.message,
            });
        }
        self.connected = true;
        self.connection_id = result.connection_id;
        self.read_only = result.read_only;

        socket.send(&data::entry_list_request(self.connection_id))?;
        socket.send(&data::track_data_request(self.connection_id))?;
        Ok(())
    }

    fn process_realtime_update(
        &mut self,
        update: RealtimeUpdate,
        model: &mut RwLockWriteGuard<Model>,
    ) -> Result<()> {
        debug!("RealtimeUpdate");

        if self.current_session_index != update.session_index || model.sessions.is_empty() {
            info!("New session detected");
            // A new session has started.
            let session = model::Session {
                id: model.sessions.len() as i32,
                session_type: convert_session_type(update.session_type),
                session_time: Time::from(update.session_end_time + update.session_time),
                time_remaining: Time::from(update.session_end_time),
                phase: convert_session_phase(update.session_phase.clone()),
                time_of_day: Time::from(update.time_of_day * 1000.0),
                ambient_temp: update.ambient_temp as f32,
                track_temp: update.track_temp as f32,
                ..Default::default()
            };
            model.current_session = model.sessions.len();
            model.sessions.push(session);
            self.current_session_index = update.session_index;
        }

        let current_session_index = model.current_session;
        let current_session = &mut model.sessions[current_session_index];
        current_session.time_remaining = Time::from(update.session_end_time);

        let current_phase = convert_session_phase(update.session_phase);
        if current_phase != current_session.phase {
            info!(
                "Session phase changed from {:?} to {:?}",
                current_session.phase, current_phase
            );
            current_session.phase = current_phase;
        }
        current_session.time_of_day = Time::from(update.time_of_day * 1000.0);
        current_session.ambient_temp = update.ambient_temp as f32;
        current_session.track_temp = update.track_temp as f32;
        Ok(())
    }

    fn process_entry_list_car(
        &self,
        entry_list_car: EntryListCar,
        model: &mut RwLockWriteGuard<Model>,
    ) -> Result<()> {
        debug!("EntryListCar");
        let current_session_index = model.current_session;
        if model.sessions[current_session_index]
            .entries
            .contains_key(&(entry_list_car.car_id as i32))
        {
            return Ok(());
        }
        info!("New entry has connected: #{}", entry_list_car.race_number);
        let entry = Entry {
            id: entry_list_car.car_id as i32,
            drivers: log_todo(HashMap::new(), "Create drivers for entry"),
            current_driver: entry_list_car.current_driver_index as i32,
            team_name: entry_list_car.team_name,
            car: log_todo(model::CarModel::None, "Convert car model to enum"),
            car_number: entry_list_car.race_number,
            nationality: entry_list_car.car_nationality,
            ..Default::default()
        };

        model.sessions[current_session_index]
            .entries
            .insert(entry.id, entry);

        Ok(())
    }
}

/// A wrapper around a udp socket for easier use.
struct AccSocket {
    socket: UdpSocket,
}

impl AccSocket {
    /// Send a message to the game.
    fn send(&self, buf: &[u8]) -> Result<()> {
        match self.socket.send(buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(ConnectionError::CannotSend(e)),
        }
    }

    /// Send a registration request.
    fn send_registration_request(
        &self,
        update_interval: i32,
        password: &str,
        command_password: &str,
    ) -> Result<()> {
        self.send(&data::register_request(
            password,
            update_interval,
            command_password,
        ))
    }

    /// Read all available messages and return them as a list.
    /// Will block until a message in available and then
    /// read all available messages.
    fn read_availabe_messages(&mut self) -> Result<Vec<Message>> {
        let mut buf = [0u8; 2048];

        // Set blocking to block until a new message is available.
        self.socket
            .set_nonblocking(false)
            .expect("Should be able to set nonblocking");

        let mut messages = Vec::new();
        loop {
            if let Err(e) = self.socket.recv(&mut buf) {
                match e.kind() {
                    ErrorKind::WouldBlock => break,
                    ErrorKind::TimedOut => return Err(ConnectionError::TimedOut),
                    ErrorKind::ConnectionReset => return Err(ConnectionError::SocketUnavailable),
                    _ => return Err(ConnectionError::CannotReceive(e)),
                }
            }
            // Set nonblocking to read all messages until the first time it would block, then return.
            self.socket.set_nonblocking(true).unwrap();

            messages.push(data::read_response(&buf).map_err(ConnectionError::CannotParse)?);
        }
        Ok(messages)
    }
}

fn convert_session_phase(value: data::SessionPhase) -> model::SessionPhase {
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

fn convert_session_type(value: SessionType) -> model::SessionType {
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
