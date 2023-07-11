//! This module includes the additional model data for this adapter.

use std::fmt::Display;

use crate::model::{Camera, EntryGameData, GameCamera, SessionGameData};

use super::{data::CarLocation, AccConnectionError};

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
    /// Returns the data as the ACC variant.
    pub fn as_acc(&self) -> Option<&AccSession> {
        match self {
            SessionGameData::Acc(data) => Some(data),
            _ => None,
        }
    }
    /// Returns the data as the ACC variant mutably.
    pub fn as_acc_mut(&mut self) -> Option<&mut AccSession> {
        match self {
            SessionGameData::Acc(ref mut data) => Some(data),
            _ => None,
        }
    }

    /// Returns the data as the ACC variant or return an error.
    pub(crate) fn assert_acc_mut(&mut self) -> Result<&mut AccSession, AccConnectionError> {
        match self {
            SessionGameData::Acc(data) => Ok(data),
            _ => Err(AccConnectionError::Other(
                "Session game data is not compatible with the acc adapter".to_owned(),
            )),
        }
    }

    /// Returns the data as the ACC variant or return an error.
    pub(crate) fn assert_acc(&self) -> Result<&AccSession, AccConnectionError> {
        match self {
            SessionGameData::Acc(data) => Ok(data),
            _ => Err(AccConnectionError::Other(
                "Session game data is not compatible with the acc adapter".to_owned(),
            )),
        }
    }
}

/// Contains additional information that is presented by the game.
///
/// These fields may not necessairly be usefull to anyone but they
/// exist to make all the data from the game available.
#[derive(Debug, Default, Clone)]
pub struct AccEntry {
    /// The ingame id for this car.
    pub car_id: i16,
    /// The cup category of the car.
    pub cup_category: u8,
    /// The location of the car.
    pub car_location: CarLocation,
    /// The position of this car in its cup.
    pub cup_position: i16,
    /// TODO: find out what this is exactly.
    pub track_position: i16,
}

impl EntryGameData {
    /// Returns the data as the ACC variant.
    pub fn as_acc(&self) -> Option<&AccEntry> {
        match self {
            EntryGameData::Acc(data) => Some(data),
            _ => None,
        }
    }
    /// Returns the data as the ACC variant mutably.
    pub fn as_acc_mut(&mut self) -> Option<&mut AccEntry> {
        match self {
            EntryGameData::Acc(ref mut data) => Some(data),
            _ => None,
        }
    }

    /// Returns the data as the ACC variant or return an error.
    pub(crate) fn assert_acc_mut(&mut self) -> Result<&mut AccEntry, AccConnectionError> {
        match self {
            EntryGameData::Acc(data) => Ok(data),
            _ => Err(AccConnectionError::Other(
                "Entry game data is not compatible with the acc adapter".to_owned(),
            )),
        }
    }

    /// Returns the data as the ACC variant or return an error.
    pub(crate) fn _assert_acc(&self) -> Result<&AccEntry, AccConnectionError> {
        match self {
            EntryGameData::Acc(data) => Ok(data),
            _ => Err(AccConnectionError::Other(
                "Entry game data is not compatible with the acc adapter".to_owned(),
            )),
        }
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum AccCamera {
    /// A Helicopter camera.
    Helicam,
    /// Pitlane camera.
    ///
    /// If the focused car is not in the pitlane, this camera will cycle between
    /// helicam, Tv1, and Tv2.
    /// If the focused car is in the pitlane it will cycle between helicam and pitlane.
    Pitlane,
    /// A TV like camera that automatically switches the camera to keep the focused car in view.
    Tv1,
    /// A more cinematic camera that automatically switches the camera to keep the focused car in view.
    Tv2,
    /// A third person chase camera that is flying behind the car.
    Chase,
    /// A third person chase camera that is flying further behind the car.
    FarChase,
    /// A camera on the bonnet of the car.
    Bonnet,
    /// A camera locked to front of the bumper.
    DashPro,
    /// The cockpit camera.
    #[default]
    Cockpit,
    /// A camera locked to the dash.
    Dash,
    /// A camera inside the helmet.
    Helmet,
    /// A camera showing the interior of the car facing forwards.
    Onboard0,
    /// A camera facing rearwards showing the driver.
    Onboard1,
    /// A camera on the passenger side of the dashboard facing forwards.
    Onboard2,
    /// A camaera facing rearwards showing the rear wing of the car.
    Onboard3,
}

impl AccCamera {
    /// Get the camera definition for the camera.
    fn camera_definition(&self) -> (&'static str, &'static str) {
        match self {
            AccCamera::Helicam => ("Helicam", "Helicam"),
            AccCamera::Pitlane => ("pitlane", "camera"),
            AccCamera::Tv1 => ("set1", "camera"),
            AccCamera::Tv2 => ("set2", "camera"),
            AccCamera::Chase => ("Drivable", "Chase"),
            AccCamera::FarChase => ("Drivable", "FarChase"),
            AccCamera::Bonnet => ("Drivable", "Bonnet"),
            AccCamera::DashPro => ("Drivable", "DashPro"),
            AccCamera::Cockpit => ("Drivable", "Cockpit"),
            AccCamera::Dash => ("Drivable", "Dash"),
            AccCamera::Helmet => ("Drivable", "Helmet"),
            AccCamera::Onboard0 => ("Onboard", "Onboard0"),
            AccCamera::Onboard1 => ("Onboard", "Onboard1"),
            AccCamera::Onboard2 => ("Onboard", "Onboard2"),
            AccCamera::Onboard3 => ("Onboard", "Onboard3"),
        }
    }
}

impl Display for AccCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccCamera::Helicam => write!(f, "ACC Helicam"),
            AccCamera::Pitlane => write!(f, "ACC Pitlane"),
            AccCamera::Tv1 => write!(f, "ACC Tv1"),
            AccCamera::Tv2 => write!(f, "ACC Tv2"),
            AccCamera::Chase => write!(f, "ACC Chase"),
            AccCamera::FarChase => write!(f, "ACC FarChase"),
            AccCamera::Bonnet => write!(f, "ACC Bonnet"),
            AccCamera::DashPro => write!(f, "ACC DashPro"),
            AccCamera::Cockpit => write!(f, "ACC Cockpit"),
            AccCamera::Dash => write!(f, "ACC Dash"),
            AccCamera::Helmet => write!(f, "ACC Helmet"),
            AccCamera::Onboard0 => write!(f, "ACC Onboard0"),
            AccCamera::Onboard1 => write!(f, "ACC Onboard1"),
            AccCamera::Onboard2 => write!(f, "ACC Onboard2"),
            AccCamera::Onboard3 => write!(f, "ACC Onboard3"),
        }
    }
}

impl Camera {
    /// Get the acc camera definition for this camera setting.
    /// None if the camera is not supported by acc.
    pub(crate) fn as_acc_camera_definition(&self) -> Option<(&str, &str)> {
        let camera = match self {
            Camera::None => None,
            Camera::FirstPerson => Some(AccCamera::Cockpit),
            Camera::Chase => Some(AccCamera::Chase),
            Camera::TV => Some(AccCamera::Tv1),
            Camera::Hellicopter => Some(AccCamera::Helicam),
            Camera::Game(game) => match game {
                GameCamera::Acc(camera) => Some(camera.clone()),
                _ => None,
            },
        };
        camera.map(|camera| camera.camera_definition())
    }
}
