use std::{
    collections::VecDeque,
    sync::{
        mpsc::{Receiver, TryRecvError},
        Arc, RwLock,
    },
};

use thiserror::Error;
use tracing::{error, info};

use crate::{log_todo, model::Model, AdapterCommand, GameAdapter, UpdateEvent};

use self::{
    irsdk::{Data, Irsdk},
    processors::{
        base::BaseProcessor, lap::LapProcessor, IRacingProcessor, IRacingProcessorContext,
    },
};

pub mod irsdk;
mod processors;

/// A specialized result for Connection errors.
pub(crate) type IRacingResult<T> = std::result::Result<T, crate::AdapterError>;

#[derive(Debug, Error)]
pub enum IRacingError {
    #[error("The game is not running")]
    GameNotRunning,
    #[error("The game disconnected")]
    Disconnected,
    #[error("Missing required data: {0}")]
    MissingData(String),
    #[error("Internal windows error: {0}")]
    WindowsError(windows::core::Error),
    #[error("The adapter encountered an error: {0}")]
    Other(String),
}

impl From<IRacingError> for crate::AdapterError {
    fn from(value: IRacingError) -> Self {
        crate::AdapterError::IRacing(value)
    }
}

pub struct IRacingAdapter;
impl GameAdapter for IRacingAdapter {
    fn run(
        &mut self,
        model: Arc<RwLock<Model>>,
        command_rx: Receiver<AdapterCommand>,
        update_event: UpdateEvent,
    ) -> IRacingResult<()> {
        let sdk = Irsdk::new().map_err(|_| IRacingError::GameNotRunning)?;

        if let Ok(mut model) = model.write() {
            model.connected = true;
            model.event_name.set("iRacing".to_owned());
        }
        let mut connection = IRacingConnection::new(model.clone(), command_rx, update_event, sdk);
        let result = connection.run_loop();

        if let Ok(mut model) = model.write() {
            model.connected = false;
        }

        result
    }
}

struct IRacingConnection {
    model: Arc<RwLock<Model>>,
    command_rx: Receiver<AdapterCommand>,
    update_event: UpdateEvent,
    sdk: Irsdk,
    static_data_update_count: Option<i32>,
    lap_processor: LapProcessor,
    base_processor: BaseProcessor,
}

impl IRacingConnection {
    fn new(
        model: Arc<RwLock<Model>>,
        command_rx: Receiver<AdapterCommand>,
        update_event: UpdateEvent,
        sdk: Irsdk,
    ) -> Self {
        Self {
            model,
            command_rx,
            update_event,
            sdk,
            static_data_update_count: None,
            lap_processor: LapProcessor::new(),
            base_processor: BaseProcessor {},
        }
    }

    fn run_loop(&mut self) -> IRacingResult<()> {
        loop {
            let should_close = self.handle_commands()?;
            if should_close {
                break;
            }

            if let Err(error) = self.sdk.wait_for_update(16) {
                match error {
                    irsdk::WaitError::Timeout => continue,
                    irsdk::WaitError::Win32Error(code) => Err(IRacingError::WindowsError(code))?,
                }
            }

            let data = self.sdk.poll().map_err(|e| match e {
                irsdk::PollError::NotConnected => IRacingError::Disconnected,
            })?;

            self.update_model(&data)?;
            self.update_event.trigger();
        }
        Ok(())
    }

    fn handle_commands(&self) -> IRacingResult<bool> {
        let should_close = match self.command_rx.try_recv() {
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

    fn update_model(&mut self, data: &Data) -> IRacingResult<()> {
        let mut context = IRacingProcessorContext {
            model: &mut *self
                .model
                .write()
                .map_err(|_| IRacingError::Other("Model was poisoned".into()))?,
            events: VecDeque::new(),
            data,
        };

        if self
            .static_data_update_count
            .map_or(true, |count| count != data.static_data.update_count)
        {
            self.base_processor.static_data(&mut context)?;
            self.lap_processor.static_data(&mut context)?;

            self.static_data_update_count = Some(data.static_data.update_count);
        }

        self.base_processor.live_data(&mut context)?;
        self.lap_processor.live_data(&mut context)?;

        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            self.base_processor.event(&mut context, &event)?;
            self.lap_processor.event(&mut context, &event)?;
        }

        Ok(())
    }
}
