mod action;
mod resistance;

pub use action::*;
pub use resistance::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Rating {
    Critical,
    Success,
    Partial,
    Failure,
}

pub trait Action {
    fn roll(&self, n: u8) -> ActionOutcome;
}

pub trait Resistance {
    fn roll(&self, n: u8) -> ResistanceOutcome;
}

impl Rating {
    fn evaluate(rolled: impl IntoIterator<Item = u8>) -> Self {
        let mut rolled = rolled.into_iter().take(2);

        match (rolled.next(), rolled.next()) {
            (Some(6), Some(6)) => Rating::Critical,
            (Some(6), _) => Rating::Success,
            (Some(4) | Some(5), _) => Rating::Partial,
            _ => Rating::Failure,
        }
    }
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
    stress: i8,
}

impl ResistanceOutcome {
    pub fn rating(&self) -> Rating {
        self.rating.clone()
    }

    pub fn dice(&self) -> Vec<u8> {
        self.dice.clone()
    }

    pub fn stress(&self) -> i8 {
        self.stress
    }
}
