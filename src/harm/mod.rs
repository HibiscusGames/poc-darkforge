use thiserror::Error;

pub mod severity;

#[derive(Debug, Clone, Copy, Error, Eq, PartialEq)]
pub enum Error {
    #[error("cannot increase severity past Fatal")]
    IncreaseOutOfBounds,
    #[error("cannot decrease severity below Lesser")]
    DecreaseOutOfBounds,
}

pub type Result<T> = std::result::Result<T, Error>;
