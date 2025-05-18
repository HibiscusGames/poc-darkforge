/// Implementation of action rolls and their outcomes.
mod action;
/// Implementation of resistance rolls and their outcomes.
mod resistance;

pub use action::*;
pub use resistance::*;

/// Represents the possible outcomes of dice rolls in the game system.
#[derive(Clone, Debug, PartialEq)]
pub enum Rating {
    /// The best possible outcome, representing an exceptional success.
    Critical,
    /// A full success, achieving the intended goal without complications.
    Success,
    /// A mixed result with some success but also complications.
    Partial,
    /// A failed attempt that does not achieve the intended goal.
    Failure,
}

/// Trait for objects that can perform action rolls.
///
/// Actions represent attempts to achieve goals and overcome obstacles.
pub trait Action {
    /// Rolls dice for an action and returns the outcome.
    ///
    /// # Arguments
    /// * `n` - The number of dice to roll (pool size)
    ///
    /// # Returns
    /// An `ActionOutcome` containing the dice results and rating
    fn roll(&self, n: u8) -> ActionOutcome;
}

/// Trait for objects that can perform resistance rolls.
///
/// Resistances represent attempts to avoid or mitigate negative consequences.
pub trait Resistance {
    /// Rolls dice for a resistance check and returns the outcome.
    ///
    /// # Arguments
    /// * `n` - The number of dice to roll (pool size)
    ///
    /// # Returns
    /// A `ResistanceOutcome` containing the dice results, rating, and stress cost
    fn roll(&self, n: u8) -> ResistanceOutcome;
}

impl Rating {
    /// Evaluates dice rolls to determine the outcome rating.
    ///
    /// The rating is determined by the highest one or two dice in the roll:
    /// - Two sixes: Critical success
    /// - At least one six: Success
    /// - Highest die is 4 or 5: Partial success
    /// - Otherwise: Failure
    ///
    /// # Arguments
    /// * `rolled` - An iterator of dice roll values
    ///
    /// # Returns
    /// The appropriate `Rating` based on the dice values
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

/// Represents the result of an action roll.
///
/// Contains the dice that were rolled and the outcome rating.
#[derive(Debug, PartialEq)]
pub struct ActionOutcome {
    /// The dice values that were rolled, sorted according to the rules.
    dice: Vec<u8>,
    /// The rating that determines the success level of the action.
    rating: Rating,
}

impl ActionOutcome {
    /// Returns the rating of this action outcome.
    ///
    /// # Returns
    /// A clone of the rating (Critical, Success, Partial, or Failure)
    pub fn rating(&self) -> Rating {
        self.rating.clone()
    }

    /// Returns the dice values that were rolled for this action.
    ///
    /// # Returns
    /// A clone of the dice vector
    pub fn dice(&self) -> Vec<u8> {
        self.dice.clone()
    }
}

/// Represents the result of a resistance roll.
///
/// Contains the dice that were rolled, the outcome rating, and the stress cost.
#[derive(Debug, PartialEq)]
pub struct ResistanceOutcome {
    /// The dice values that were rolled, sorted according to the rules.
    dice: Vec<u8>,
    /// The rating that determines the success level of the resistance.
    rating: Rating,
    /// The amount of stress taken as a result of the resistance roll.
    stress: i8,
}

impl ResistanceOutcome {
    /// Returns the rating of this resistance outcome.
    ///
    /// # Returns
    /// A clone of the rating (Critical, Success, Partial, or Failure)
    pub fn rating(&self) -> Rating {
        self.rating.clone()
    }

    /// Returns the dice values that were rolled for this resistance.
    ///
    /// # Returns
    /// A clone of the dice vector
    pub fn dice(&self) -> Vec<u8> {
        self.dice.clone()
    }

    /// Returns the stress cost of this resistance roll.
    ///
    /// # Returns
    /// The amount of stress taken (negative values represent stress reduction)
    pub fn stress(&self) -> i8 {
        self.stress
    }
}
