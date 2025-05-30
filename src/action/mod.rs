//! Action module handles game mechanics related to positions and effects.
//!
//! This module provides:
//! - Position and Effect enums representing character states
//! - Trading mechanics between positions and effects
//! - Validation and error handling for invalid trades
//! - Helper methods for manipulating positions and effects

pub mod effect;
pub mod position;

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use enum_map::{Enum, EnumMap};
use thiserror::Error;

pub use crate::action::{effect::Effect, position::Position};
use crate::data::value::{Error as ValueError, UnsignedInteger, Value};

const ACTION_MAX: usize = 4;

#[derive(Error, Debug, PartialEq)]
pub enum ActionError {
    #[error(transparent)]
    ValueError(#[from] ValueError),
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

pub trait Actions: Debug + Default + PartialEq {
    fn get(&self, action: Action) -> u8;
    fn set(&mut self, action: Action, value: u8) -> Result<u8>;
    fn increment(&mut self, action: Action, increment: u8) -> Result<u8>;
}

pub type ActionValue = UnsignedInteger<u8, 0, ACTION_MAX>;

type Result<T> = std::result::Result<T, ActionError>;

#[derive(Debug, Default, PartialEq)]
pub struct ActionsMap(EnumMap<Action, ActionValue>);

impl Actions for ActionsMap {
    fn get(&self, action: Action) -> u8 {
        self[action].get()
    }

    fn set(&mut self, action: Action, value: u8) -> Result<u8> {
        self[action].set(value).map_err(ActionError::from)
    }

    fn increment(&mut self, action: Action, increment: u8) -> Result<u8> {
        self[action].increment(increment).map_err(ActionError::from)
    }
}

impl Deref for ActionsMap {
    type Target = EnumMap<Action, ActionValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ActionsMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::data::value::Error as ValueError;

    const ALL_ACTIONS: &[Action] = &[
        Action::Hunt,
        Action::Study,
        Action::Survey,
        Action::Tinker,
        Action::Finesse,
        Action::Prowl,
        Action::Skirmish,
        Action::Wreck,
        Action::Attune,
        Action::Command,
        Action::Consort,
        Action::Sway,
    ];

    proptest!(
        #[test]
        fn test_set_and_get_action_between_0_and_4(
            action in prop::sample::select(ALL_ACTIONS),
            value in 0u8..=4u8
        ) {
            let mut actions = ActionsMap::default();

            actions.set(action, value).expect("should have set action rating");

            assert_eq!(value, actions.get(action));
        }

        #[test]
        fn test_action_ratings_above_max_are_clamped_to_max(
            action in prop::sample::select(ALL_ACTIONS),
            value in 5u8..u8::MAX
        ) {
            let mut actions = ActionsMap::default();

            match actions.set(action, value).expect_err("should have clamped") {
                ActionError::ValueError(ValueError::ClampedMax) => assert!(value > 4, "Action rating clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}"),
            }

            assert_eq!(4, actions.get(action), "Action rating should clamp precisely to MAX (4)");
        }

        #[test]
        fn test_increment_action_rating_clamps_to_max(
            action in prop::sample::select(ALL_ACTIONS),
            increment in 5u8..=u8::MAX
        ) {
            let mut actions = ActionsMap::default();

            match actions.increment(action, increment).expect_err("should have clamped") {
                ActionError::ValueError(ValueError::ClampedMax) => assert!(increment > 4, "Action rating clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}"),
            }

            assert_eq!(4, actions.get(action), "Action rating should clamp precisely to MAX (4)");
        }
    );
}
