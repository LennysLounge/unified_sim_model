use crate::model::{EntryId, Model};

/// This processor calculates the distance an entry has driven
/// and solves some inconsistencies that may be present in the data.

/// Calculate the distance driven for an entry.
pub fn calc_distance_driven(model: &mut Model, entry_id: &EntryId) {
    // If the lap completed line and the spline position line are not exactly matched up,
    // then it is possible for one to change to a new value before the other. This causes
    // a spike in the data. This processor fixes this issue.
    let entry = model
        .current_session_mut()
        .and_then(|session| session.entries.get_mut(entry_id));
    if let Some(entry) = entry {
        let mut distance_driven = *entry.spline_pos + *entry.lap_count as f32;

        if (entry.spline_pos > 0.95 || entry.spline_pos < 0.05) && !*entry.in_pits {
            let diff_to_last_update = distance_driven - *entry.distance_driven;
            if diff_to_last_update < -0.5 {
                distance_driven += 1.0;
            }
            if diff_to_last_update > 0.5 {
                distance_driven -= 1.0;
            }
        }

        entry.distance_driven.set(distance_driven);
    }
}
