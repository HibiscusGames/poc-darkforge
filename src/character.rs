//! Implements character mechanics including action ratings, harm, and trauma.
//!
//! Characters in the game have:
//! - Attributes (Insight, Prowess, Resolve)
//! - Action ratings (ranging from 0-4)
//! - Harm tracking
//! - Trauma tracking

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::data::value::UnsignedInteger;

const ACTION_MAX: usize = 4;
const STRESS_MAX: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    // Insight
    /// When you Hunt, you carefully track a target
    Hunt,
    /// When you `Study`, you scrutinize details and interpret evidence.
    Study,
    /// When you `Survey`, you observe the situation and anticipate outcomes.
    Survey,
    /// When you `Tinker`, you fiddle with devices and mechanisms.
    Tinker,
    // Prowess
    /// When you `Finesse`, you employ dextrous manipulation or subtle misdirection.
    Finesse,
    /// When you `Prowl`, you traverse skilfully and quietly.
    Prowl,
    /// When you `Skirmish`, you entangle a target in close combat so they can’t easily escape.
    Skirmish,
    /// When you `Wreck`, you unleash savage force.
    Wreck,
    // Resolve
    /// When you `Attune`, you open your mind to arcane power.
    Attune,
    /// When you `Command`, you compel swift obedience.
    Command,
    /// When you `Consort`, you socialize with friends and contacts.
    Consort,
    /// When you `Sway`, you influence with guile, charm or argument.
    Sway,
}

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

/// A character represents a member of the crew controlled by the player.
///
/// Characters have:
/// - A name
/// - A set of skill ratings for actions
/// - A stress tracker
/// - A trauma tracker
/// - A harm tracker
#[derive(Debug, PartialEq)]
pub struct Character {
    /// The name of the character.
    name: String,
    /// The skill ratings for the actions the character can perform.
    actions: Actions,
    /// The stress tracker for the character.
    stress: Stress,
    /// The trauma tracker for the character.
    traumas: Traumas,
}

type Actions = HashMap<Action, ActionValue>;
type ActionValue = UnsignedInteger<u8, 0, ACTION_MAX>;
type Stress = UnsignedInteger<u8, 0, STRESS_MAX>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Traumas([Option<Trauma>; 4]);

impl Character {
    pub fn new(name: &str) -> Self {
        Character {
            name: name.to_string(),
            actions: init_actions(),
            stress: Stress::default(),
            traumas: Traumas([None; 4]),
        }
    }

    pub fn action(&self, action: Action) -> Option<&ActionValue> {
        self.actions.get(&action)
    }

    pub fn action_mut(&mut self, action: Action) -> Option<&mut ActionValue> {
        self.actions.get_mut(&action)
    }

    pub fn stress(&self) -> &Stress {
        &self.stress
    }

    pub fn stress_mut(&mut self) -> &mut Stress {
        &mut self.stress
    }

    pub fn traumas(&self) -> &Traumas {
        &self.traumas
    }

    /// Returns true if the character has pending trauma (stress level at maximum)
    pub fn has_pending_trauma(&self) -> bool {
        self.stress.get() >= STRESS_MAX as u8
    }
}

fn init_actions() -> Actions {
    let mut actions = Actions::with_capacity(12);
    actions.insert(Action::Hunt, ActionValue::default());
    actions.insert(Action::Study, ActionValue::default());
    actions.insert(Action::Survey, ActionValue::default());
    actions.insert(Action::Tinker, ActionValue::default());
    actions.insert(Action::Finesse, ActionValue::default());
    actions.insert(Action::Prowl, ActionValue::default());
    actions.insert(Action::Skirmish, ActionValue::default());
    actions.insert(Action::Wreck, ActionValue::default());
    actions.insert(Action::Attune, ActionValue::default());
    actions.insert(Action::Command, ActionValue::default());
    actions.insert(Action::Consort, ActionValue::default());
    actions.insert(Action::Sway, ActionValue::default());

    actions
}

