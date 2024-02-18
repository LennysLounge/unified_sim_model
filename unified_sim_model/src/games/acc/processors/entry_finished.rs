use crate::games::common::entry_finished;

use super::AccProcessor;

pub struct EntryFinishedProcessor;
impl AccProcessor for EntryFinishedProcessor {
    fn event(
        &mut self,
        event: &crate::model::Event,
        context: &mut super::AccProcessorContext,
    ) -> crate::games::acc::Result<()> {
        entry_finished::calc_entry_finished(event, context.model);
        Ok(())
    }
}
