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
pub mod lap;

/// A context for a processor to work in.
pub struct AccProcessorContext<'a> {
    pub(crate) socket: &'a mut AccSocket,
    pub(crate) model: &'a mut Model,
    pub(crate) events: VecDeque<Event>,
}

/// This trait descibes a processor that can process the
/// data events from the game and modify the model.
pub trait AccProcessor {
    fn registration_result(
        &mut self,
        _result: &RegistrationResult,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn session_update(
        &mut self,
        _update: &SessionUpdate,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn realtime_car_update(
        &mut self,
        _update: &RealtimeCarUpdate,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn entry_list(&mut self, _list: &EntryList, _context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    fn track_data(&mut self, _track: &TrackData, _context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }

    fn entry_list_car(
        &mut self,
        _car: &EntryListCar,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn broadcast_event(
        &mut self,
        _event: &BroadcastingEvent,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        Ok(())
    }

    fn event(&mut self, _event: &Event, _context: &mut AccProcessorContext) -> Result<()> {
        Ok(())
    }
}

pub fn process_message(
    me: &mut impl AccProcessor,
    message: &Message,
    context: &mut AccProcessorContext,
) -> Result<()> {
    use Message::*;
    match message {
        Unknown(t) => Err(AccConnectionError::Other(format!("Unknown message type: {}", t)).into()),
        RegistrationResult(ref result) => me.registration_result(result, context),
        SessionUpdate(ref update) => me.session_update(update, context),
        RealtimeCarUpdate(ref update) => me.realtime_car_update(update, context),
        EntryList(ref list) => me.entry_list(list, context),
        TrackData(ref track) => me.track_data(track, context),
        EntryListCar(ref car) => me.entry_list_car(car, context),
        BroadcastingEvent(ref event) => me.broadcast_event(event, context),
    }
}
