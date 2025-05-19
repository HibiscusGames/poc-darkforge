#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    Desperate,
    Risky,
    Controlled,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Effect {
    Zero,
    Limited,
    Standard,
    Great,
    Extreme,
}

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
}

#[cfg(test)]
mod tests {
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
}