impl Traumas {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|t| t.is_none())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::data::value::Error as ValueError;

    proptest! {
        #[test]
        fn test_set_and_get_action_between_0_and_4(
            action in prop::sample::select(vec![Action::Hunt, Action::Study, Action::Survey, Action::Tinker, Action::Finesse, Action::Prowl, Action::Skirmish, Action::Wreck, Action::Attune, Action::Command, Action::Consort, Action::Sway]),
            value in 0u8..=4u8
        ) {
            let mut character = Character::new("Test Character");

            character.action_mut(action).expect("should have found action").set(value).expect("should have set action rating");

            assert_eq!(value, character.action(action).expect("should have found action").get());
        }

        #[test]
        fn test_action_ratings_above_max_are_clamped_to_max(
            action in prop::sample::select(vec![Action::Hunt, Action::Study, Action::Survey, Action::Tinker, Action::Finesse, Action::Prowl, Action::Skirmish, Action::Wreck, Action::Attune, Action::Command, Action::Consort, Action::Sway]),
            value in 5u8..u8::MAX
        ) {
            let mut character = Character::new("Test Character");

            match character.action_mut(action).expect("should have found action").set(value).expect_err("should have clamped") {
                ValueError::ClampedMax => assert!(value > 4, "Action rating clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}"),
            }

            assert_eq!(4, character.action(action).expect("should have found action").get(), "Action rating should clamp precisely to MAX (4)");
        }

        #[test]
        fn test_increment_action_rating_clamps_to_max(
            action in prop::sample::select(vec![Action::Hunt, Action::Study, Action::Survey, Action::Tinker, Action::Finesse, Action::Prowl, Action::Skirmish, Action::Wreck, Action::Attune, Action::Command, Action::Consort, Action::Sway]),
            increment in 5u8..=u8::MAX
        ) {
            let mut character = Character::new("Test Character");

            match character.action_mut(action).expect("should have found action").increment(increment).expect_err("should have clamped") {
                ValueError::ClampedMax => assert!(increment > 4, "Action rating clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}"),
            }

            assert_eq!(4, character.action(action).expect("should have found action").get(), "Action rating should clamp precisely to MAX (4)");
        }

        #[test]
        fn test_set_and_get_stress_level_between_0_and_10(
            stress in 0u8..=10u8
        ) {
            let mut character = Character::new("Test Character");

            character.stress_mut().set(stress).expect("should have set stress level");

            assert_eq!(stress, character.stress().get());
        }

        #[test]
        fn test_setting_stress_levels_above_max_clamp_to_max(
            stress in 11u8..=u8::MAX
        ) {
            let mut character = Character::new("Test Character");

            match character.stress_mut().set(stress).expect_err("should have clamped") {
                ValueError::ClampedMax => assert!(stress > 10, "Stress level clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}")
            }

            assert_eq!(STRESS_MAX as u8, character.stress().get(), "Stress level should clamp precisely to MAX ({STRESS_MAX})");
        }

        #[test]
        fn test_incrementing_stress_levels_above_max_clamp_to_max(
            stress in 0u8..=10u8
        ) {
            let mut character = Character::new("Test Character");
            let increment = (STRESS_MAX + 1) as u8 - stress;

            character.stress_mut().set(stress).expect("should have set stress level");
            match character.stress_mut().increment(increment).expect_err("should have clamped") {
                ValueError::ClampedMax => assert!(stress + increment > STRESS_MAX as u8, "Stress level clamped when it was lower than max ({stress} + {increment} < {STRESS_MAX})"),
                e => panic!("unexpected error: {e:?}")
            }

            assert_eq!(STRESS_MAX as u8, character.stress().get(), "Stress level should clamp precisely to MAX ({STRESS_MAX})");
        }
    }

    #[test]
    fn test_returns_false_when_checking_for_pending_trauma_given_a_character_with_a_stress_level_below_10() {
        let mut character = Character::new("Test Character");
        character.stress_mut().set(9).expect("should have set stress level");

        assert!(!character.has_pending_trauma());
    }

    #[test]
    fn test_returns_true_when_checking_for_pending_trauma_given_a_character_with_a_stress_level_of_10() {
        let mut character = Character::new("Test Character");
        character.stress_mut().set(10).expect("should have set stress level");

        assert!(character.has_pending_trauma());
    }

    #[test]
    fn test_new_character_has_empty_trauma_list() {
        let character = Character::new("Test Character");

        assert!(character.traumas().is_empty());
    }
}
