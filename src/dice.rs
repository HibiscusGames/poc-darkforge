use rand::distr::{Distribution, Uniform};

pub enum SortOrder {
    Ascending,
    Descending,
}

pub fn roll_d6(n: u8, sort_order: SortOrder) -> Vec<u8> {
    let mut rolls: Vec<u8> = Uniform::new_inclusive(1, 6)
        .unwrap()
        .sample_iter(&mut rand::rng())
        .take(n as usize)
        .collect();
    sort_order.sort(&mut rolls);

    rolls
}

impl SortOrder {
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
            let result = roll_d6(count, SortOrder::Ascending);
            prop_assert_eq!(result.len(), count as usize, "should return correct number of dice");
        }

        #[test]
        fn test_dice_values_are_within_range(count in 1u8..=255) {
            let result = roll_d6(count, SortOrder::Ascending);
            prop_assert!(result.iter().all(|&d| (1..=6).contains(&d)), "should be within range 1-6");
        }

        #[test]
        fn test_dice_are_sorted_descending(count in 1u8..=255) {
            let result = roll_d6(count, SortOrder::Descending);
            prop_assert!(result.windows(2).all(|w| w[0] >= w[1]), "should be sorted descending");
        }

        #[test]
        fn test_dice_are_sorted_ascending(count in 1u8..=255) {
            let result = roll_d6(count, SortOrder::Ascending);
            prop_assert!(result.windows(2).all(|w| w[0] <= w[1]), "should be sorted ascending");
        }
    }
}
