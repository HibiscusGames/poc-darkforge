use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use super::{Error, Tracker};

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
pub struct ArrayTracker<T: Clone + Debug + Display + Eq, const N: usize> {
    /// The internal storage for items, using `Option<T>` to represent presence/absence.
    inner: [Option<T>; N],
}

impl<T: Clone + Debug + Display + Eq, const N: usize> ArrayTracker<T, N> {
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
    pub fn new(input: &[Option<T>]) -> Result<Self, Error<T>> {
        if input.len() > N {
            return Err(Error::TooManyItems(N, input.len()));
        }

        let mut inner = [const { None }; N];
        for (i, item) in input.iter().enumerate() {
            inner[i] = item.clone();
        }

        Ok(Self { inner })
    }
}

impl<T: Clone + Debug + Display + Eq, const N: usize> Default for ArrayTracker<T, N> {
    fn default() -> Self {
        Self::new(&[]).unwrap()
    }
}

impl<T: Clone + Debug + Display + Eq, const N: usize> Tracker<T> for ArrayTracker<T, N> {
    fn append(&mut self, value: T) -> Result<(), Error<T>> {
        if self.is_full() {
            return Err(Error::TooManyItems(N, self.count() + 1));
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
