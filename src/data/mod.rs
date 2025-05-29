pub mod tracker;
pub mod value;

use std::hash::Hash;

use num_traits::PrimInt;
use thiserror::Error;
pub use tracker::ArrayTracker;
pub use value::{Integer, SignedInteger, UnsignedInteger};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Value(#[from] value::Error),
    #[error(transparent)]
    Tracker(#[from] tracker::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Defines the core operations for tracking a collection of items with a fixed capacity.
///
/// This trait provides a common interface for different tracker implementations,
/// allowing them to be used interchangeably in code that needs to track items.
pub trait Tracker<T: Clone + Eq> {
    /// Adds a new item to the tracker.
    ///
    /// If the tracker is already full, the item will not be added.
    fn append(&mut self, value: T) -> Result<()>;

    /// Returns a vector containing all items currently in the tracker.
    fn list(&self) -> Vec<&T>;

    /// Returns the number of items currently in the tracker.
    fn count(&self) -> usize;

    /// Returns true if the tracker contains no items.
    fn is_empty(&self) -> bool;

    /// Returns true if the tracker is at full capacity.
    fn is_full(&self) -> bool;
}

/// A value that can be incremented and decremented and is clamped to a range.
pub trait Value<I: PrimInt + Hash>: Default + Copy + Clone + PartialEq + Eq + Hash {
    /// Increments the action value by the specified amount.
    ///
    /// Returns `Err(ValueError::Max)` if the action value is already at the maximum.
    fn increment(&mut self, amount: I) -> Result<I>;

    /// Decrements the action value by the specified amount.
    ///
    /// Returns `Err(ValueError::Min)` if the action value is already at the minimum.
    fn decrement(&mut self, amount: I) -> Result<I>;

    /// Sets the action value to the specified amount.
    ///
    /// Returns `Err(ValueError::Max)` if the action value is already at the maximum.
    /// Returns `Err(ValueError::Min)` if the action value is already at the minimum.
    fn set(&mut self, amount: I) -> Result<I>;

    /// Returns the current action value.
    fn get(&self) -> I;
}
