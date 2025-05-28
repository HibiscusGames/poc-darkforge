//! Action module handles game mechanics related to positions and effects.
//!
//! This module provides:
//! - Position and Effect enums representing character states
//! - Trading mechanics between positions and effects
//! - Validation and error handling for invalid trades
//! - Helper methods for manipulating positions and effects

pub mod effect;
pub mod position;

use std::ops::{Deref, DerefMut};

use enum_map::{Enum, EnumMap};
use thiserror::Error;

pub use crate::action::{effect::Effect, position::Position};
use crate::data::UnsignedInteger;

const ACTION_MAX: usize = 4;

#[derive(Error, Debug, PartialEq)]
/// Error type for action rolls
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

#[derive(Clone, Copy, Debug, Enum, PartialEq, Eq, Hash)]
/// Action ratings govern how skilled characters are at performing categories of tasks.
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

pub type ActionValue = UnsignedInteger<u8, 0, ACTION_MAX>;

type Result<T> = std::result::Result<T, ActionError>;

#[derive(Debug, Default, PartialEq)]
pub struct Actions(EnumMap<Action, ActionValue>);

impl Deref for Actions {
    type Target = EnumMap<Action, ActionValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Actions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{
        action::Action,
        character::Character,
        data::{Error as DataError, Value, value::Error as ValueError},
    };

    proptest!(
        #[test]
        fn test_set_and_get_action_between_0_and_4(
            action in prop::sample::select(vec![Action::Hunt, Action::Study, Action::Survey, Action::Tinker, Action::Finesse, Action::Prowl, Action::Skirmish, Action::Wreck, Action::Attune, Action::Command, Action::Consort, Action::Sway]),
            value in 0u8..=4u8
        ) {
            let mut character = Character::new("Test Character");

            character.action_mut(action).set(value).expect("should have set action rating");

            assert_eq!(value, character.action(action).get());
        }

        #[test]
        fn test_action_ratings_above_max_are_clamped_to_max(
            action in prop::sample::select(vec![Action::Hunt, Action::Study, Action::Survey, Action::Tinker, Action::Finesse, Action::Prowl, Action::Skirmish, Action::Wreck, Action::Attune, Action::Command, Action::Consort, Action::Sway]),
            value in 5u8..u8::MAX
        ) {
            let mut character = Character::new("Test Character");

            match character.action_mut(action).set(value).expect_err("should have clamped") {
                DataError::Value(ValueError::ClampedMax) => assert!(value > 4, "Action rating clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}"),
            }

            assert_eq!(4, character.action(action).get(), "Action rating should clamp precisely to MAX (4)");
        }

        #[test]
        fn test_increment_action_rating_clamps_to_max(
            action in prop::sample::select(vec![Action::Hunt, Action::Study, Action::Survey, Action::Tinker, Action::Finesse, Action::Prowl, Action::Skirmish, Action::Wreck, Action::Attune, Action::Command, Action::Consort, Action::Sway]),
            increment in 5u8..=u8::MAX
        ) {
            let mut character = Character::new("Test Character");

            match character.action_mut(action).increment(increment).expect_err("should have clamped") {
                DataError::Value(ValueError::ClampedMax) => assert!(increment > 4, "Action rating clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}"),
            }

            assert_eq!(4, character.action(action).get(), "Action rating should clamp precisely to MAX (4)");
        }
    );
}
