use std::sync::RwLockWriteGuard;

use crate::model::{EntryId, Model};

/// This processor calculates the distance an entry has driven
/// and solves some inconsistencies that may be present in the data

/// Calculate the distance driven for an entry.
pub fn calc_distance_driven(model: &mut RwLockWriteGuard<Model>, entry_id: &EntryId) {
    let Some(session) = model.current_session_mut() else {
        return;
    };

    let Some(entry) = session.entries.get_mut(entry_id) else {
        return;
    };

    entry.distance_driven = entry.spline_pos + entry.lap_count as f32;
}
