use std::error::Error;

/// Implements action mechanics for character actions in the game.
pub mod action;
/// Implements character mechanics including action ratings, harm, and trauma.
pub mod character;
/// Provides data structures and utilities for the game.
pub mod data;
/// Provides generic dice rolling functionality with support for different distributions and sorting orders.
pub mod dice;
/// Implements roll mechanics for actions and resistances, including outcome evaluation.
pub mod roll;
/// Implements stress and trauma mechanics for characters.
pub mod stress;

pub fn run() -> Result<(), Box<dyn Error>> {
    todo!("not implemented")
}
