#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    Desperate,
    Risky,
    Controlled,
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
}
