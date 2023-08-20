use thiserror::Error;
use tracing::error;

use crate::{
    model::{EntryId, Model, Value},
    AdapterCommand, GameAdapter, UpdateEvent,
};
use std::{
    collections::VecDeque,
    io::ErrorKind,
    net::UdpSocket,
    result,
    sync::{
        mpsc::{self, Receiver, TryRecvError},
        Arc, RwLock,
    },
    time::{Duration, Instant},
};

use self::{
    data::{IncompleteTypeError, Message},
    processors::{
        base::BaseProcessor, connection::ConnectionProcessor, lap::LapProcessor, process_message,
        AccProcessor, AccProcessorContext,
    },
};

use super::common::{distance_driven, entry_finished};

mod data;
pub mod model;
mod processors;

/// A specialized result for Connection errors.
pub type Result<T> = result::Result<T, crate::AdapterError>;

#[derive(Debug, Error)]
pub enum AccConnectionError {
    #[error("Io error: {0}")]
    IoError(std::io::Error),
    #[error("Error writing to udp socket: {0}")]
    CannotSend(std::io::Error),
    #[error("Error receiving data: {0}")]
    CannotReceive(std::io::Error),
    #[error("Cannot parse message: {0}")]
    CannotParse(IncompleteTypeError),
    #[error("Connection to the game timed out")]
    TimedOut,
    #[error("Game connection is not available")]
    SocketUnavailable,
    #[error("Game refused the connection. Reson: {message}")]
    ConnectionRefused { message: String },
    #[error("Connection encountered an error: {0}")]
    Other(String),
}

impl From<AccConnectionError> for crate::AdapterError {
    fn from(value: AccConnectionError) -> Self {
        crate::AdapterError::ACC(value)
    }
}

pub struct AccAdapter;
impl GameAdapter for AccAdapter {
    fn run(
        &mut self,
        model: Arc<RwLock<Model>>,
        command_rx: mpsc::Receiver<AdapterCommand>,
        update_event: UpdateEvent,
    ) -> result::Result<(), crate::AdapterError> {
        let mut connection = AccConnection::new(model.clone(), command_rx, update_event)?;

        // Setup the model state for this game.
        if let Ok(mut model) = model.write() {
            model.event_name = Value::new("Assetto Corsa Competizione".to_string()).with_editable();
            model.connected = true;
        }

        let result = connection.run_loop();

        if let Ok(mut model) = model.write() {
            model.connected = false;
        }

        result
    }
}

pub struct AccConnection {
    model: Arc<RwLock<Model>>,
    command_rx: Receiver<AdapterCommand>,
    update_event: UpdateEvent,
    socket: AccSocket,
    base_proc: BaseProcessor,
    connection_proc: ConnectionProcessor,
    lap_proc: LapProcessor,
}

