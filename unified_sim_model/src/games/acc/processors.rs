use std::collections::VecDeque;

use crate::{
    games::acc::AccConnectionError,
    model::{Event, Model},
};

use super::{
    data::{
        BroadcastingEvent, EntryList, EntryListCar, Message, RealtimeCarUpdate, RegistrationResult,
        SessionUpdate, TrackData,
    },
    AccSocket, Result,
};

pub mod base;
pub mod connection;
pub mod distance_driven;
pub mod entry_finished;
pub mod gap_to_leader;
pub mod lap;

pub mod session_progress;
/// A context for a processor to work in.
pub struct AccProcessorContext<'a> {
    pub(crate) socket: &'a mut AccSocket,
    pub(crate) model: &'a mut Model,
    pub(crate) events: VecDeque<Event>,
}

/// This trait descibes a processor that can process the
/// data events from the game and modify the model.
pub trait AccProcessor {
    #[allow(unused)]
    fn registration_result(
        &mut self,
        result: &RegistrationResult,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn session_update(
        &mut self,
        update: &SessionUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn entry_list(&mut self, list: &EntryList, context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn track_data(&mut self, track: &TrackData, context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn entry_list_car(
        &mut self,
        car: &EntryListCar,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn broadcast_event(
        &mut self,
        event: &BroadcastingEvent,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn event(&mut self, event: &Event, context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    fn process_message(
        &mut self,
        message: &Message,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        match message {
            Message::Unknown(t) => {
                Err(AccConnectionError::Other(format!("Unknown message type: {}", t)).into())
            }
            Message::RegistrationResult(result) => self.registration_result(result, context),
            Message::SessionUpdate(update) => self.session_update(update, context),
            Message::RealtimeCarUpdate(update) => self.realtime_car_update(update, context),
            Message::EntryList(list) => self.entry_list(list, context),
            Message::TrackData(track) => self.track_data(track, context),
            Message::EntryListCar(car) => self.entry_list_car(car, context),
            Message::BroadcastingEvent(event) => self.broadcast_event(event, context),
        }
    }
}
