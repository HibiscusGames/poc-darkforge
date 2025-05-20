pub mod effect;
pub mod position;

use thiserror::Error;

pub use crate::action::{effect::Effect, position::Position};

#[derive(Error, Debug, PartialEq)]
pub enum ActionError {
    #[error("cannot decrease position below {0:?}")]
    PositionClampedLow(Position),
    #[error("cannot increase position above {0:?}")]
    PositionClampedHigh(Position),
    #[error("cannot increase effect above {0:?}")]
    EffectClampedHigh(Effect),
    #[error("cannot decrease effect below {0:?}")]
    EffectClampedLow(Effect),
}

type Result<T> = std::result::Result<T, ActionError>;
