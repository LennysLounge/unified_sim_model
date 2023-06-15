use thiserror::Error;
use tracing::{error};

use crate::{
    model::{EntryId, Event, Model},
    AdapterCommand, GameAdapter, UpdateEvent,
};
use std::{
    collections::VecDeque,
    io::ErrorKind,
    net::UdpSocket,
    result,
    sync::{
        mpsc::{self, TryRecvError},
        Arc, RwLock, RwLockWriteGuard,
    },
    time::Duration,
};

use self::{
    data::{
        BroadcastingEvent, EntryList, EntryListCar, IncompleteTypeError, Message,
        RealtimeCarUpdate, RegistrationResult, SessionUpdate, TrackData,
    },
    processors::{base::BaseProcessor, connection::ConnectionProcessor, lap::LapProcessor},
};

use super::common::distance_driven;

mod data;
mod processors;

/// A specialized result for Connection errors.
type Result<T> = result::Result<T, crate::AdapterError>;

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

pub struct AccAdapter {
    socket: AccSocket,
    base_proc: BaseProcessor,
    connection_proc: ConnectionProcessor,
    lap_proc: LapProcessor,
}

impl GameAdapter for AccAdapter {
    fn run(
        &mut self,
        model: Arc<RwLock<Model>>,
        command_rx: mpsc::Receiver<AdapterCommand>,
        update_event: &UpdateEvent,
    ) -> result::Result<(), crate::AdapterError> {
        self.socket.send_registration_request(100, "", "")?;

        loop {
            match command_rx.try_recv() {
                Ok(action) => match action {
                    AdapterCommand::Close => {
                        break;
                    }
                },
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => error!("The adapter sender has disappeared"),
            }

            let message = self.socket.read_message()?;
            self.process_message(&message, &model)?;

            // Technically the order of messages put the realtime updates with car information
            // after the session update however we dont have a way to know when all
            // realtime updates have been received to trigger the event.
            // Instead we trigger the event and accept a delay of one update for car data.
            if let Message::SessionUpdate(_) = message {
                update_event.trigger();
            }
        }

        self.socket.send_unregister_request()
    }
}

impl AccAdapter {
    pub fn new() -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| AccConnectionError::IoError(e))?;
        socket
            .connect("127.0.0.1:9000")
            .map_err(|e| AccConnectionError::IoError(e))?;
        socket
            .set_read_timeout(Some(Duration::from_millis(500)))
            .expect("Read timeout duration should be larger than 0");
        Ok(Self {
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

    fn process_message(&mut self, message: &Message, model: &Arc<RwLock<Model>>) -> Result<()> {
        let mut context = AccProcessorContext {
            socket: &mut self.socket,
            model: model
                .write()
                .map_err(|_| AccConnectionError::Other("Model was poisoned".into()))?,
            events: VecDeque::new(),
        };

        process_message(&mut self.base_proc, &message, &mut context)?;
        process_message(&mut self.connection_proc, &message, &mut context)?;
        process_message(&mut self.lap_proc, &message, &mut context)?;

        match message {
            Message::RealtimeCarUpdate(update) => distance_driven::calc_distance_driven(
                &mut context.model,
                &EntryId(update.car_id as i32),
            ),
            _ => (),
        }

        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            self.base_proc.event(&event, &mut context)?;
            self.connection_proc.event(&event, &mut context)?;
            self.lap_proc.event(&event, &mut context)?;
            context.model.events.push(event);
        }

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

    fn read_message(&mut self) -> Result<Message> {
        let mut buf = [0u8; 2048];
        self.socket.recv(&mut buf).map_err(|e| match e.kind() {
            ErrorKind::TimedOut => AccConnectionError::TimedOut,
            ErrorKind::ConnectionReset => AccConnectionError::SocketUnavailable,
            _ => AccConnectionError::CannotReceive(e),
        })?;

        data::read_response(&buf).map_err(|e| AccConnectionError::CannotParse(e).into())
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
        Unknown(t) => Err(AccConnectionError::Other(format!("Unknown message type: {}", t)).into()),
        RegistrationResult(ref result) => me.registration_result(result, context),
        SessionUpdate(ref update) => me.session_update(update, context),
        RealtimeCarUpdate(ref update) => me.realtime_car_update(update, context),
        EntryList(ref list) => me.entry_list(list, context),
        TrackData(ref track) => me.track_data(track, context),
        EntryListCar(ref car) => me.entry_list_car(car, context),
        BroadcastingEvent(ref event) => me.broadcast_event(event, context),
    }
}
