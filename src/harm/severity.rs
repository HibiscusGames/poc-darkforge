use super::{Error, Result};

/// Represents severity levels of harm a character can sustain during play.
///
/// Harm is tracked at different severity levels, and too much harm can put a character out of action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    /// Minor harm, ex: Battered, Drained, Distracted, Scared, Confused.
    Lesser,
    /// Moderate harm, ex: Exhausted, Deep Cut to Arm, Concussion, Panicked, Seduced.
    Moderate,
    /// Severe harm, ex: Impaled, Broken Leg, Shot in Chest, Badly Burned, Terrified.
    Severe,
    /// Fatal harm, ex: Electrocuted, Drowned, Stabbed in the Heart.
    Fatal,
}

impl Severity {
    pub fn up(&self) -> Result<Self> {
        match self {
            Severity::Lesser => Ok(Severity::Moderate),
            Severity::Moderate => Ok(Severity::Severe),
            Severity::Severe => Ok(Severity::Fatal),
            Severity::Fatal => Err(Error::IncreaseOutOfBounds),
        }
    }

    pub fn down(&self) -> Option<Self> {
        todo!("not implemented")
    }

    pub fn capacity(&self) -> usize {
        todo!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use proptest::{prelude::*, sample};

    use super::*;

    proptest! {
        #[test]
        fn test_severity_increase_to_fatal((init, expected) in (sample::select(&[(Severity::Lesser, Severity::Moderate), (Severity::Moderate, Severity::Severe), (Severity::Severe, Severity::Fatal)]))) {
            let got = init.up().expect("should have increased severity");

            assert_eq!(got, expected);
        }
    }

    #[test]
    fn test_cannot_increase_severity_past_fatal() {
        let got = Severity::Fatal.up().expect_err("should have failed to increase severity");

        assert_eq!(got, Error::IncreaseOutOfBounds);
    }
}
