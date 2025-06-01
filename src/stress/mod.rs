mod level;
pub mod trauma;

pub use level::*;
pub use trauma::{Trauma, Traumas};

#[derive(Debug, PartialEq)]
pub struct Tracker<L: Level, T: Traumas> {
    stress: L,
    traumas: T,
}

impl<L: Level, T: Traumas> Default for Tracker<L, T> {
    fn default() -> Self {
        Self {
            stress: Default::default(),
            traumas: Default::default(),
        }
    }
}