impl AccConnection {
    pub fn new(
        model: Arc<RwLock<Model>>,
        command_rx: mpsc::Receiver<AdapterCommand>,
        update_event: UpdateEvent,
    ) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(AccConnectionError::IoError)?;
        socket
            .connect("127.0.0.1:9000")
            .map_err(AccConnectionError::IoError)?;
        socket
            .set_read_timeout(Some(Duration::from_millis(500)))
            .expect("Read timeout duration should be larger than 0");
        Ok(Self {
            model,
            command_rx,
            update_event,
            socket: AccSocket {
                socket,
                connected: false,
                connection_id: 0,
                read_only: false,
            },
            base_proc: BaseProcessor::default(),
            connection_proc: ConnectionProcessor::default(),
            lap_proc: LapProcessor::default(),
        })
    }

    fn run_loop(&mut self) -> Result<()> {
        self.socket.send_registration_request(100, "", "")?;

        let mut last_update = Instant::now();
        loop {
            let now = Instant::now();
            if now.duration_since(last_update).as_secs() > 10 {
                return Err(AccConnectionError::TimedOut.into());
            }

            let should_close = match self.command_rx.try_recv() {
                Ok(action) => self.handle_command(action)?,
                Err(TryRecvError::Empty) => false,
                Err(TryRecvError::Disconnected) => {
                    // This should only happen if all adapters have been dropped.
                    // In which case it is impossible to interact with this adapter any more.
                    // To avoid leaking memory we quit.
                    error!("All adapter handle have been dropped it is impossible to communicate with this game adapter.");
                    true
                }
            };
            if should_close {
                break;
            }

            let message = match self.socket.read_message() {
                Ok(message) => message,
                Err(e) => match e {
                    AccConnectionError::TimedOut => continue,
                    e => return Err(e.into()),
                },
            };
            self.process_message(&message)?;

            // Technically the order of messages put the realtime updates with car information
            // after the session update however we dont have a way to know when all
            // realtime updates have been received to trigger the event.
            // Instead we trigger the event and accept a delay of one update for car data.
            if let Message::SessionUpdate(_) = message {
                self.update_event.trigger();
            }

            last_update = now;
        }

        self.socket.send_unregister_request()?;
        Ok(())
    }

    fn handle_command(&self, command: AdapterCommand) -> Result<bool> {
        match command {
            AdapterCommand::Close => {
                return Ok(true);
            }
            AdapterCommand::FocusOnCar(entry_id) => self
                .socket
                .send_change_camera_request(Some(entry_id.0 as i16), None)?,
            AdapterCommand::ChangeCamera(camera) => {
                let camera = camera.as_acc_camera_definition();
                if camera.is_some() {
                    self.socket.send_change_camera_request(None, camera)?;
                }
            }
        };
        Ok(false)
    }

    fn process_message(&mut self, message: &Message) -> Result<()> {
        let mut context = AccProcessorContext {
            socket: &mut self.socket,
            model: &mut *self
                .model
                .write()
                .map_err(|_| AccConnectionError::Other("Model was poisoned".into()))?,
            events: VecDeque::new(),
        };

        process_message(&mut self.base_proc, message, &mut context)?;
        process_message(&mut self.connection_proc, message, &mut context)?;
        process_message(&mut self.lap_proc, message, &mut context)?;

        if let Message::RealtimeCarUpdate(update) = message {
            let entry = context
                .model
                .current_session_mut()
                .and_then(|session| session.entries.get_mut(&EntryId(update.car_id as i32)));
            if let Some(entry) = entry {
                distance_driven::calc_distance_driven(entry);
            }
        }

        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            self.base_proc.event(&event, &mut context)?;
            self.connection_proc.event(&event, &mut context)?;
            self.lap_proc.event(&event, &mut context)?;

            entry_finished::calc_entry_finished(&event, context.model);
            context.model.events.push(event);
        }

        Ok(())
    }
}

/// A wrapper around a udp socket for easier use.
pub(crate) struct AccSocket {
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
            Err(e) => Err(AccConnectionError::CannotSend(e).into()),
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

    /// Send a unregister request.
    fn send_unregister_request(&self) -> Result<()> {
        self.send(&data::unregister_request(self.connection_id))
    }

    /// Send a change camera request.
    /// If no camera is given this will only change the currently focused car.
    fn send_change_camera_request(
        &self,
        car_id: Option<i16>,
        camera: Option<(&str, &str)>,
    ) -> Result<()> {
        self.send(&data::focus_request(self.connection_id, car_id, camera))
    }

    fn read_message(&mut self) -> std::result::Result<Message, AccConnectionError> {
        let mut buf = [0u8; 2048];
        self.socket.recv(&mut buf).map_err(|e| match e.kind() {
            ErrorKind::TimedOut => AccConnectionError::TimedOut,
            ErrorKind::ConnectionReset => AccConnectionError::SocketUnavailable,
            _ => AccConnectionError::CannotReceive(e),
        })?;

        data::read_response(&buf).map_err(AccConnectionError::CannotParse)
    }
}
