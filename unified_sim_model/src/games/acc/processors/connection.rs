use std::collections::HashMap;

use tracing::info;

use crate::{
    games::acc::{
        data::{RealtimeCarUpdate, SessionUpdate},
        AccProcessor, AccProcessorContext, Result,
    },
    model::{EntryId, Event},
};

/// Processor for the connection property on an entry.
///
/// An entry is considered connected aslong as it recieves an update
/// for every session update. This is acomplished as follows.
/// The EntryJoined event will add an entry to be tracked by this processor.
/// During the session update, we reset the connection status for each entry.
/// Once an entry receives an update it also updates its connected statsu to true.
/// At the start of the next session update, all entries that have not
/// received an update are disconnected and will trigger an event.
///
/// If an entry starts receiving updates after it was disconnected,
/// it will be reconnected and trigger the event.
///
#[derive(Debug, Default)]
pub struct ConnectionProcessor {
    /// Maps an entry to their connection status.
    entries: HashMap<EntryId, bool>,
}

impl AccProcessor for ConnectionProcessor {
    fn session_update(
        &mut self,
        _update: &SessionUpdate,
        context: &mut AccProcessorContext,
    ) -> Result<()> {
        let session = context
            .model
            .current_session_mut()
            .expect("Session update should create a session");

        for entry in session.entries.values_mut() {
            let is_connected = self
                .entries
                .get_mut(&entry.id)
                .expect("An entry in the session should also have a connection entry.");
            match (&is_connected, *entry.connected) {
                (true, false) => {
                    info!("Entry reconnected: #{}", *entry.car_number);
                    context.events.push_back(Event::EntryReconnected(entry.id));
                }
                (false, true) => {
                    info!("Entry disconnected: #{}", *entry.car_number);
                    context.events.push_back(Event::EntryDisconnected(entry.id));
                }
                _ => (),
            }
            entry.connected.set(*is_connected);
            *is_connected = false;
        }
        Ok(())
    }

    fn realtime_car_update(
        &mut self,
        update: &RealtimeCarUpdate,
        _context: &mut AccProcessorContext,
    ) -> Result<()> {
        if let Some(is_connected) = self.entries.get_mut(&EntryId(update.car_id as i32)) {
            *is_connected = true;
        }

        Ok(())
    }

    fn event(&mut self, event: &Event, _context: &mut AccProcessorContext) -> Result<()> {
        if let Event::EntryConnected(entry_id) = event {
            self.entries.insert(*entry_id, true);
        }
        Ok(())
    }
}
