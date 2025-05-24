pub mod tracker;
pub mod value;

use thiserror::Error;
pub use tracker::{ArrayTracker, Tracker};
pub use value::{Integer, SignedInteger, UnsignedInteger, Value};

#[derive(Error, Debug)]
pub enum Error {
    #[error("transparent")]
    Value(#[from] value::Error),
    #[error("transparent")]
    Tracker(#[from] tracker::Error),
}
