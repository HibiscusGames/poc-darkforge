use super::{ActionError, Effect, Result};

/// Represents the character's position in the fiction.
///
/// Positions range from Desperate (worst) to Controlled (best).
/// A character's position affects their risk and potential consequences.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Position {
    Desperate,
    Risky,
    Controlled,
}

impl Position {
    /// Improves the position by one step (e.g., Desperate → Risky).
    ///
    /// If already at the maximum position (Controlled), returns Controlled.
    pub fn improve(&self) -> Self {
        match self {
            Position::Desperate => Position::Risky,
            Position::Risky => Position::Controlled,
            Position::Controlled => Position::Controlled,
        }
    }

    /// Diminishes the position by one step (e.g., Controlled → Risky).
    ///
    /// If already at the minimum position (Desperate), returns Desperate.
    pub fn diminish(&self) -> Self {
        match self {
            Position::Controlled => Position::Risky,
            Position::Risky => Position::Desperate,
            Position::Desperate => Position::Desperate,
        }
    }

    /// Attempts to trade the current position for an improved effect.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The position is already at the lowest level (Desperate)
    /// - The effect is already at or above the highest allowed level (Great)
    ///
    /// # Returns
    ///
    /// A tuple with the diminished position and the increased effect
    /// Attempts to trade the current position for an improved effect.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The position is already at the lowest level (Desperate)
    /// - The effect is already at or above the highest allowed level (Great)
    ///
    /// # Returns
    ///
    /// A tuple with the diminished position and the increased effect
    pub fn trade_for_effect(&self, effect: Effect) -> Result<(Self, Effect)> {
        if *self == Position::Desperate {
            return Err(ActionError::PositionClampedLow(Position::Desperate));
        }

        if effect >= Effect::Great {
            return Err(ActionError::EffectClampedHigh(Effect::Great));
        }

        Ok((self.diminish(), effect.increase()))
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::from_desperate_to_risky(Position::Desperate, Position::Risky)]
    #[case::from_risky_to_controlled(Position::Risky, Position::Controlled)]
    #[case::from_controlled_to_controlled(Position::Controlled, Position::Controlled)]
    fn test_position_improves(#[case] from: Position, #[case] to: Position) {
        assert_eq!(from.improve(), to);
    }

    #[rstest]
    #[case::from_controlled_to_risky(Position::Controlled, Position::Risky)]
    #[case::from_risky_to_desperate(Position::Risky, Position::Desperate)]
    #[case::from_desperate_to_desperate(Position::Desperate, Position::Desperate)]
    fn test_position_diminishes(#[case] from: Position, #[case] to: Position) {
        assert_eq!(from.diminish(), to)
    }

    #[rstest]
    #[case::controlled_to_risky_for_effect(Position::Controlled, Effect::Limited, Position::Risky, Effect::Standard)]
    #[case::risky_to_desperate_for_effect(Position::Risky, Effect::Limited, Position::Desperate, Effect::Standard)]
    #[case::controlled_to_risky_for_great_effect(Position::Controlled, Effect::Standard, Position::Risky, Effect::Great)]
    fn test_trade_position_for_effect(
        #[case] initial_position: Position, #[case] initial_effect: Effect, #[case] expected_position: Position, #[case] expected_effect: Effect,
    ) {
        let (new_position, new_effect) = initial_position
            .trade_for_effect(initial_effect)
            .expect("should have traded successfully");
        assert_eq!(new_position, expected_position);
        assert_eq!(new_effect, expected_effect);
    }

    #[rstest]
    #[case::cannot_decrease_below_desperate(Position::Desperate, Effect::Limited, ActionError::PositionClampedLow(Position::Desperate))]
    #[case::cannot_increase_above_great(Position::Controlled, Effect::Great, ActionError::EffectClampedHigh(Effect::Great))]
    #[case::cannot_increase_above_extreme(Position::Controlled, Effect::Extreme, ActionError::EffectClampedHigh(Effect::Great))]
    fn test_fail_to_trade_position_for_effect(#[case] initial_position: Position, #[case] initial_effect: Effect, #[case] error: ActionError) {
        let err = initial_position.trade_for_effect(initial_effect).expect_err("should have failed");
        assert_eq!(err, error);
    }

    proptest! {
        #[test]
        fn test_min_prevents_decrease_below_minimum(
            position in prop_oneof![Just(Position::Desperate), Just(Position::Risky), Just(Position::Controlled)],
        ) {
            let diminished = position.diminish();
            if position != Position::Desperate {
                prop_assert!(diminished < position, "Decreased position {:?} should be less than original position {:?}", diminished, position)
            }
        }

        #[test]
        fn test_min_prevents_increase_above_maximum(
            position in prop_oneof![Just(Position::Desperate), Just(Position::Risky), Just(Position::Controlled)],
        ) {
            let improved = position.improve();
            if position != Position::Controlled {
                prop_assert!(improved > position, "Increased position {:?} should be greater than original position {:?}", improved, position)
            }
        }
    }
}
