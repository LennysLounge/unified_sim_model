use crate::{games::common::distance_driven, model::EntryId};

use super::AccProcessor;

pub struct DistanceDrivenProcessor;
impl AccProcessor for DistanceDrivenProcessor {
    fn realtime_car_update(
        &mut self,
        update: &crate::games::acc::data::RealtimeCarUpdate,
        context: &mut super::AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        context
            .model
            .current_session_mut()
            .and_then(|session| session.entries.get_mut(&EntryId(update.car_id as i32)))
            .map(|entry| distance_driven::calc_distance_driven(entry));
        Ok(())
    }
}
