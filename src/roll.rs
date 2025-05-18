use crate::dice::{self, SortOrder};

#[derive(Clone)]
pub enum Rating {
    Critical,
    Success,
    Partial,
    Failure,
}

pub struct Outcome {
    dice: Vec<u8>,
    rating: Rating,
}

impl Outcome {
    pub fn rating(&self) -> Rating {
        self.rating.clone()
    }

    pub fn dice(&self) -> Vec<u8> {
        self.dice.clone()
    }
}

pub fn roll_action(dice_pool: u8) -> Outcome {
    if dice_pool == 0 {
        let rolled = dice::roll_d6(2, SortOrder::Ascending);
        Outcome {
            dice: rolled.clone(),
            rating: evaluate(vec![rolled[0]]),
        }
    } else {
        let rolled = dice::roll_d6(dice_pool, SortOrder::Descending);
        Outcome {
            dice: rolled.clone(),
            rating: evaluate(rolled),
        }
    }
}

fn evaluate(rolled: impl IntoIterator<Item = u8>) -> Rating {
    let mut rolled = rolled.into_iter().take(2);

    match (rolled.next(), rolled.next()) {
        (Some(6), Some(6)) => Rating::Critical,
        (Some(6), _) => Rating::Success,
        (Some(4) | Some(5), _) => Rating::Partial,
        _ => Rating::Failure,
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn test_dice_pool_returns_correct_number_of_dice(pool_size in 1_u8..=255_u8) {
            let outcome = roll_action(pool_size);
            prop_assert_eq!(outcome.dice().len(), pool_size as usize);
        }

        #[test]
        fn test_dice_are_sorted_descending(pool_size in 1u8..=255) {
            let result = roll_action(pool_size);
            prop_assert!(result.dice().windows(2).all(|w| w[0] >= w[1]), "should be sorted descending");
        }

        #[test]
        fn test_roll_action_zero_pool_rolls_two_dice(_ in 0u8..=255u8) {
            let outcome = roll_action(0);
            assert_eq!(outcome.dice().len(), 2, "Zero pool should roll 2 dice");
        }

        #[test]
        fn test_zero_pool_uses_lowest_die_for_result(_ in 1u8..=255u8) {
            let outcome = roll_action(0);
            let die1 = outcome.dice()[0];
            let die2 = outcome.dice()[1];

            prop_assert!(die1 <= die2, "the lowest die should be first")
        }
    }
}
