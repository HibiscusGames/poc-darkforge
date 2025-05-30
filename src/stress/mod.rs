mod level;
pub mod trauma;

pub use level::*;
use thiserror::Error;

use crate::data::value::Error as ValueErrorReason;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error(transparent)]
    ValueError(#[from] ValueErrorReason),
}
