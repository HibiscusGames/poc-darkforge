use crate::data::{ArrayTracker, Result, UnsignedInteger, Value};

const STRESS_MAX: usize = 10;

/// A trauma is a persistent emotional or psychological condition that affects a character's behaviour and outlook.
/// It is gained as a consequence of maxing out the stress meter during a heist.
/// Traumas can be an effective way to gain experience if they are allowed to affect the heist. But this can have consequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trauma {
    /// You’re not moved by emotional appeals or social bonds.
    Cold,
    /// You’re often lost in reverie, reliving past horrors, seeing things.
    Haunted,
    /// You’re enthralled by one thing: an activity, a person, an ideology.
    Obsessed,
    /// You imagine danger everywhere; you can’t trust others.
    Paranoid,
    /// You have little regard for your own safety or best interests.
    Reckless,
    /// You lose your edge; you become sentimental, passive, gentle.
    Soft,
    /// Your emotional state is volatile. You can instantly rage, or fall into despair, act impulsively, or freeze up.
    Unstable,
    /// You seek out opportunities to hurt people, even for no good reason.
    Vicious,
}

pub type Traumas = ArrayTracker<Trauma, 4>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stress(UnsignedInteger<u8, 0, STRESS_MAX>);

impl Stress {
    /// Returns true if the character has pending trauma (stress level at maximum)
    pub fn has_pending_trauma(&self) -> bool {
        self.get() >= STRESS_MAX as u8
    }
}

impl Value<u8> for Stress {
    fn get(&self) -> u8 {
        self.0.get()
    }

    fn set(&mut self, value: u8) -> Result<u8> {
        self.0.set(value)
    }

    fn increment(&mut self, increment: u8) -> Result<u8> {
        self.0.increment(increment)
    }

    fn decrement(&mut self, decrement: u8) -> Result<u8> {
        self.0.decrement(decrement)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::data::{Error as DataError, value::Error as ValueError};

    proptest! {
        #[test]
        fn test_set_and_get_stress_level_between_0_and_10(
            stress_level in 0u8..=10u8
        ) {
            let mut stress = Stress::default();

            stress.set(stress_level).expect("should have set stress level");

            assert_eq!(stress_level, stress.get());
        }

        #[test]
        fn test_setting_stress_levels_above_max_clamp_to_max(
            stress_level in 11u8..=u8::MAX
        ) {
            let mut stress = Stress::default();

            match stress.set(stress_level).expect_err("should have clamped") {
                DataError::Value(ValueError::ClampedMax) => assert!(stress_level > STRESS_MAX as u8, "Stress level clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}")
            }

            assert_eq!(STRESS_MAX as u8, stress.get(), "Stress level should clamp precisely to MAX ({STRESS_MAX})");
        }

        #[test]
        fn test_incrementing_stress_levels_above_max_clamp_to_max(
            stress_level in 0u8..=10u8
        ) {
            let mut stress = Stress::default();
            let increment = (STRESS_MAX + 1) as u8 - stress_level;

            stress.set(stress_level).expect("should have set stress level");
            match stress.increment(increment).expect_err("should have clamped") {
                DataError::Value(ValueError::ClampedMax) => assert!(stress_level + increment > STRESS_MAX as u8, "Stress level clamped when it was lower than max ({stress_level} + {increment} < {STRESS_MAX})"),
                e => panic!("unexpected error: {e:?}")
            }

            assert_eq!(STRESS_MAX as u8, stress.get(), "Stress level should clamp precisely to MAX ({STRESS_MAX})");
        }
    }

    #[test]
    fn test_returns_false_when_checking_for_pending_trauma_given_a_character_with_a_stress_level_below_10() {
        let mut stress = Stress::default();
        stress.set(9).expect("should have set stress level");

        assert!(!stress.has_pending_trauma());
    }

    #[test]
    fn test_returns_true_when_checking_for_pending_trauma_given_a_character_with_a_stress_level_of_10() {
        let mut stress = Stress::default();
        stress.set(10).expect("should have set stress level");

        assert!(stress.has_pending_trauma());
    }
}
