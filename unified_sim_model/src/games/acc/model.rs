//! This module includes the additional model data for this adapter.

use crate::model::SessionGameData;

use super::AccConnectionError;

/// Contains additional information that is presented by the game.
///
/// These fields may not necessairly be usefull to anyone but they
/// exist to make all the data from the game available.
#[derive(Debug, Default, Clone)]
pub struct AccSession {
    /// This values doesnt look like it does anything.
    pub event_index: i16,
    /// The raw session index.
    pub session_index: i16,
    /// The active camerea set.
    pub camera_set: String,
    /// The active camera.
    pub camera: String,
    /// The active hud page.
    pub hud_page: String,
    /// If a replay is currently playing.
    pub replay_playing: bool,
    /// The cloud level.
    /// Note this value was disabled by kunos some time ago and only
    /// exists for compatibility.
    pub cloud_level: u8,
    /// The rain level.
    /// Note this value was disabled by kunos some time ago and only
    /// exists for compatibility.
    pub rain_level: u8,
    /// The wetness.
    /// Note this value was disabled by kunos some time ago and only
    /// exists for compatibility.
    pub wetness: u8,
}

impl SessionGameData {
    /// Returns the as the ACC variant.
    pub fn as_acc(&self) -> Option<&AccSession> {
        match self {
            SessionGameData::Acc(data) => Some(data),
            _ => None,
        }
    }
    /// Returns the as the ACC variant mutably.
    pub fn as_acc_mut(&mut self) -> Option<&mut AccSession> {
        match self {
            SessionGameData::Acc(ref mut data) => Some(data),
            _ => None,
        }
    }

    /// Returns the as the ACC variant or return an error.
    pub(crate) fn assert_acc_mut(&mut self) -> Result<&mut AccSession, AccConnectionError> {
        match self {
            SessionGameData::Acc(data) => Ok(data),
            _ => Err(AccConnectionError::Other(
                "Session game data is not compatible with the acc adapter".to_owned(),
            )),
        }
    }

    /// Returns the as the ACC variant or return an error.
    pub(crate) fn assert_acc(&self) -> Result<&AccSession, AccConnectionError> {
        match self {
            SessionGameData::Acc(data) => Ok(data),
            _ => Err(AccConnectionError::Other(
                "Session game data is not compatible with the acc adapter".to_owned(),
            )),
        }
    }
}
