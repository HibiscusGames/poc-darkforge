mod action;
mod resistance;

pub use action::*;
pub use resistance::*;

pub trait Action {
    fn roll(&self, n: u8) -> ActionOutcome;
}

pub trait Resistance {
    fn roll(&self, n: u8) -> ResistanceOutcome;
}

#[derive(Debug, PartialEq)]
pub struct ActionOutcome {
    dice: Vec<u8>,
    rating: Rating,
}

impl ActionOutcome {
    pub fn rating(&self) -> Rating {
        self.rating.clone()
    }

    pub fn dice(&self) -> Vec<u8> {
        self.dice.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct ResistanceOutcome {
    dice: Vec<u8>,
    rating: Rating,
    stress: u8,
}
