pub mod value;

use thiserror::Error;
pub use value::Value;

#[derive(Error, Debug)]
pub enum Error {
    #[error("transparent")]
    Value(#[from] value::Error),
}
