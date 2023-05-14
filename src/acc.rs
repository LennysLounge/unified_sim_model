use crate::model::Model;
use std::{
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

use self::{
    base_processor::BaseProcessor,
    data::{
        BroadcastingEvent, EntryList, EntryListCar, IncompleteTypeError, Message,
        RealtimeCarUpdate, RealtimeUpdate, RegistrationResult, TrackData,
    },
};

mod base_processor;
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
            socket: AccSocket {
                socket,
                connected: false,
                connection_id: 0,
                read_only: false,
            },
            model: model.clone(),
            base_processor: base_processor::BaseProcessor::default(),
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
    base_processor: BaseProcessor,
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
        let model = self
            .model
            .write()
            .map_err(|_| ConnectionError::Other("Model was poisoned".into()))?;

        let mut context = AccProcessorContext {
            socket: &mut self.socket,
            model,
        };

        for message in messages {
            use Message::*;
            match message {
                Unknown(t) => {
                    return Err(ConnectionError::Other(format!(
                        "Unknown message type: {}",
                        t
                    )));
                }
                RegistrationResult(result) => self
                    .base_processor
                    .registration_result(&result, &mut context)?,
                RealtimeUpdate(update) => {
                    self.base_processor.realtime_update(&update, &mut context)?
                }
                RealtimeCarUpdate(update) => self
                    .base_processor
                    .realtime_car_update(&update, &mut context)?,
                EntryList(list) => self.base_processor.entry_list(&list, &mut context)?,
                TrackData(track) => self.base_processor.track_data(&track, &mut context)?,
                EntryListCar(car) => self.base_processor.entry_list_car(&car, &mut context)?,
                BroadcastingEvent(event) => {
                    self.base_processor.broadcast_even(&event, &mut context)?
                }
            }
        }

        debug!("Additional processing now");
        self.base_processor.post_update(&mut context)?;
        if let Some(session) = context.model.current_session() {
            info!(
                "Session time: {}, Session time remaining: {}",
                session.session_time, session.time_remaining
            );
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

/// A context for a processor to work in.
struct AccProcessorContext<'a> {
    socket: &'a mut AccSocket,
    model: RwLockWriteGuard<'a, Model>,
}

/// This trait descibes a processor that can process the
/// data events from the game and modify the model.
trait AccUpdateProcessor {
    #[allow(unused_variables)]
    fn registration_result(
        &mut self,
        result: &RegistrationResult,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn realtime_update(
        &mut self,
        update: &RealtimeUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn entry_list(&mut self, list: &EntryList, context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn track_data(&mut self, track: &TrackData, context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn entry_list_car(
        &mut self,
        car: &EntryListCar,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn broadcast_even(
        &mut self,
        event: &BroadcastingEvent,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    /// This method is run after all other messages have been processed.
    #[allow(unused_variables)]
    fn post_update(&mut self, context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }
}
