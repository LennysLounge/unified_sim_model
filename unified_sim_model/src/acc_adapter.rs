use crate::model::{Event, Model};
use std::{
    collections::VecDeque,
    error::Error,
    fmt::Display,
    io::ErrorKind,
    net::UdpSocket,
    result,
    sync::{Arc, RwLock, RwLockWriteGuard},
    thread::{self, JoinHandle},
    time::Duration,
};

use self::{
    data::{
        BroadcastingEvent, EntryList, EntryListCar, IncompleteTypeError, Message,
        RealtimeCarUpdate, RegistrationResult, SessionUpdate, TrackData,
    },
    processors::{base::BaseProcessor, connection::ConnectionProcessor, lap::LapProcessor},
};

pub mod data;
mod processors;

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
    /// The join handle to close the connection thread to the game.
    pub join_handle: JoinHandle<Result<()>>,
    /// The shared model.
    pub model: Arc<RwLock<Model>>,
    // TODO: channel
}

impl AccAdapter {
    pub fn new() -> result::Result<AccAdapter, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect("127.0.0.1:9000")?;
        socket
            .set_read_timeout(Some(Duration::from_millis(2000)))
            .expect("Read timeout duration should be larger than 0");
        let model = Arc::new(RwLock::new(Model::default()));

        Ok(AccAdapter {
            join_handle: AccConnection::spawn(socket, model.clone()),
            model,
        })
    }
}

struct AccConnection {
    socket: AccSocket,
    model: Arc<RwLock<Model>>,
    base_proc: BaseProcessor,
    connection_proc: ConnectionProcessor,
    lap_proc: LapProcessor,
}

impl AccConnection {
    fn spawn(socket: UdpSocket, model: Arc<RwLock<Model>>) -> JoinHandle<Result<()>> {
        thread::Builder::new()
            .name("Acc connection".into())
            .spawn(move || {
                let mut connection = Self::new(socket, model);
                connection.run_connection()
            })
            .expect("should be able to spawn thread")
    }

    fn new(socket: UdpSocket, model: Arc<RwLock<Model>>) -> Self {
        Self {
            socket: AccSocket {
                socket,
                connected: false,
                connection_id: 0,
                read_only: false,
            },
            model,
            base_proc: BaseProcessor::default(),
            connection_proc: ConnectionProcessor::default(),
            lap_proc: LapProcessor::default(),
        }
    }

    fn run_connection(&mut self) -> Result<()> {
        self.socket.send_registration_request(1000, "", "")?;

        loop {
            // TODO: read channel

            let message = self.socket.read_message()?;
            self.process_message(message)?;
        }
    }

    fn process_message(&mut self, message: Message) -> Result<()> {
        let mut context = AccProcessorContext {
            socket: &mut self.socket,
            model: self
                .model
                .write()
                .map_err(|_| ConnectionError::Other("Model was poisoned".into()))?,
            events: VecDeque::new(),
        };

        process_message(&mut self.base_proc, &message, &mut context)?;
        process_message(&mut self.connection_proc, &message, &mut context)?;
        process_message(&mut self.lap_proc, &message, &mut context)?;

        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            self.base_proc.event(&event, &mut context)?;
            self.connection_proc.event(&event, &mut context)?;
            self.lap_proc.event(&event, &mut context)?;
            context.model.events.push(event);
        }

        //addition processing
        Ok(())
    }
}

/// A wrapper around a udp socket for easier use.
struct AccSocket {
    connected: bool,
    connection_id: i32,
    read_only: bool,
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

    /// Send a entry list request.
    fn send_entry_list_request(&self) -> Result<()> {
        self.send(&data::entry_list_request(self.connection_id))
    }

    /// Send a track data request.
    fn send_track_data_request(&self) -> Result<()> {
        self.send(&data::track_data_request(self.connection_id))
    }

    fn read_message(&mut self) -> Result<Message> {
        let mut buf = [0u8; 2048];
        self.socket.recv(&mut buf).map_err(|e| match e.kind() {
            ErrorKind::TimedOut => ConnectionError::TimedOut,
            ErrorKind::ConnectionReset => ConnectionError::SocketUnavailable,
            _ => ConnectionError::CannotReceive(e),
        })?;

        data::read_response(&buf).map_err(ConnectionError::CannotParse)
    }
}

/// A context for a processor to work in.
struct AccProcessorContext<'a> {
    socket: &'a mut AccSocket,
    model: RwLockWriteGuard<'a, Model>,
    events: VecDeque<Event>,
}

/// This trait descibes a processor that can process the
/// data events from the game and modify the model.
trait AccProcessor {
    fn registration_result(
        &mut self,
        _result: &RegistrationResult,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn session_update(
        &mut self,
        _update: &SessionUpdate,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn realtime_car_update(
        &mut self,
        _update: &RealtimeCarUpdate,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn entry_list(&mut self, _list: &EntryList, _context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    fn track_data(&mut self, _track: &TrackData, _context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    fn entry_list_car(
        &mut self,
        _car: &EntryListCar,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn broadcast_event(
        &mut self,
        _event: &BroadcastingEvent,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn event(&mut self, _event: &Event, _context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }
}

fn process_message(
    me: &mut impl AccProcessor,
    message: &Message,
    context: &mut AccProcessorContext,
) -> Result<()> {
    use Message::*;
    match message {
        Unknown(t) => Err(ConnectionError::Other(format!(
            "Unknown message type: {}",
            t
        ))),
        RegistrationResult(ref result) => me.registration_result(result, context),
        SessionUpdate(ref update) => me.session_update(update, context),
        RealtimeCarUpdate(ref update) => me.realtime_car_update(update, context),
        EntryList(ref list) => me.entry_list(list, context),
        TrackData(ref track) => me.track_data(track, context),
        EntryListCar(ref car) => me.entry_list_car(car, context),
        BroadcastingEvent(ref event) => me.broadcast_event(event, context),
    }
}