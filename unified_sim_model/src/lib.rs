use games::{acc, dummy};
use thiserror::Error;
use tracing::warn;

use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard,
    },
    thread::JoinHandle,
};

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

/// A error with the game connection.
#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("Acc connection error")]
    ACC(acc::AccConnectionError),
}

/// An adapter to a game.
pub struct Adapter {
    /// The shared model.
    pub model: ReadOnlyModel,
    /// The join handle to close the connection thread to the game.
    join_handle: Option<JoinHandle<Result<(), AdapterError>>>,
    /// Channel to send adapter actions to the adapter.
    sender: Sender<AdapterCommand>,
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
        let (sender, receiver) = mpsc::channel();
        Adapter {
            join_handle: Some(acc::AccConnection::spawn(model.clone(), receiver)),
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
            self.sender
                .send(command)
                .expect("Should be able to send if the thread is not finished.");
        }
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
