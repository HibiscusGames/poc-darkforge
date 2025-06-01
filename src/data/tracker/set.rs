use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
};

use super::{Error, Tracker};

/// A tracker implementation that stores unique elements in a set with a fixed maximum capacity.
///
/// The `SetTracker` ensures that:
/// - No duplicate elements can be added
/// - The number of elements cannot exceed the capacity `N`
/// - Elements must implement `Clone`, `Debug`, `Display`, `Eq`, and `Hash` traits
///
/// # Type Parameters
///
/// * `T` - The type of elements to track, must implement several traits for equality comparison and display
/// * `N` - A const generic parameter that defines the maximum capacity of the tracker
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetTracker<T: Clone + Debug + Display + Eq + Hash, const N: usize> {
    inner: HashSet<T>,
}

impl<T: Clone + Debug + Display + Eq + Hash, const N: usize> SetTracker<T, N> {
    /// Creates a new `SetTracker` initialized with the provided elements.
    ///
    /// # Arguments
    ///
    /// * `input` - A slice of elements to initialize the tracker with
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error<T>>` - A new tracker if successful, or an error if:
    ///   - The input contains more elements than the capacity `N`
    ///   - The input contains duplicate elements
    ///
    /// # Examples
    ///
    /// ```
    /// # use darkforge::data::tracker::{SetTracker, Tracker};
    ///
    /// let tracker = SetTracker::<u8, 4>::new(&[1, 2, 3]).unwrap();
    /// assert_eq!(3, tracker.count());
    /// ```
    pub fn new(input: &[T]) -> Result<Self, Error<T>> {
        if input.len() > N {
            return Err(Error::TooManyItems(N, input.len()));
        }

        let mut tracker = Self {
            inner: HashSet::with_capacity(N),
        };
        input.iter().try_for_each(|item| tracker.append(item.clone()))?;

        Ok(tracker)
    }
}

impl<T: Clone + Debug + Display + Eq + Hash, const N: usize> Default for SetTracker<T, N> {
    /// Creates a new empty `SetTracker`.
    ///
    /// # Returns
    ///
    /// * `Self` - A new empty tracker
    ///
    /// # Examples
    ///
    /// ```
    /// # use darkforge::data::tracker::{SetTracker, Tracker};
    ///
    /// let tracker = SetTracker::<u8, 4>::default();
    /// assert!(tracker.is_empty());
    /// ```
    fn default() -> Self {
        Self::new(&[]).unwrap()
    }
}

impl<T: Clone + Debug + Display + Eq + Hash, const N: usize> Tracker<T> for SetTracker<T, N> {
    /// Adds a new element to the tracker if it doesn't already exist and there is capacity.
    ///
    /// # Arguments
    ///
    /// * `value` - The element to add to the tracker
    ///
    /// # Returns
    ///
    /// * `Result<(), Error<T>>` - Success if the element was added, or an error if:
    ///   - The element is a duplicate
    ///   - The tracker is already at maximum capacity
    ///
    /// # Examples
    ///
    /// ```
    /// # use darkforge::data::tracker::{SetTracker, Tracker};
    ///
    /// let mut tracker = SetTracker::<u8, 4>::default();
    /// tracker.append(1).expect("should append");
    /// ```
    fn append(&mut self, value: T) -> Result<(), Error<T>> {
        if self.inner.contains(&value) {
            return Err(Error::Duplicate(value));
        }

        if self.inner.len() == N {
            return Err(Error::TooManyItems(N, self.inner.len() + 1));
        }

        self.inner.insert(value);
        Ok(())
    }

    /// Returns a vector containing references to all elements in the tracker.
    ///
    /// # Returns
    ///
    /// * `Vec<&T>` - A vector of references to the tracked elements
    ///
    /// # Examples
    ///
    /// ```
    /// # use darkforge::data::tracker::{SetTracker, Tracker};
    ///
    /// let tracker = SetTracker::<u8, 4>::new(&[1, 2, 3]).unwrap();
    /// let elements = tracker.list();
    /// assert_eq!(3, elements.len());
    /// ```
    fn list(&self) -> Vec<&T> {
        self.inner.iter().collect()
    }

    /// Returns the number of elements currently in the tracker.
    ///
    /// # Returns
    ///
    /// * `usize` - The count of elements in the tracker
    ///
    /// # Examples
    ///
    /// ```
    /// # use darkforge::data::tracker::{SetTracker, Tracker};
    ///
    /// let tracker = SetTracker::<u8, 4>::new(&[1, 2, 3]).unwrap();
    /// assert_eq!(3, tracker.count());
    /// ```
    fn count(&self) -> usize {
        self.inner.len()
    }

    /// Checks if the tracker contains no elements.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the tracker is empty, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// # use darkforge::data::tracker::{SetTracker, Tracker};
    ///
    /// let tracker = SetTracker::<u8, 4>::default();
    /// assert!(tracker.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Checks if the tracker is at maximum capacity.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the tracker is full, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// # use darkforge::data::tracker::{SetTracker, Tracker};
    ///
    /// let tracker = SetTracker::<u8, 4>::new(&[1, 2, 3, 4]).unwrap();
    /// assert!(tracker.is_full());
    /// ```
    fn is_full(&self) -> bool {
        self.inner.len() == N
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_set_tracker_is_empty() {
        let tracker = SetTracker::<u8, 4>::default();

        assert!(tracker.is_empty());
    }

    #[test]
    fn test_set_tracker_add_to_empty() {
        let mut tracker = SetTracker::<u8, 4>::default();

        tracker.append(1).expect("should append");

        assert_eq!(1, tracker.count());

        assert_eq!(vec![&1u8], tracker.list());
    }

    #[test]
    fn test_set_tracker_add_to_full_fails() {
        let mut tracker = SetTracker::<u8, 2>::default();

        tracker.append(1).expect("should append");
        tracker.append(2).expect("should append");

        let err = tracker.append(3).expect_err("should have failed");

        assert_eq!(Error::TooManyItems(2, 3), err);
    }

    #[test]
    fn test_set_tracker_is_full() {
        let tracker = SetTracker::<u8, 4>::new(&[1, 2, 3, 4]).unwrap();

        assert!(tracker.is_full());
    }

    #[test]
    fn test_new_set_tracker_cannot_be_created_with_duplicates() {
        let err = SetTracker::<u8, 2>::new(&[3, 3]).expect_err("should have failed");

        assert_eq!(Error::Duplicate(3), err);
    }

    #[test]
    fn test_set_tracker_appends_duplicate_fails() {
        let mut tracker = SetTracker::<u8, 4>::default();

        tracker.append(1).expect("should append");
        let err = tracker.append(1).expect_err("should have failed");

        assert_eq!(Error::Duplicate(1), err);
    }
}
