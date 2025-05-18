use rand::distr::{Distribution, StandardUniform, Uniform};

/// Defines the order in which dice rolls should be sorted.
///
/// Used when rolling multiple dice to determine the order of results.
pub enum SortOrder {
    /// Sort dice values from lowest to highest.
    Ascending,
    /// Sort dice values from highest to lowest.
    Descending,
}

/// A trait for dice pools that can generate random rolls based on a specific distribution.
///
/// Implementations of this trait can represent different types of dice with varying
/// numbers of sides and probability distributions.
pub trait DicePool<D: Distribution<u8> = StandardUniform> {
    /// Returns a reference to the underlying distribution used for generating random values.
    fn distribution(&self) -> &D;

    /// Rolls a specified number of dice and returns the results sorted according to the given order.
    ///
    /// # Arguments
    /// * `n` - The number of dice to roll (between 1 and 255)
    /// * `sort_order` - The order in which to sort the dice results
    ///
    /// # Returns
    /// A vector of dice values sorted according to the specified order
    fn roll(&self, n: u8, sort_order: SortOrder) -> Vec<u8> {
        let mut rolls: Vec<u8> = self.distribution().sample_iter(&mut rand::rng()).take(n as usize).collect();

        sort_order.sort(&mut rolls);

        rolls
    }
}

/// A convenience type alias for a standard six-sided die (d6).
///
/// Uses a uniform distribution by default, with values from 1 to 6.
pub type D6<D = Uniform<u8>> = DN<6, D>;

/// A generic dice type with a configurable number of sides and distribution.
///
/// The type parameter `SIDES` determines the maximum value of the die.
/// The type parameter `D` determines the probability distribution used for rolls.
pub struct DN<const SIDES: u8, D: Distribution<u8> = Uniform<u8>>(D);

impl<const SIDES: u8> Default for DN<SIDES> {
    /// Creates a default dice with `SIDES` sides using a uniform distribution.
    ///
    /// The distribution ranges from 1 to `SIDES` inclusive.
    fn default() -> Self {
        Self::new(Uniform::new_inclusive(1, SIDES).expect("Invalid range"))
    }
}

impl<const SIDES: u8, D: Distribution<u8>> DN<SIDES, D> {
    /// Creates a new dice with the specified distribution.
    ///
    /// # Arguments
    /// * `distribution` - The probability distribution to use for generating dice values
    pub fn new(distribution: D) -> Self {
        Self(distribution)
    }
}

impl<const SIDES: u8, D: Distribution<u8>> DicePool<D> for DN<SIDES, D> {
    fn distribution(&self) -> &D {
        &self.0
    }

    fn roll(&self, n: u8, sort_order: SortOrder) -> Vec<u8> {
        let mut rolls: Vec<u8> = self.distribution().sample_iter(&mut rand::rng()).take(n as usize).collect();
        sort_order.sort(&mut rolls);

        rolls
    }
}

impl SortOrder {
    /// Sorts a slice of dice rolls according to the specified order.
    ///
    /// # Arguments
    /// * `rolls` - A mutable slice of dice roll values to be sorted in place
    fn sort(&self, rolls: &mut [u8]) {
        match self {
            SortOrder::Ascending => rolls.sort_unstable(),
            SortOrder::Descending => rolls.sort_unstable_by(|a, b| b.cmp(a)),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn test_returns_correct_number_of_dice(count in 1u8..=255) {
            let result = D6::default().roll(count, SortOrder::Ascending);
            prop_assert_eq!(result.len(), count as usize, "should return correct number of dice");
        }

        #[test]
        fn test_dice_values_are_within_range(count in 1u8..=255) {
            let result = D6::default().roll(count, SortOrder::Ascending);
            prop_assert!(result.iter().all(|&d| (1..=6).contains(&d)), "should be within range 1-6");
        }

        #[test]
        fn test_dice_are_sorted_descending(count in 1u8..=255) {
            let result = D6::default().roll(count, SortOrder::Descending);
            prop_assert!(result.windows(2).all(|w| w[0] >= w[1]), "should be sorted descending");
        }

        #[test]
        fn test_dice_are_sorted_ascending(count in 1u8..=255) {
            let result = D6::default().roll(count, SortOrder::Ascending);
            prop_assert!(result.windows(2).all(|w| w[0] <= w[1]), "should be sorted ascending");
        }
    }
}
