//! Implements character mechanics including action ratings, harm, and trauma.
//!
//! Characters in the game have:
//! - Attributes (Insight, Prowess, Resolve)
//! - Action ratings (ranging from 0-4)
//! - Harm tracking
//! - Trauma tracking

#[derive(Debug, PartialEq)]
pub struct Character {
    name: String,
    actions: Actions,
}

#[derive(Debug, PartialEq)]
pub struct Actions {
    // Insight
    /// When you Hunt, you carefully track a target
    pub hunt: u8,
    /// When you `study`, you scrutinize details and interpret evidence.
    pub study: u8,
    /// When you `survey`, you observe the situation and anticipate outcomes.
    pub survey: u8,
    /// When you `tinker`, you fiddle with devices and mechanisms.
    pub tinker: u8,
    // Prowess
    /// When you `finesse`, you employ dextrous manipulation or subtle misdirection.
    pub finesse: u8,
    /// When you `prowl`, you traverse skilfully and quietly.
    pub prowl: u8,
    /// When you `skirmish`, you entangle a target in close combat so they can’t easily escape.
    pub skirmish: u8,
    /// When you `wreck`, you unleash savage force.
    pub wreck: u8,
    // Resolve
    /// When you `attune`, you open your mind to arcane power.
    pub attune: u8,
    /// When you `command`, you compel swift obedience.
    pub command: u8,
    /// When you `consort`, you socialize with friends and contacts.
    pub consort: u8,
    /// When you `sway`, you influence with guile, charm or argument.
    pub sway: u8,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_character_has_default_action_ratings() {
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
}
