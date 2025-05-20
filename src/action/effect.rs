use super::{ActionError, Position, Result};

/// Represents the potency of an action's effect in the fiction.
///
/// Effects range from Zero (no effect) to Extreme (maximum effect).
/// The effect determines how significant the outcome of an action is.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Effect {
    Zero,
    Limited,
    Standard,
    Great,
    Extreme,
}

impl Effect {
    /// Increases the effect by one step (e.g., Limited → Standard).
    ///
    /// If already at the maximum effect (Extreme), returns Extreme.
    pub fn increase(&self) -> Self {
        match self {
            Effect::Zero => Effect::Limited,
            Effect::Limited => Effect::Standard,
            Effect::Standard => Effect::Great,
            Effect::Great => Effect::Extreme,
            Effect::Extreme => Effect::Extreme,
        }
    }

    /// Decreases the effect by one step (e.g., Standard → Limited).
    ///
    /// If already at the minimum effect (Zero), returns Zero.
    pub fn decrease(&self) -> Self {
        match self {
            Effect::Extreme => Effect::Great,
            Effect::Great => Effect::Standard,
            Effect::Standard => Effect::Limited,
            Effect::Limited => Effect::Zero,
            Effect::Zero => Effect::Zero,
        }
    }

    /// Ensures the effect is at least the specified value.
    ///
    /// If the effect is less than the specified value, returns the specified value.
    /// Otherwise, returns the original effect.
    pub fn at_least(self, value: Self) -> Self {
        if self < value { value } else { self }
    }

    /// Ensures the effect is at most the specified value.
    ///
    /// If the effect is greater than the specified value, returns the specified value.
    /// Otherwise, returns the original effect.
    pub fn at_most(self, value: Self) -> Self {
        if self > value { value } else { self }
    }

    /// Attempts to trade the current effect for an improved position.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The effect is already at or below the lowest allowed level (Limited)
    /// - The position is already at the highest level (Controlled)
    ///
    /// # Returns
    ///
    /// A tuple with the diminished effect and the improved position
    pub fn trade_for_position(&self, position: Position) -> Result<(Self, Position)> {
        if *self <= Effect::Limited {
            return Err(ActionError::EffectClampedLow(Effect::Limited));
        }

        if position == Position::Controlled {
            return Err(ActionError::PositionClampedHigh(Position::Controlled));
        }

        Ok((self.decrease(), position.improve()))
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::from_zero_to_limited(Effect::Zero, Effect::Limited)]
    #[case::from_limited_to_standard(Effect::Limited, Effect::Standard)]
    #[case::from_standard_to_great(Effect::Standard, Effect::Great)]
    #[case::from_great_to_extreme(Effect::Great, Effect::Extreme)]
    #[case::from_extreme_to_extreme(Effect::Extreme, Effect::Extreme)]
    fn test_effect_increases(#[case] from: Effect, #[case] to: Effect) {
        assert_eq!(from.increase(), to);
    }

    #[rstest]
    #[case::from_extreme_to_great(Effect::Extreme, Effect::Great)]
    #[case::from_great_to_standard(Effect::Great, Effect::Standard)]
    #[case::from_standard_to_limited(Effect::Standard, Effect::Limited)]
    #[case::from_limited_to_none(Effect::Limited, Effect::Zero)]
    #[case::from_zero_to_zero(Effect::Zero, Effect::Zero)]
    fn test_effect_decreases(#[case] from: Effect, #[case] to: Effect) {
        assert_eq!(from.decrease(), to);
    }

    #[rstest]
    #[case::from_zero_to_zero(Effect::Zero, Effect::Zero, Effect::Zero)]
    #[case::from_limited_to_limited(Effect::Limited, Effect::Limited, Effect::Limited)]
    #[case::from_standard_to_standard(Effect::Standard, Effect::Standard, Effect::Standard)]
    #[case::from_great_to_great(Effect::Great, Effect::Great, Effect::Great)]
    #[case::from_extreme_to_extreme(Effect::Extreme, Effect::Extreme, Effect::Extreme)]
    fn test_at_least_edge_cases(#[case] from: Effect, #[case] clamp: Effect, #[case] to: Effect) {
        assert_eq!(from.at_least(clamp), to);
    }

