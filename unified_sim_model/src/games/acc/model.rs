//! This module includes the additional model data for this adapter.

use crate::model::SessionGameData;

#[derive(Debug, Default, Clone)]
pub struct AccSession {
    /// The raw session index.
    pub(crate) _index: i16,
}

impl SessionGameData {
    pub fn as_acc(&self) -> Option<&AccSession> {
        match self {
            SessionGameData::Acc(data) => Some(data),
            _ => None,
        }
    }
    pub fn as_acc_mut(&mut self) -> Option<&mut AccSession> {
        match self {
            SessionGameData::Acc(ref mut data) => Some(data),
            _ => None,
        }
    }
}
