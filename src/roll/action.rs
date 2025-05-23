//! Implementation of action rolls for the dice system.
//!
//! Action rolls are used to determine the success or failure of character actions in the game.
//! This module provides a generic implementation that works with any dice pool and distribution.
use core::marker::PhantomData;

use rand::distr::{Distribution, Uniform};

use super::{Action, ActionOutcome, Rating};
use crate::dice::{D6, DicePool, SortOrder};

/// A dice pool for performing action rolls with configurable dice and distribution.
pub struct ActionDicePool<T: DicePool<D>, D: Distribution<u8>> {
    /// The underlying dice pool used for generating random values.
    pool: T,
    /// Phantom data to track the distribution type parameter.
    _phantom: PhantomData<D>,
}

impl<T: DicePool<D>, D: Distribution<u8>> ActionDicePool<T, D> {
    /// Creates a new action dice pool with the specified underlying dice pool.
    ///
    /// # Arguments
    /// * `pool` - The dice pool to use for generating random values
    pub fn new(pool: T) -> Self {
        Self { pool, _phantom: PhantomData }
    }
}

impl Default for ActionDicePool<D6<Uniform<u8>>, Uniform<u8>> {
    /// Creates a default action dice pool using standard six-sided dice.
    ///
    /// This uses a uniform distribution for values from 1 to 6.
    fn default() -> Self {
        Self {
            pool: D6::default(),
            _phantom: PhantomData,
        }
    }
}

impl<T: DicePool<D>, D: Distribution<u8>> Action for ActionDicePool<T, D> {
    /// Rolls dice for an action and returns the outcome.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of dice to roll, representing the character's rating in the action.
    ///
    /// # Behaviour
    ///
    /// - If `n` is 0 (zero dice pool): Rolls 2 dice sorted in ascending order and uses only the lowest die for rating evaluation.
    /// - If `n` is greater than 0: Rolls `n` dice sorted in descending order and uses all dice for rating evaluation.
    ///
    /// This implements the core "Blades in the Dark" dice mechanics where rolling more dice increases your chances of success.
    fn roll(&self, n: u8) -> ActionOutcome {
        if n == 0 {
            let rolled = self.pool.roll(2, SortOrder::Ascending);
            ActionOutcome {
                dice: rolled.clone(),
                rating: Rating::evaluate(vec![
                    rolled
                        .first()
                        .cloned()
                        .expect("rolled must not be empty, this should not be possible with correct code"),
                ]),
            }
        } else {
            let rolled = self.pool.roll(n, SortOrder::Descending);
            ActionOutcome {
                dice: rolled.clone(),
                rating: Rating::evaluate(rolled),
            }
        }
    }
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
        fn test_dice_pool_returns_correct_number_of_dice(pool_size in 1_u8..=255_u8) {
            let outcome = ActionDicePool::default().roll(pool_size);
            prop_assert_eq!(outcome.dice().len(), pool_size as usize);
        }

        #[test]
        fn test_dice_are_sorted_descending(pool_size in 1u8..=255) {
            let result = ActionDicePool::default().roll(pool_size);
            prop_assert!(result.dice().windows(2).all(|w| w[0] >= w[1]), "should be sorted descending");
        }

        #[test]
        fn test_roll_action_zero_pool_rolls_two_dice(_ in 0u8..=255u8) {
            let outcome = ActionDicePool::default().roll(0);
            assert_eq!(outcome.dice().len(), 2, "Zero pool should roll 2 dice");
        }

        #[test]
        fn test_zero_pool_uses_lowest_die_for_result(_ in 1u8..=255u8) {
            let outcome = ActionDicePool::default().roll(0);
            let die1 = outcome.dice()[0];
            let die2 = outcome.dice()[1];

            prop_assert!(die1 <= die2, "the lowest die should be first")
        }
    }

    #[rstest]
    #[case::return_critical_when_six_and_six(vec![6, 6], Rating::Critical)]
    #[case::return_success_when_six_and_five(vec![5, 6], Rating::Success)]
    #[case::return_partial_when_one_and_five(vec![1, 5], Rating::Partial)]
    #[case::return_partial_when_one_and_four(vec![1, 4], Rating::Partial)]
    #[case::return_failure_when_one_and_three(vec![1, 3], Rating::Failure)]
    #[case::return_failure_when_one_and_two(vec![1, 2], Rating::Failure)]
    #[case::return_failure_when_one_and_one(vec![1, 1], Rating::Failure)]
    fn test_action_roll_evaluates_to_correct_rating(#[case] dice: Vec<u8>, #[case] rating: Rating) {
        let mut expect_dice = dice.clone();
        expect_dice.reverse();

        assert_eq!(
            ActionDicePool::new(StubDicePool::new(dice.clone())).roll(2),
            ActionOutcome { dice: expect_dice, rating }
        );
    }

    #[rstest]
    #[case::return_success_when_six_and_six(vec![6, 6], Rating::Success)]
    #[case::return_partial_when_six_and_five(vec![6, 5], Rating::Partial)]
    #[case::return_partial_when_six_and_four(vec![6, 4], Rating::Partial)]
    #[case::return_failure_when_six_and_three(vec![6, 3], Rating::Failure)]
    #[case::return_failure_when_six_and_two(vec![6, 2], Rating::Failure)]
    #[case::return_failure_when_six_and_one(vec![6, 1], Rating::Failure)]
    fn test_zero_pool_action_roll_evaluates_to_correct_rating(#[case] dice: Vec<u8>, #[case] rating: Rating) {
        let mut expect_dice = dice.clone();
        expect_dice.reverse();

        assert_eq!(
            ActionDicePool::new(StubDicePool::new(dice.clone())).roll(0),
            ActionOutcome { dice: expect_dice, rating }
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
