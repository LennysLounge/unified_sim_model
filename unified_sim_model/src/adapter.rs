use std::{
    error::Error,
    fmt::Display,
    sync::{
        mpsc::{self, Sender},
        Arc, PoisonError, RwLock, RwLockReadGuard,
    },
    thread::JoinHandle,
};

use crate::model::Model;

pub mod acc;
pub mod dummy;

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
    join_handle: Option<JoinHandle<Result<(), ConnectionError>>>,
    /// The shared model.
    pub model: ReadOnlyModel,
    /// Channel to send adapter actions to the adapter.
    pub sender: Sender<AdapterAction>,
}

impl Adapter {
    /// Create a new dummy adapter for testing.
    pub fn new_dummy() -> Adapter {
        let model = Arc::new(RwLock::new(Model::default()));
        let (sender, _receiver) = mpsc::channel();
        Adapter {
            join_handle: Some(dummy::DummyAdapter::spawn(model.clone())),
            model: ReadOnlyModel::new(model),
            sender,
        }
    }

    /// Create a new assetto corsa competizione adapter.
    pub fn new_acc() -> Adapter {
        let model = Arc::new(RwLock::new(Model::default()));
        let (sender, _receiver) = mpsc::channel();
        Adapter {
            join_handle: Some(acc::AccConnection::spawn(model.clone())),
            model: ReadOnlyModel::new(model),
            sender,
        }
    }

    /// Returns `true` if the adapter has finised its connection to the game
    pub fn is_finished(&self) -> bool {
        match &self.join_handle {
            Some(handle) => handle.is_finished(),
            None => true,
        }
    }

    /// Joins the adapter thread and returns the result.
    ///
    /// The result is only returned the first time this method is called after
    /// the thread has finished. Calling it before the thread is finished will return
    /// `None`. Calling it after the result has been taking will also return `None`.
    pub fn take_result(&mut self) -> Option<Result<(), ConnectionError>> {
        if !self.is_finished() {
            return None;
        }
        self.join_handle
            .take()
            .map(|join_handle| join_handle.join().expect("Should be able to join thread"))
    }
}

/// A readonly view on a model.
/// To read the model it must first be locked. Locking follows all the same
/// rules as a `read` method in `RwLock`.
#[derive(Clone)]
pub struct ReadOnlyModel {
    model: Arc<RwLock<Model>>,
}

impl ReadOnlyModel {
    /// Creates a new read only model.
    fn new(model: Arc<RwLock<Model>>) -> Self {
        Self { model }
    }
    /// Locks the underlying `RwLock` and returns a read only view to the model.
    pub fn read(
        &self,
    ) -> Result<RwLockReadGuard<'_, Model>, PoisonError<RwLockReadGuard<'_, Model>>> {
        self.model.read()
    }
}

/// Actions for the adapter to execute.
pub enum AdapterAction {
    ClearEvents,
}
