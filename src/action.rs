use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Position {
    Desperate,
    Risky,
    Controlled,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Effect {
    Zero,
    Limited,
    Standard,
    Great,
    Extreme,
}

#[derive(Error, Debug, PartialEq)]
pub enum ActionError {
    #[error("cannot decrease position below {0:?}")]
    PositionClampedLow(Position),
    #[error("cannot increase position above {0:?}")]
    PositionClampedHigh(Position),
    #[error("cannot increase effect above {0:?}")]
    EffectClampedHigh(Effect),
    #[error("cannot decrease effect below {0:?}")]
    EffectClampedLow(Effect),
}

type Result<T> = std::result::Result<T, ActionError>;

impl Effect {
    pub fn increase(&self) -> Self {
        match self {
            Effect::Zero => Effect::Limited,
            Effect::Limited => Effect::Standard,
            Effect::Standard => Effect::Great,
            Effect::Great => Effect::Extreme,
            Effect::Extreme => Effect::Extreme,
        }
    }

    pub fn decrease(&self) -> Self {
        match self {
            Effect::Extreme => Effect::Great,
            Effect::Great => Effect::Standard,
            Effect::Standard => Effect::Limited,
            Effect::Limited => Effect::Zero,
            Effect::Zero => Effect::Zero,
        }
    }

    pub fn at_least(self, value: Self) -> Self {
        if self < value { value } else { self }
    }

    pub fn at_most(self, value: Self) -> Self {
        if self > value { value } else { self }
    }

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

impl Position {
    pub fn improve(&self) -> Self {
        match self {
            Position::Desperate => Position::Risky,
            Position::Risky => Position::Controlled,
            Position::Controlled => Position::Controlled,
        }
    }

    pub fn diminish(&self) -> Self {
        match self {
            Position::Controlled => Position::Risky,
            Position::Risky => Position::Desperate,
            Position::Desperate => Position::Desperate,
        }
    }

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
        fn test_min_prevents_decrease_below_minimum(
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
        fn test_min_prevents_increase_above_maximum(
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
