use crate::model::{self};

pub mod cars;
pub mod messages;

pub use cars::*;
pub use messages::*;

#[derive(Debug, Default, Clone)]
pub enum SessionPhase {
    #[default]
    None,
    Starting,
    PreFormation,
    FormationLap,
    PreSession,
    Session,
    SessionOver,
    PostSession,
    ResultUi,
}

impl From<SessionPhase> for model::SessionPhase {
    fn from(value: SessionPhase) -> Self {
        match value {
            SessionPhase::None => model::SessionPhase::None,
            SessionPhase::Starting => model::SessionPhase::PreSession,
            SessionPhase::PreFormation => model::SessionPhase::PreSession,
            SessionPhase::FormationLap => model::SessionPhase::PostSession,
            SessionPhase::PreSession => model::SessionPhase::PreSession,
            SessionPhase::Session => model::SessionPhase::Session,
            SessionPhase::SessionOver => model::SessionPhase::PostSession,
            SessionPhase::PostSession => model::SessionPhase::PostSession,
            SessionPhase::ResultUi => model::SessionPhase::Finished,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum SessionType {
    Practice,
    Qualifying,
    Superpole,
    Race,
    Hotlap,
    Hotstint,
    HotlapSuperpole,
    Replay,
    #[default]
    None,
}

impl From<SessionType> for model::SessionType {
    fn from(value: SessionType) -> Self {
        match value {
            SessionType::Practice => Self::Practice,
            SessionType::Qualifying => Self::Qualifying,
            SessionType::Superpole => Self::Qualifying,
            SessionType::Race => Self::Race,
            SessionType::Hotlap => Self::Practice,
            SessionType::Hotstint => Self::Practice,
            SessionType::HotlapSuperpole => Self::Practice,
            SessionType::Replay => Self::None,
            SessionType::None => Self::None,
        }
    }
}