    #[rstest]
    #[case::from_zero_to_zero(Effect::Zero, Effect::Zero, Effect::Zero)]
    #[case::from_limited_to_limited(Effect::Limited, Effect::Limited, Effect::Limited)]
    #[case::from_standard_to_standard(Effect::Standard, Effect::Standard, Effect::Standard)]
    #[case::from_great_to_great(Effect::Great, Effect::Great, Effect::Great)]
    #[case::from_extreme_to_extreme(Effect::Extreme, Effect::Extreme, Effect::Extreme)]
    fn test_at_most_edge_cases(#[case] from: Effect, #[case] clamp: Effect, #[case] to: Effect) {
        assert_eq!(from.at_most(clamp), to);
    }

    #[rstest]
    #[case::standard_to_limited_for_position(Effect::Great, Position::Risky, Effect::Standard, Position::Controlled)]
    #[case::great_to_standard_for_position(Effect::Standard, Position::Risky, Effect::Limited, Position::Controlled)]
    #[case::standard_to_limited_for_risky_position(Effect::Standard, Position::Desperate, Effect::Limited, Position::Risky)]
    fn test_trade_effect_for_position(
        #[case] initial_effect: Effect, #[case] initial_position: Position, #[case] expected_effect: Effect, #[case] expected_position: Position,
    ) {
        let (new_effect, new_position) = initial_effect
            .trade_for_position(initial_position)
            .expect("should have traded successfully");
        assert_eq!(new_position, expected_position);
        assert_eq!(new_effect, expected_effect);
    }

    #[rstest]
    #[case::cannot_decrease_below_limited(Effect::Limited, Position::Desperate, ActionError::EffectClampedLow(Effect::Limited))]
    #[case::cannot_decrease_below_zero(Effect::Zero, Position::Desperate, ActionError::EffectClampedLow(Effect::Limited))]
    #[case::cannot_increase_above_controlled(Effect::Great, Position::Controlled, ActionError::PositionClampedHigh(Position::Controlled))]
    fn test_fail_to_trade_effect_for_position(#[case] initial_effect: Effect, #[case] initial_position: Position, #[case] error: ActionError) {
        let err = initial_effect.trade_for_position(initial_position).expect_err("should have failed");
        assert_eq!(err, error);
    }

    proptest! {
        #[test]
        fn test_at_least_prevents_decrease_below_minimum(
            effect in prop_oneof![Just(Effect::Zero), Just(Effect::Limited), Just(Effect::Standard), Just(Effect::Great), Just(Effect::Extreme)],
            clamp in prop_oneof![Just(Effect::Zero), Just(Effect::Limited), Just(Effect::Standard), Just(Effect::Great), Just(Effect::Extreme)]
        ) {
            let decreased = effect.decrease();
            if effect != Effect::Zero {
                prop_assert!(decreased < effect, "Decreased effect {:?} should be less than original effect {:?}", decreased, effect)
            }

            let clamped = decreased.at_least(clamp.clone());
            prop_assert!(clamped >= clamp, "Clamped effect {:?} should not be less than clamp value {:?}", clamped, clamp);
        }

        #[test]
        fn test_at_most_prevents_increase_above_maximum(
            effect in prop_oneof![Just(Effect::Zero), Just(Effect::Limited), Just(Effect::Standard), Just(Effect::Great), Just(Effect::Extreme)],
            clamp in prop_oneof![Just(Effect::Zero), Just(Effect::Limited), Just(Effect::Standard), Just(Effect::Great), Just(Effect::Extreme)]
        ) {
            let increased = effect.increase();
            if effect != Effect::Extreme {
                prop_assert!(increased > effect, "Increased effect {:?} should be greater than original effect {:?}", increased, effect)
            }

            let clamped = increased.at_most(clamp.clone());
            prop_assert!(clamped <= clamp, "Clamped effect {:?} should not be greater than clamp value {:?}", clamped, clamp);
        }
    }
}
