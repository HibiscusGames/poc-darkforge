use rand::distr::{Distribution, StandardUniform, Uniform};

pub enum SortOrder {
    Ascending,
    Descending,
}

pub trait DicePool<D: Distribution<u8> = StandardUniform> {
    fn distribution(&self) -> &D;

    fn roll(&self, n: u8, sort_order: SortOrder) -> Vec<u8> {
        let mut rolls: Vec<u8> = self.distribution().sample_iter(&mut rand::rng()).take(n as usize).collect();

        sort_order.sort(&mut rolls);

        rolls
    }
}

pub type D6<D = Uniform<u8>> = DN<6, D>;

pub struct DN<const SIDES: u8, D: Distribution<u8> = Uniform<u8>>(D);

impl<const SIDES: u8> Default for DN<SIDES> {
    fn default() -> Self {
        Self::new(Uniform::new_inclusive(1, SIDES).expect("Invalid range"))
    }
}

impl<const SIDES: u8, D: Distribution<u8>> DN<SIDES, D> {
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
