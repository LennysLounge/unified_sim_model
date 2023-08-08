//! This processor calculates when an entry has finished its session
//! and sets the 'is_finished' value.

use crate::model::{Event, Lap, LapCompleted, Model, SessionId, SessionPhase, SessionType};

/// Calculate if the entries have completed the session.
pub fn calc_entry_finished(event: &Event, model: &mut Model) {
    match event {
        Event::SessionPhaseChanged(id, phase) => phase_changed(id, phase, model),
        Event::LapCompleted(LapCompleted { lap, .. }) => lap_completed(lap, model),
        _ => (),
    }
}

fn phase_changed(id: &SessionId, phase: &SessionPhase, model: &mut Model) {
    let Some(session) = model.sessions.get_mut(id) else {return};
    match phase {
        SessionPhase::None
        | SessionPhase::Waiting
        | SessionPhase::Preparing
        | SessionPhase::Formation
        | SessionPhase::Active => {
            // All entries are active.
            for entry in session.entries.values_mut() {
                entry.is_finished.set(false);
            }
        }
        SessionPhase::Ending => {
            match session.session_type.as_ref() {
                // During timing sessions an entry may finish the lap they are currently on.
                // Entries in the pits and disconnected are treated as finished.
                // Entries on track are finished once they have completed their lap.
                SessionType::Practice | SessionType::Qualifying => {
                    for entry in session.entries.values_mut() {
                        if entry.connected == false || entry.in_pits == true {
                            entry.is_finished.set(true);
                        }
                    }
                }
                SessionType::Race => (),
                SessionType::None => (),
            }
        }
        SessionPhase::Finished => {
            // All entries are finished.
            for entry in session.entries.values_mut() {
                entry.is_finished.set(true);
            }
        }
    }
}

fn lap_completed(lap: &Lap, model: &mut Model) {
    let Some(session) = model.current_session_mut() else {return};
    let Some(entry_id) = lap.entry_id else {return};

    if session.phase != SessionPhase::Ending {
        return;
    }

    match session.session_type.as_ref() {
        // Finishing a lap while the session is in the ending phase puts
        // the entry into the finished state.
        SessionType::Practice | SessionType::Qualifying => {
            if let Some(entry) = session.entries.get_mut(&entry_id) {
                entry.is_finished.set(true);
            }
        }
        SessionType::Race => {
            // An entry finishes the session when they complete their current lap
            // after the leader has finished the session.
            // The leader finished the session by completing their current lap
            // while the session is in the 'ending' phase.
            let leader = session
                .entries
                .values()
                .min_by_key(|entry| entry.position.as_ref());

            let is_leader = leader.is_some_and(|leader| leader.id == entry_id);
            let leader_has_finished = leader.map_or(true, |leader| leader.is_finished == true);

            if is_leader || leader_has_finished {
                if let Some(entry) = session.entries.get_mut(&entry_id) {
                    entry.is_finished.set(true);
                }
            }
        }
        SessionType::None => todo!(),
    }
}
