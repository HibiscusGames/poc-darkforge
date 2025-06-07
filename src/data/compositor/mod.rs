use core::fmt::Debug;

use anyhow::Error as AnyhowError;
use thiserror::Error;

pub mod composite;

use composite::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to parse composite pattern \"{0}\": {1}")]
    ParseError(String, #[source] AnyhowError),
}

pub trait Compositor<K> {
    fn compose(&mut self, key: K) -> String;
}

pub type Result<T> = core::result::Result<T, Error>;
type SubCompositor = dyn Compositor<String>;
