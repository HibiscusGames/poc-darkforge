//! Provides generic tracking capabilities for fixed-size collections of items.
//!
//! The tracker module offers a flexible way to manage collections of items with a fixed maximum capacity.
//! This is particularly useful for game mechanics that track a limited number of effects, such as:
//! - Character traumas (psychological conditions)
//! - Character harm levels (physical injuries)
//! - Limited inventory slots
//! - Status effects with a maximum count
//!
//! # Examples
//!
//! ```
//! use darkforge::data::tracker::{ArrayTracker, Tracker};
//!
//! // Create an empty tracker that can hold up to 4 integers
//! let mut tracker = ArrayTracker::<i32, 4>::default();
//! assert!(tracker.is_empty());
//!
//! // Add some items
//! tracker.append(42);
//! tracker.append(7);
//! assert_eq!(2, tracker.count());
//!
//! // Get all items as a vector
//! assert_eq!(vec![42, 7], tracker.list());
//! ```
//!
//! # Design
//!
//! The module is built around the `Tracker` trait, which defines the core operations for any tracker
//! implementation.
use std::fmt::{Debug, Display};

use thiserror::Error;

mod array;

pub use array::ArrayTracker;

/// Errors that can occur when working with trackers.
#[derive(Error, Debug, PartialEq)]
pub enum Error<T: Display> {
    /// Attempted to create a tracker with more items than its capacity.
    ///
    /// Contains the capacity and the number of items that was attempted to be added.
    #[error("Too many items: capacity is {0} but length would become {1}")]
    TooManyItems(usize, usize),
    /// Attempted to add a duplicate item to a tracker.
    #[error("Cannot add duplicate item to unique tracker: {0}")]
    Duplicate(T),
}

/// Defines the core operations for tracking a collection of items with a fixed capacity.
///
/// This trait provides a common interface for different tracker implementations,
/// allowing them to be used interchangeably in code that needs to track items.
pub trait Tracker<T: Clone + Display + Eq> {
    /// Adds a new item to the tracker.
    ///
    /// If the tracker is already full, the item will not be added.
    fn append(&mut self, value: T) -> Result<(), Error<T>>;

    /// Returns a vector containing all items currently in the tracker.
    fn list(&self) -> Vec<&T>;

    /// Returns the number of items currently in the tracker.
    fn count(&self) -> usize;

    /// Returns true if the tracker contains no items.
    fn is_empty(&self) -> bool;

    /// Returns true if the tracker is at full capacity.
    fn is_full(&self) -> bool;
}
