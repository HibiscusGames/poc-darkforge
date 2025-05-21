//! Implements character mechanics including action ratings, harm, and trauma.
//!
//! Characters in the game have:
//! - Attributes (Insight, Prowess, Resolve)
//! - Action ratings (ranging from 0-4)
//! - Harm tracking
//! - Trauma tracking

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Hunt,
    Study,
    Survey,
    Tinker,
    Finesse,
    Prowl,
    Skirmish,
    Wreck,
    Attune,
    Command,
    Consort,
    Sway,
}

#[derive(Debug, PartialEq)]
pub struct Character {
    name: String,
    actions: Actions,
}

#[derive(Debug, PartialEq)]
pub struct Actions {
    // Insight
    /// When you Hunt, you carefully track a target
    hunt: u8,
    /// When you `study`, you scrutinize details and interpret evidence.
    study: u8,
    /// When you `survey`, you observe the situation and anticipate outcomes.
    survey: u8,
    /// When you `tinker`, you fiddle with devices and mechanisms.
    tinker: u8,
    // Prowess
    /// When you `finesse`, you employ dextrous manipulation or subtle misdirection.
    finesse: u8,
    /// When you `prowl`, you traverse skilfully and quietly.
    prowl: u8,
    /// When you `skirmish`, you entangle a target in close combat so they can’t easily escape.
    skirmish: u8,
    /// When you `wreck`, you unleash savage force.
    wreck: u8,
    // Resolve
    /// When you `attune`, you open your mind to arcane power.
    attune: u8,
    /// When you `command`, you compel swift obedience.
    command: u8,
    /// When you `consort`, you socialize with friends and contacts.
    consort: u8,
    /// When you `sway`, you influence with guile, charm or argument.
    sway: u8,
}

impl Character {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            actions: Actions {
                hunt: 0,
                study: 0,
                survey: 0,
                tinker: 0,
                finesse: 0,
                prowl: 0,
                skirmish: 0,
                wreck: 0,
                attune: 0,
                command: 0,
                consort: 0,
                sway: 0,
            },
        }
    }

    /// Gets the rating for the specified action.
    pub fn get_action_rating(&self, action: Action) -> u8 {
        match action {
            Action::Hunt => self.actions.hunt,
            Action::Study => self.actions.study,
            Action::Survey => self.actions.survey,
            Action::Tinker => self.actions.tinker,
            Action::Finesse => self.actions.finesse,
            Action::Prowl => self.actions.prowl,
            Action::Skirmish => self.actions.skirmish,
            Action::Wreck => self.actions.wreck,
            Action::Attune => self.actions.attune,
            Action::Command => self.actions.command,
            Action::Consort => self.actions.consort,
            Action::Sway => self.actions.sway,
        }
    }

    /// Sets the rating for the specified action.
    ///
    /// The rating is clamped to the valid range of 0-4.
    pub fn set_action_rating(&mut self, action: Action, rating: u8) {
        match action {
            Action::Hunt => self.actions.hunt = rating,
            Action::Study => self.actions.study = rating,
            Action::Survey => self.actions.survey = rating,
            Action::Tinker => self.actions.tinker = rating,
            Action::Finesse => self.actions.finesse = rating,
            Action::Prowl => self.actions.prowl = rating,
            Action::Skirmish => self.actions.skirmish = rating,
            Action::Wreck => self.actions.wreck = rating,
            Action::Attune => self.actions.attune = rating,
            Action::Command => self.actions.command = rating,
            Action::Consort => self.actions.consort = rating,
            Action::Sway => self.actions.sway = rating,
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn test_new_character_has_default_action_ratings_of_0() {
        let character = Character::new("Test Character");

        assert_eq!(
            Character {
                name: "Test Character".to_string(),
                actions: Actions {
                    hunt: 0,
                    study: 0,
                    survey: 0,
                    tinker: 0,
                    finesse: 0,
                    prowl: 0,
                    skirmish: 0,
                    wreck: 0,
                    attune: 0,
                    command: 0,
                    consort: 0,
                    sway: 0,
                },
            },
            character
        );
    }

    proptest! {
        #[test]
        fn test_set_and_get_action_between_0_and_4(
            action in prop::sample::select(vec![Action::Hunt, Action::Study, Action::Survey, Action::Tinker, Action::Finesse, Action::Prowl, Action::Skirmish, Action::Wreck, Action::Attune, Action::Command, Action::Consort, Action::Sway]),
            value in 0u8..=4u8
        ) {
            let mut character = Character::new("Test Character");

            character.set_action_rating(action, value);

            assert_eq!(value, character.get_action_rating(action));
        }
    }
}
