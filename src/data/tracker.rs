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
//! implementation. The primary implementation is `ArrayTracker`, which uses a fixed-size array to
//! store items efficiently.

use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use thiserror::Error;

use super::{Result, Tracker};

/// Errors that can occur when working with trackers.
#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Attempted to create a tracker with more items than its capacity.
    ///
    /// Contains the capacity and the number of items that was attempted to be added.
    #[error("Too many items: capacity is {0} but length would become {1}")]
    TooManyItems(usize, usize),
}

/// An implementation of `Tracker` that uses a fixed-size array to store items.
///
/// The `ArrayTracker` provides an efficient way to track a fixed number of items
/// using a pre-allocated array. This is particularly useful for game mechanics
/// where the maximum number of items is known at compile time.
///
/// The generic parameter `T` is the type of items being tracked, and `N` is the
/// maximum capacity of the tracker.
///
/// # Type Parameters
///
/// * `T` - The type of items being tracked. Must implement `Clone` and `Eq`.
/// * `N` - The maximum capacity of the tracker (const generic parameter).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayTracker<T: Clone + Eq, const N: usize> {
    /// The internal storage for items, using `Option<T>` to represent presence/absence.
    inner: [Option<T>; N],
}

impl<T: Clone + Eq, const N: usize> ArrayTracker<T, N> {
    /// Creates a new `ArrayTracker` initialized with the given items.
    ///
    /// # Arguments
    ///
    /// * `input` - A slice of optional items to initialize the tracker with.
    ///
    /// # Returns
    ///
    /// * `Ok(ArrayTracker)` - If the input slice has a length less than or equal to the capacity.
    /// * `Err(Error::TooManyItems)` - If the input slice has more items than the tracker's capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use darkforge::data::tracker::{ArrayTracker, Tracker};
    ///
    /// // Create a tracker with some initial values
    /// let tracker = ArrayTracker::<i32, 4>::new(&[Some(1), Some(2), None, Some(4)]).unwrap();
    /// assert_eq!(3, tracker.count());
    /// ```
    pub fn new(input: &[Option<T>]) -> Result<Self> {
        if input.len() > N {
            return Err(Error::TooManyItems(N, input.len()).into());
        }

        let mut inner = [const { None }; N];
        for (i, item) in input.iter().enumerate() {
            inner[i] = item.clone();
        }

        Ok(Self { inner })
    }
}

impl<T: Clone + Eq, const N: usize> Default for ArrayTracker<T, N> {
    fn default() -> Self {
        Self::new(&[]).unwrap()
    }
}

impl<T: Clone + Eq, const N: usize> Tracker<T> for ArrayTracker<T, N> {
    fn append(&mut self, value: T) -> Result<()> {
        if self.is_full() {
            return Err(Error::TooManyItems(N, self.count() + 1).into());
        }

        for slot in &mut self.inner {
            if slot.is_none() {
                *slot = Some(value);
                return Ok(());
            }
        }

        unreachable!("No empty slot found despite is_full() check");
    }

    fn list(&self) -> Vec<&T> {
        self.inner.iter().filter_map(|item| item.as_ref()).collect()
    }

    fn count(&self) -> usize {
        self.inner.iter().filter(|item| item.is_some()).count()
    }

    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    fn is_full(&self) -> bool {
        self.count() == N
    }
}

impl<T: Clone + Eq, const N: usize, IDX> Index<IDX> for ArrayTracker<T, N>
where
    IDX: SliceIndex<[Option<T>]>,
{
    type Output = IDX::Output;
    fn index(&self, index: IDX) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T: Clone + Eq, const N: usize, IDX> IndexMut<IDX> for ArrayTracker<T, N>
where
    IDX: SliceIndex<[Option<T>]>,
{
    fn index_mut(&mut self, index: IDX) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

#[cfg(test)]
pub mod test {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn test_array_tracker_is_empty() {
        let tracker = ArrayTracker::<u8, 4>::default();

        assert!(tracker.is_empty());
    }

    #[test]
    fn test_array_tracker_is_full() {
        let tracker = ArrayTracker::<u8, 4>::new(&[Some(1), Some(2), Some(3), Some(4)]).unwrap();

        assert!(tracker.is_full());
    }

    proptest! {
        #[test]
        fn test_array_tracker_appends_values_up_to_max(init in 0u8..=3u8) {
            let mut slice = vec![];
            slice.resize(init as usize, Some(0));

            let mut tracker = ArrayTracker::<u8, 4>::new(&slice).unwrap();

            tracker.append(1).expect("should append");

            // Check that the value was appended
            let expected_count = init as usize + 1;
            assert_eq!(expected_count, tracker.count());

            // Check that the items list contains the values we expect
            let mut expected_items = vec![&0u8; init as usize];
            expected_items.push(&1u8);
            assert_eq!(expected_items, tracker.list());
        }

        #[test]
        fn test_count_prints_correct_value(value in 0u8..=4u8) {
            let mut slice = vec![];
            slice.resize(value as usize, Some(0));

            let tracker = ArrayTracker::<u8, 4>::new(&slice).unwrap();

            assert_eq!(value as usize, tracker.count());
        }
    }
}
