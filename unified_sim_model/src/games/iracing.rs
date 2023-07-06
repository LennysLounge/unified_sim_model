use std::{
    sync::{
        mpsc::{Receiver, TryRecvError},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

use thiserror::Error;
use tracing::error;

use crate::{log_todo, model::Model, AdapterCommand, GameAdapter, UpdateEvent};

use self::irsdk::Irsdk;

pub mod irsdk;

/// A specialized result for Connection errors.
type Result<T> = std::result::Result<T, crate::AdapterError>;

#[derive(Debug, Error)]
pub enum IRacingError {
    #[error("The game is not running")]
    GameNotRunning,
    #[error("The game disconnected")]
    Disconnected,
    #[error("The adapter encountered an error: {0}")]
    Other(String),
}

impl From<IRacingError> for crate::AdapterError {
    fn from(value: IRacingError) -> Self {
        crate::AdapterError::IRacing(value)
    }
}

pub struct IRacingAdapter {}
impl GameAdapter for IRacingAdapter {
    fn run(
        &mut self,
        _model: Arc<RwLock<Model>>,
        command_rx: Receiver<AdapterCommand>,
        update_event: &UpdateEvent,
    ) -> Result<()> {
        let mut irsdk = Irsdk::new().map_err(|_| IRacingError::GameNotRunning)?;

        loop {
            let should_close = Self::handle_commands(&command_rx)?;
            if should_close {
                break;
            }

            let _data = irsdk.poll().map_err(|e| match e {
                irsdk::PollError::NotConnected => IRacingError::Disconnected,
            })?;

            //info!("laps: {:?}", data.car_idx_lap);

            //Self::update_model(&data, &model)?;

            update_event.trigger();

            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}

impl IRacingAdapter {
    fn handle_commands(receiver: &Receiver<AdapterCommand>) -> Result<bool> {
        let should_close = match receiver.try_recv() {
            Ok(command) => match command {
                AdapterCommand::Close => true,
                AdapterCommand::FocusOnCar(_) => {
                    log_todo(false, "Focus on car command not implemented yet")
                }
                AdapterCommand::ChangeCamera(_) => {
                    log_todo(false, "Change camera command not implemented yet")
                }
            },
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => {
                // This should only happen if all adapters have been dropped.
                // In which case it is impossible to interact with this adapter any more.
                // To avoid leaking memory we quit.
                error!("All adapter handle have been dropped it is impossible to communicate with this game adapter.");
                true
            }
        };

        Ok(should_close)
    }

    // fn update_model(data: &Data, model: &Arc<RwLock<Model>>) -> Result<()> {
    //     let model = model
    //         .write()
    //         .map_err(|_| IRacingError::Other("Model was poisoned".into()))?;

    //     //info!("Current gear: {:?}", data.gear.value());

    //     Ok(())
    // }
}
