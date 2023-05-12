use std::{
    error::Error,
    fmt::Display,
    io::ErrorKind,
    net::UdpSocket,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{
    messages::{self, read_response, IncompleteTypeError, Message},
    model::Model,
};

#[derive(Debug)]
pub enum ConnectionError {
    CannotSend(std::io::Error),
    CannotReceive(std::io::Error),
    CannotParse(IncompleteTypeError),
    TimedOut,
    SocketUnavailable,
    ConnectionRefused { message: String },
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
        }
    }
}

impl Error for ConnectionError {}

pub struct AccAdapter {
    pub join_handle: JoinHandle<Result<(), ConnectionError>>,
    pub model: Arc<RwLock<Model>>,
    // channel
}

impl AccAdapter {
    pub fn new() -> Result<AccAdapter, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect("127.0.0.1:9000")?;
        socket
            .set_read_timeout(Some(Duration::from_millis(2000)))
            .expect("Read timeout duration should be larger than 0");
        let model = Arc::new(RwLock::new(Model::default()));

        Ok(AccAdapter {
            join_handle: start_acc_connection_thread(socket, model.clone()),
            model,
        })
    }
}

fn start_acc_connection_thread(
    socket: UdpSocket,
    model: Arc<RwLock<Model>>,
) -> JoinHandle<Result<(), ConnectionError>> {
    let mut connection = AccConnection {
        socket,
        model,
        connected: false,
        connection_id: 0,
        read_only: false,
    };
    thread::spawn(move || connection.run_connection())
}

struct AccConnection {
    socket: UdpSocket,
    model: Arc<RwLock<Model>>,
    connected: bool,
    connection_id: i32,
    read_only: bool,
}

impl AccConnection {
    fn run_connection(&mut self) -> Result<(), ConnectionError> {
        self.send(&messages::register_request("", 1000, ""))?;

        loop {
            println!("Read messages until it would block");

            let messages = self.read_availabe_messages()?;
            self.process_messages(messages)?;

            println!("All messages received!");
            println!("Processing now");
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn process_messages(&mut self, messages: Vec<Message>) -> Result<(), ConnectionError> {
        let mut model = self.model.write().expect("RwLock should not be poisoned");

        for message in messages {
            match message {
                messages::Message::Unknown(_) => todo!(),
                messages::Message::RegistrationResult(result) => {
                    if !result.success {
                        return Err(ConnectionError::ConnectionRefused {
                            message: result.message,
                        });
                    }
                    self.connected = true;
                    self.connection_id = result.connection_id;
                    self.read_only = result.read_only;

                    self.send(&messages::entry_list_request(self.connection_id))?;
                    self.send(&messages::track_data_request(self.connection_id))?;
                }
                messages::Message::RealtimeUpdate(_) => println!("RealtimeUpdate"),
                messages::Message::RealtimeCarUpdate(_) => println!("RealtimeCarUpdate"),
                messages::Message::EntryList(_) => println!("EntryList"),
                messages::Message::TrackData(track_data) => {
                    model.track_name = track_data.track_name;
                    model.track_length = track_data.track_meter;
                    // TODO: Save track data to internal model.
                }
                messages::Message::EntryListCar(_) => println!("EntryListCar"),
                messages::Message::BroadcastingEvent(_) => println!("BroadcastingEvent"),
            }
        }
        //addition processing
        Ok(())
    }

    fn read_availabe_messages(&mut self) -> Result<Vec<messages::Message>, ConnectionError> {
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

            messages.push(read_response(&buf).map_err(ConnectionError::CannotParse)?);
        }
        Ok(messages)
    }

    fn send(&self, buf: &[u8]) -> Result<(), ConnectionError> {
        match self.socket.send(buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(ConnectionError::CannotSend(e)),
        }
    }
}
