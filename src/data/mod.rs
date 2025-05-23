pub mod tracker;
pub mod value;

use thiserror::Error;
pub use tracker::Tracker;
pub use value::Value;

#[derive(Error, Debug)]
pub enum Error {
    #[error("transparent")]
    Value(#[from] value::Error),
    #[error("transparent")]
    Tracker(#[from] tracker::Error),
}
