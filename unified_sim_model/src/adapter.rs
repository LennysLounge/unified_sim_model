use std::{
    error::Error,
    fmt::Display,
    sync::{Arc, RwLock},
    thread::JoinHandle,
};

use crate::model::Model;

pub mod acc;

/// A error with the game connection.
#[derive(Debug)]
pub enum ConnectionError {
    ACC(acc::AccConnectionError),
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::ACC(e) => write!(f, "Acc connection error: {}", e),
        }
    }
}

impl Error for ConnectionError {}

/// An adapter to a game.
pub struct Adapter {
    /// The join handle to close the connection thread to the game.
    pub join_handle: JoinHandle<Result<(), ConnectionError>>,
    /// The shared model.
    pub model: Arc<RwLock<Model>>,
    // TODO: channel
}

impl Adapter {
    pub fn new_acc() -> Adapter {
        let model = Arc::new(RwLock::new(Model::default()));
        Adapter {
            join_handle: acc::AccConnection::spawn(model.clone()),
            model: model,
        }
    }
}
