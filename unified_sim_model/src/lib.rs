use games::{acc, dummy::DummyAdapter};
use thiserror::Error;
use tracing::warn;

use std::{
    sync::{mpsc, Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread::{self, JoinHandle},
};

pub mod broadcast;
pub mod games;
pub mod model;
pub mod time;

pub use time::Time;

use crate::model::Model;

#[allow(dead_code)]
fn log_todo<T>(v: T, message: &str) -> T {
    warn!("TODO: {message}");
    v
}

/// The base trait that has to be implemented by a game adapter.
pub trait GameAdapter {
    /// Run the game connection and read data from the game.
    /// This method is run inside a thread. When this method returns
    /// the associated thread finishes.
    ///
    /// * `command_rx` The adapter must react to adapter commands published on this channel
    /// to the best of its abilities. It is not expected for an adapter to correctly react to
    /// every command since not every game provides the interface to implement every command.
    ///
    /// * `update_tx` The adapter should publish update events on this channel to allow
    /// a user of the adapter to react to changes in the model without having to scan for changes themself.
    ///  The update level should be the smallest possible whenever possible.
    fn run(
        &mut self,
        model: Arc<RwLock<Model>>,
        command_rx: mpsc::Receiver<AdapterCommand>,
        update_tx: broadcast::Sender<ModelUpdate>,
    ) -> Result<(), AdapterError>;
}

/// A error with the game adapter.
#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("Acc connection error")]
    ACC(acc::AccConnectionError),
}

/// An adapter to a game.
pub struct Adapter {
    /// The data model that is shared with the game adapter.
    pub model: ReadOnlyModel,
    /// The join handle to close the connection thread to the game.
    join_handle: Option<JoinHandle<Result<(), AdapterError>>>,
    /// Channel for sending commands to the game.
    command_tx: mpsc::Sender<AdapterCommand>,
    /// Channel for sending update events to the user.
    _update_rx: broadcast::Receiver<ModelUpdate>,
}

impl Adapter {
    /// Create a new adapter with a game adapter.
    pub fn new(game: impl GameAdapter + Send + 'static) -> Self {
        let model = Arc::new(RwLock::new(Model::default()));
        let (command_tx, command_rx) = mpsc::channel();
        let (update_tx, update_rx) = broadcast::channel();
        Self {
            model: ReadOnlyModel::new(model.clone()),
            join_handle: Some(Self::spawn(game, model, command_rx, update_tx)),
            command_tx,
            _update_rx: update_rx,
        }
    }
    /// Create a new dummy adapter.
    /// The adapter will write some data into the model and immediately finish.
    pub fn new_dummy() -> Adapter {
        Self::new(DummyAdapter::new())
    }

    pub fn new_acc() -> Result<Adapter, Box<dyn std::error::Error>> {
        Ok(Self::new(acc::AccAdapter::new()?))
    }

    // /// Create a new dummy adapter for testing.
    // pub fn new_dummy() -> Adapter {
    //     let model = Arc::new(RwLock::new(Model::default()));
    //     let (command_tx, _command_rx) = mpsc::channel();
    //     let (_update_tx, _update_rx) = mpsc::channel();
    //     Adapter {
    //         join_handle: Some(dummy::DummyAdapter::spawn(model.clone())),
    //         model: ReadOnlyModel::new(model),
    //         command_tx,
    //         _update_rx,
    //     }
    // }

    // /// Create a new assetto corsa competizione adapter.
    // pub fn new_acc() -> Adapter {
    //     let model = Arc::new(RwLock::new(Model::default()));
    //     let (command_tx, command_rx) = mpsc::channel();
    //     let (_updaet_tx, _update_rx) = mpsc::channel();
    //     Adapter {
    //         join_handle: Some(acc::AccConnection::spawn(model.clone(), command_rx)),
    //         model: ReadOnlyModel::new(model),
    //         command_tx,
    //         _update_rx,
    //     }
    // }

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
    /// the thread has finished. Calling it after the result has been taking will also return `None`.
    pub fn join(&mut self) -> Option<Result<(), AdapterError>> {
        self.join_handle
            .take()
            .map(|join_handle| join_handle.join().expect("Should be able to join thread"))
    }

    /// Clears the current events from the model.
    pub fn clear_events(&mut self) -> Result<(), PoisonError<RwLockWriteGuard<'_, Model>>> {
        let mut model = self.model.model.write()?;
        model.events.clear();
        Ok(())
    }
    /// Send a adapter command to the game.
    pub fn send(&mut self, command: AdapterCommand) {
        if !self.is_finished() {
            self.command_tx
                .send(command)
                .expect("Should be able to send if the thread is not finished.");
        }
    }

    fn spawn(
        mut game: impl GameAdapter + Send + 'static,
        model: Arc<RwLock<Model>>,
        command_rx: mpsc::Receiver<AdapterCommand>,
        update_tx: broadcast::Sender<ModelUpdate>,
    ) -> JoinHandle<Result<(), AdapterError>> {
        thread::Builder::new()
            .name("Acc connection".into())
            .spawn(move || game.run(model, command_rx, update_tx))
            .expect("should be able to spawn thread")
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

/// Commands for the adapter to execute.
pub enum AdapterCommand {
    /// Close the adapter and return the thread.
    Close,
}
/// Notifies any users of the adapter about changes in the model.
#[derive(Clone)]
pub enum ModelUpdate {}
