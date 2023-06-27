use games::{acc, dummy::DummyAdapter, iracing};
use model::{Camera, EntryId};
use thiserror::Error;
use tracing::warn;

use std::{
    sync::{mpsc, Arc, Condvar, Mutex, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread::{self, JoinHandle},
    time::Duration,
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
        update_event: &UpdateEvent,
    ) -> Result<(), AdapterError>;
}

/// A error with the game adapter.
#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("Acc connection error: {0}")]
    ACC(acc::AccConnectionError),
    #[error("IRacing connection error: {0}")]
    IRacing(iracing::IRacingError),
}

/// The result of an adapter after it has finished.
/// If the adapter had to finish because of an error the error
/// is reported in the `Err` variant.
pub type AdapteResult = Result<(), AdapterError>;

/// An adapter to a game.
///
/// The Adapter is the connection point between the game and your code.
/// When the adapter is the created it opens a new thread with a game specifc connection to
/// the game. The adapter is then the window to interact with the game connection.
/// Should all instances of an adapter be dropped or destroyed, the game connection
/// that is associated with that adapter is also closed.
#[derive(Clone)]
pub struct Adapter {
    /// The data model that is shared with the game adapter.
    pub model: ReadOnlyModel,
    /// The join handle to close the connection thread to the game.
    join_handle: Arc<RwLock<Option<JoinHandle<AdapteResult>>>>,
    /// Channel for sending commands to the game.
    command_tx: mpsc::Sender<AdapterCommand>,
    /// An event that is triggered when new data is available.
    update_event: UpdateEvent,
}

impl Adapter {
    /// Create a new adapter with a game adapter.
    pub fn new(game: impl GameAdapter + Send + 'static) -> Self {
        let model = Arc::new(RwLock::new(Model::default()));
        let (command_tx, command_rx) = mpsc::channel();
        let update_event = UpdateEvent::new();
        Self {
            model: ReadOnlyModel::new(model.clone()),
            join_handle: Arc::new(RwLock::new(Some(Self::spawn(
                game,
                model,
                command_rx,
                update_event.clone(),
            )))),
            command_tx,
            update_event,
        }
    }
    /// Create a new dummy adapter.
    /// The adapter will write some data into the model and immediately finish.
    pub fn new_dummy() -> Adapter {
        Self::new(DummyAdapter::default())
    }

    /// Create a new Assetto Corsa Competizione adapter.
    pub fn new_acc() -> Adapter {
        Self::new(acc::AccAdapter {})
    }

    /// Create a new iRacing adapter.
    pub fn new_iracing() -> Adapter {
        Self::new(iracing::IRacingAdapter {})
    }

    /// Returns `true` if the adapter has finised its connection to the game
    pub fn is_finished(&self) -> bool {
        self.join_handle
            .read()
            .unwrap()
            .as_ref()
            .map_or(true, |handle| handle.is_finished())
    }

    /// Joins the adapter thread and returns the result.
    ///
    /// The result is only returned the first time this method is called after
    /// the thread has finished. Calling it after the result has been taking will also return `None`.
    pub fn join(&mut self) -> Option<Result<(), AdapterError>> {
        self.join_handle
            .write()
            .unwrap()
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
    ///
    /// There is no guarantee that a command is received by the game and that there will
    /// be a desired effect. It is adapters responsibility to fulfill the command to the
    /// best of its abilities.
    pub fn send(&self, command: AdapterCommand) {
        if !self.is_finished() {
            // Since success is not a guarantee of this method we dont need to notify the
            // user of a failed send.
            _ = self.command_tx.send(command);
        }
    }

    /// Block this thread until a new update is available in the model.
    ///
    /// Returns a error if the event source is closed before an event is triggered.
    pub fn wait_for_update(&self) -> Result<(), WaitError> {
        self.update_event.wait()
    }

    /// Blocks this thread until a new update is available in the model.
    ///
    /// Returns an error if the event source is closed or the timeout expires.
    pub fn wait_for_update_timeout(&self, duration: Duration) -> Result<(), WaitError> {
        self.update_event.wait_timeout(duration)
    }

    fn spawn(
        mut game: impl GameAdapter + Send + 'static,
        model: Arc<RwLock<Model>>,
        command_rx: mpsc::Receiver<AdapterCommand>,
        update_event: UpdateEvent,
    ) -> JoinHandle<Result<(), AdapterError>> {
        update_event.enable();
        thread::Builder::new()
            .name("Acc connection".into())
            .spawn(move || {
                let result = game.run(model, command_rx, &update_event);
                update_event.disable();
                result
            })
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
    /// Change the focus to another entry.
    FocusOnCar(EntryId),
    /// Change the camera.
    ChangeCamera(Camera),
}

/// An event that is triggered when the model receives an update.
///
/// This is a wrapper around a convar.
#[derive(Clone)]
pub struct UpdateEvent {
    pair: Arc<(Mutex<EventState>, Condvar)>,
}

/// An error that can occur when waiting for an event.
#[derive(Debug, Error)]
pub enum WaitError {
    #[error("Event source is closed")]
    EventDisabled,
    #[error("Wait timeout expired")]
    TimeoutExpired,
}

struct EventState {
    enabled: bool,
    counter: usize,
}

impl UpdateEvent {
    fn new() -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(EventState {
                    enabled: false,
                    counter: 0,
                }),
                Condvar::new(),
            )),
        }
    }

    /// Enable this event.
    /// When the event is enabled, any call to wait will block until an event has been published.
    fn enable(&self) {
        let (state, _) = &*self.pair;
        state.lock().unwrap().enabled = true;
    }

    /// Disable this event.
    /// When the event is disable, any call to wait will return with an error.
    /// If there are any threads waiting for an event, this will disable the event and
    /// trigger it once to wake up any threads.
    fn disable(&self) {
        let (state, var) = &*self.pair;
        state.lock().unwrap().enabled = false;
        var.notify_all();
    }

    /// Trigger the event.
    ///
    /// Only triggers the event if the event is enabled.
    pub fn trigger(&self) {
        let (state_mutex, var) = &*self.pair;
        let mut state = state_mutex.lock().unwrap();
        if !state.enabled {
            return;
        }
        state.counter += 1;
        var.notify_all();
    }

    /// Block and wait for the next event.
    ///
    /// This function will error when the event source closes.
    pub fn wait(&self) -> Result<(), WaitError> {
        let (state_mutex, var) = &*self.pair;
        let mut state = state_mutex.lock().unwrap();
        if !state.enabled {
            return Err(WaitError::EventDisabled);
        }
        let prev_event_count = state.counter;
        while state.enabled && state.counter == prev_event_count {
            state = var.wait(state).unwrap();
        }
        if !state.enabled {
            return Err(WaitError::EventDisabled);
        }
        Ok(())
    }

    /// Block and wait for the next event or until the timeout expires.
    ///
    /// THis function will error when the event source closes or when the timeout expires.
    pub fn wait_timeout(&self, duration: Duration) -> Result<(), WaitError> {
        let (state_mutex, var) = &*self.pair;
        let mut state = state_mutex.lock().unwrap();
        if !state.enabled {
            return Err(WaitError::EventDisabled);
        }
        let prev_event_count = state.counter;
        while state.enabled && state.counter == prev_event_count {
            let (next_state, result) = var.wait_timeout(state, duration).unwrap();
            state = next_state;
            if result.timed_out() {
                return Err(WaitError::TimeoutExpired);
            }
        }
        if !state.enabled {
            return Err(WaitError::EventDisabled);
        }
        Ok(())
    }
}
