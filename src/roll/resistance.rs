use core::marker::PhantomData;

use rand::distr::{Distribution, Uniform};

use super::{Rating, Resistance, ResistanceOutcome};
use crate::dice::{D6, DicePool, SortOrder};

/// A dice pool for performing resistance rolls.
///
/// This struct wraps a generic dice pool and provides methods for rolling
/// resistance dice according to the game rules.
pub struct ResistanceDicePool<T: DicePool<D>, D: Distribution<u8>> {
    /// The underlying dice pool used for generating random values.
    pool: T,
    /// Phantom data to track the distribution type parameter.
    _phantom: PhantomData<D>,
}

impl<T: DicePool<D>, D: Distribution<u8>> ResistanceDicePool<T, D> {
    /// Creates a new resistance dice pool with the specified underlying dice pool.
    ///
    /// # Arguments
    /// * `pool` - The dice pool to use for generating random values
    pub fn new(pool: T) -> Self {
        Self { pool, _phantom: PhantomData }
    }
}

impl Default for ResistanceDicePool<D6<Uniform<u8>>, Uniform<u8>> {
    /// Creates a default resistance dice pool using standard six-sided dice.
    ///
    /// This uses a uniform distribution for values from 1 to 6.
    fn default() -> Self {
        Self {
            pool: D6::default(),
            _phantom: PhantomData,
        }
    }
}

impl<T: DicePool<D>, D: Distribution<u8>> Resistance for ResistanceDicePool<T, D> {
    /// Rolls dice for a resistance check and evaluates the outcome including stress cost.
    ///
    /// # Arguments
    /// * `n` - The number of dice to roll (pool size)
    ///
    /// # Returns
    /// A `ResistanceOutcome` containing the dice results, rating, and stress cost
    ///
    /// # Special cases
    /// If `n` is 0 (zero dice pool), rolls 2 dice and uses only the lowest die for rating and stress.
    /// Otherwise, rolls `n` dice and uses the highest 1-2 dice for rating and the highest die for stress.
    fn roll(&self, n: u8) -> ResistanceOutcome {
        if n == 0 {
            let rolled = self.pool.roll(2, SortOrder::Ascending);
            let lowest = rolled
                .first()
                .cloned()
                .expect("rolled must not be empty, this should not be possible with correct code");
            let rating = Rating::evaluate(vec![lowest]);

            ResistanceOutcome {
                dice: rolled,
                rating: rating.clone(),
                stress: calculate_stress(rating, lowest),
            }
        } else {
            let rolled = self.pool.roll(n, SortOrder::Descending);
            let highest = rolled
                .first()
                .cloned()
                .expect("rolled must not be empty, this should not be possible with correct code");
            let rating = Rating::evaluate(rolled.clone());

            ResistanceOutcome {
                dice: rolled,
                rating: rating.clone(),
                stress: calculate_stress(rating, highest),
            }
        }
    }
}

/// Calculates the stress cost of a resistance roll based on the rating and die value.
///
/// # Arguments
/// * `rating` - The outcome rating of the resistance roll
/// * `val` - The die value used for stress calculation (highest die for normal pools, lowest for zero pools)
///
/// # Returns
/// The stress cost as a signed integer:
/// * -1 for Critical successes (stress reduction)
/// * 0-5 for other outcomes, depending on the die value (6 → 0, 5 → 1, etc.)
fn calculate_stress(rating: Rating, val: u8) -> i8 {
    if rating == Rating::Critical {
        return -1;
    }

    const BASE_STRESS_COST: i8 = 6;

    BASE_STRESS_COST - val as i8
}

#[cfg(test)]
mod tests {
    use std::{
        iter::{Cycle, Iterator},
        sync::{Arc, Mutex},
    };

    use proptest::prelude::*;
    use rand::{Rng, distr::Distribution};
    use rstest::rstest;

    use super::*;

    proptest! {
        #[test]
        fn test_resistance_roll_returns_correct_number_of_dice(pool_size in 1_u8..=255_u8) {
            let outcome = ResistanceDicePool::default().roll(pool_size);
            prop_assert_eq!(outcome.dice.len(), pool_size as usize);
        }

        #[test]
        fn test_resistance_dice_are_sorted_descending(pool_size in 1_u8..=255) {
            let result = ResistanceDicePool::default().roll(pool_size);
            prop_assert!(result.dice.windows(2).all(|w| w[0] >= w[1]), "should be sorted descending");
        }

        #[test]
        fn test_zero_pool_resistance_roll_rolls_two_dice(_ in 0u8..=0u8) {
            let outcome = ResistanceDicePool::default().roll(0);
            assert_eq!(outcome.dice.len(), 2, "zero pool should roll 2 dice");
        }

        #[test]
        fn test_zero_pool_uses_lowest_die_for_result(_ in 0u8..=0u8) {
            let outcome = ResistanceDicePool::default().roll(0);
            let die1 = outcome.dice[0];
            let die2 = outcome.dice[1];

            prop_assert!(die1 <= die2, "the lowest die should be first");
        }

        #[test]
        fn test_stress_calculation_follows_rules(pool_size in 1u8..=255_u8) {
            let outcome = ResistanceDicePool::default().roll(pool_size);
            let die_value = outcome.dice().first().cloned().expect("dice must not be empty");
            let expected_stress = if outcome.rating() == Rating::Critical { -1 } else { match die_value {
                6 => 0,
                5 => 1,
                4 => 2,
                3 => 3,
                2 => 4,
                _ => 5,
            }};

            prop_assert_eq!(outcome.stress(), expected_stress);
        }
    }

    #[rstest]
    #[case::critical_success(vec![1, 6, 6], Rating::Critical, -1)]
    #[case::success_with_six(vec![1, 6], Rating::Success, 0)]
    #[case::partial_with_five(vec![1, 5], Rating::Partial, 1)]
    #[case::partial_with_four(vec![1, 4], Rating::Partial, 2)]
    #[case::failure_with_three(vec![1, 3], Rating::Failure, 3)]
    #[case::failure_with_two(vec![1, 2], Rating::Failure, 4)]
    #[case::failure_with_one(vec![1, 1], Rating::Failure, 5)]
    fn test_resistance_roll_evaluates_to_correct_rating_and_stress(#[case] dice: Vec<u8>, #[case] rating: Rating, #[case] stress: i8) {
        let mut expected_dice = dice.clone();

        if dice.len() > 0 {
            expected_dice.sort_unstable_by(|a, b| b.cmp(a));
        }

        assert_eq!(
            ResistanceDicePool::new(StubDicePool::new(dice)).roll(expected_dice.len() as u8),
            ResistanceOutcome {
                dice: expected_dice,
                rating,
                stress
            }
        );
    }

    #[derive(Clone)]
    struct StubDicePool(StaticDistribution<u8>);

    #[derive(Clone)]
    struct StaticDistribution<T>(Arc<Mutex<Cycle<std::vec::IntoIter<T>>>>);

    impl StubDicePool {
        fn new(vec: Vec<u8>) -> Self {
            Self(StaticDistribution::new(vec))
        }
    }

    impl DicePool<StaticDistribution<u8>> for StubDicePool {
        fn distribution(&self) -> &StaticDistribution<u8> {
            &self.0
        }
    }

    impl<T: Clone> StaticDistribution<T> {
        fn new(values: Vec<T>) -> Self {
            Self(Arc::new(Mutex::new(values.into_iter().cycle())))
        }
    }

    impl<T: Clone> Distribution<T> for StaticDistribution<T> {
        fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> T {
            self.0.lock().unwrap().next().unwrap()
        }
    }
}
