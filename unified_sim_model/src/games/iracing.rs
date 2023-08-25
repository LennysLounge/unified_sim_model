use std::{
    collections::VecDeque,
    fmt::Display,
    sync::{
        mpsc::{Receiver, TryRecvError},
        Arc, RwLock,
    },
    time::Instant,
};

use thiserror::Error;
use tracing::{error, warn};

use crate::{model::Model, AdapterCommand, GameAdapter, UpdateEvent};

use self::{
    irsdk::{defines::Messages, Data, Irsdk},
    processors::{
        base::BaseProcessor, camera::CameraProcessor, lap::LapProcessor, IRacingProcessor,
        IRacingProcessorContext,
    },
};

use super::common::entry_finished;

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
    #[error("The game connection timed out")]
    TimedOut,
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
    camera_processor: CameraProcessor,
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
            camera_processor: CameraProcessor::new(),
        }
    }

    fn run_loop(&mut self) -> IRacingResult<()> {
        let mut last_update = Instant::now();
        loop {
            let now = Instant::now();
            if now.duration_since(last_update).as_secs() > 10 {
                return Err(IRacingError::TimedOut.into());
            }

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

            if !self.sdk.is_connected() {
                break;
            }

            last_update = now;
        }
        Ok(())
    }

    fn handle_commands(&self) -> IRacingResult<bool> {
        let should_close = match self.command_rx.try_recv() {
            Ok(command) => match command {
                AdapterCommand::Close => true,
                AdapterCommand::FocusOnCar(ref entry_id) => {
                    let model = self.model.read().expect("Model should not be poisoned");
                    let entry = model
                        .current_session()
                        .and_then(|session| session.entries.get(entry_id));
                    if let Some(entry) = entry {
                        self.sdk.send_message(Messages::CamSwitchNum {
                            driver_num: entry.car_number.as_copy() as u16,
                            camera_group: 0,
                            camera: 0,
                        });
                    }
                    false
                }
                AdapterCommand::ChangeCamera(camera) => {
                    let model = self.model.read().expect("Model should not be poisoned");
                    let camera = self.camera_processor.get_camera_def(&camera);
                    if let Some(camera) = camera {
                        let focused_entry = model.focused_entry.and_then(|id| {
                            model
                                .current_session()
                                .and_then(|session| session.entries.get(&id))
                        });
                        if let Some(entry) = focused_entry {
                            self.sdk.send_message(Messages::CamSwitchNum {
                                driver_num: entry.car_number.as_copy() as u16,
                                camera_group: camera.group_num as u16,
                                camera: camera.camera_num as u16,
                            });
                        }
                    } else {
                        warn!(
                            "Unavailable camera definition issued to iRacing adapter: {:?}",
                            camera
                        );
                    }
                    false
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
            self.camera_processor.static_data(&mut context)?;

            self.static_data_update_count = Some(data.static_data.update_count);
        }

        self.base_processor.live_data(&mut context)?;
        self.lap_processor.live_data(&mut context)?;
        self.camera_processor.live_data(&mut context)?;

        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            self.base_processor.event(&mut context, &event)?;
            self.lap_processor.event(&mut context, &event)?;
            self.camera_processor.event(&mut context, &event)?;

            entry_finished::calc_entry_finished(&event, context.model);
            context.model.events.push(event);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IRacingCamera {
    group_num: i32,
    group_name: String,
    camera_num: i32,
    camera_name: String,
}

impl Display for IRacingCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "iRacing {}", self.group_name)
    }
}
