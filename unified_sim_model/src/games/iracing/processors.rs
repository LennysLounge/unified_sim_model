use std::collections::VecDeque;

use crate::model::{Event, Model};

use super::{irsdk::Data, IRacingResult};

pub mod base;
pub mod camera;
pub mod lap;
pub mod speed;

/// The context for a iracing processor.
pub struct IRacingProcessorContext<'a> {
    /// The unified model
    pub(crate) model: &'a mut Model,
    /// List of newly created events.
    pub(crate) events: VecDeque<Event>,
    /// The current up-to-date data
    pub(crate) data: &'a Data,
}

pub trait IRacingProcessor {
    /// Called when the session string changes.
    /// This call happens before the live data is updated.
    fn static_data(&mut self, context: &mut IRacingProcessorContext) -> IRacingResult<()>;
    /// Process the live data in a regular interval.
    fn live_data(&mut self, context: &mut IRacingProcessorContext) -> IRacingResult<()>;
    /// Called when an event occurs so this processor can react to it.
    fn event(&mut self, context: &mut IRacingProcessorContext, event: &Event) -> IRacingResult<()>;
}
