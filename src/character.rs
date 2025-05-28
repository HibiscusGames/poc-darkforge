//! Implements a character sheet, including actions, harm, trauma, stress, etc.
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Deref, DerefMut, Range},
};

use thiserror::Error;

use crate::{
    action::Actions,
    data::{ArrayTracker, Error as DataError, Tracker, UnsignedInteger, Value},
};

const STRESS_MAX: usize = 10;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error(transparent)]
    DataError(#[from] DataError),
    #[error(transparent)]
    HarmTrackerError(#[from] HarmTrackerError),
}

#[derive(Debug, Error, PartialEq)]
pub enum HarmTrackerError {
    #[error("Cannot heal a dead character.")]
    HealErrorDead,
    #[error("Cannot heal, character is not wounded.")]
    HealErrorHealthy,
    #[error("Cannot harm a character that is already dead.")]
    HarmErrorDead,
    #[error(transparent)]
    TrackerError(#[from] TrackerError<Harm>),
}

/// Represents physical injuries a character can sustain during play.
///
/// Harm is tracked at different severity levels, and too much harm can put a character out of action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarmLevel {
    /// Minor harm, ex: Battered, Drained, Distracted, Scared, Confused.
    Lesser,
    /// Moderate harm, ex: Exhausted, Deep Cut to Arm, Concussion, Panicked, Seduced.
    Moderate,
    /// Severe harm, ex: Impaled, Broken Leg, Shot in Chest, Badly Burned, Terrified.
    Severe,
    /// Fatal harm, ex: Electrocuted, Drowned, Stabbed in the Heart.
    Fatal,
}

/// A specific instance of harm, including the description and severity level.
///
/// Harm is stored as a string to allow for custom descriptions and to avoid
/// having to define a separate enum for each possible harm level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarmType {
    /// Fatigue represents exhaustion, mental strain or energy depletion. Typically caused by pushing yourself too hard.
    ///
    /// Examples
    /// Lesser: drained, bleary, dizzy
    /// Moderate: exhausted, sluggish
    /// Severe: nearing collapse
    /// Fatal: comatose, organ failure
    Fatigue,
    /// Hunger represents the unmet need for food.
    ///
    /// Examples
    /// Lesser: hungry
    /// Moderate: ravenous, weak
    /// Severe: starving, emaciated
    /// Fatal: starved to death
    Hunger,
    /// Thirst represents the unmet need for water.
    ///
    /// Examples
    /// Lesser: thirsty
    /// Moderate: parched, dizzy
    /// Severe: dehydrated
    /// Fatal: dessicated
    Thirst,
    /// Piercing harm represents damage that penetrates the skin.
    ///
    /// Examples
    /// Lesser: pricked, grazed
    /// Moderate: stabbed, shot in the arm, shot in the shoulder
    /// Severe: impaled, shot in the chest, shot in the belly
    /// Fatal: impaled through the chest, shot in the head
    Piercing,
    /// Slashing harm represents damage that cuts the skin.
    ///
    /// Examples
    /// Lesser: cut, scraped
    /// Moderate: slashed, cut in the arm, cut in the shoulder
    /// Severe: cut in the chest, cut in the belly, gash to the face
    /// Fatal: decapitated, slit throat
    Slashing,
    /// Blunt harm represents damage that crushes or squeezes the body.
    ///
    /// Examples
    /// Lesser: bruised, swollen
    /// Moderate: bruised ribs, swollen joint, broken arm
    /// Severe: shattered arm, broken shoulder, broken ribs
    /// Fatal: crushed chest, shattered skull
    Blunt,
    /// Psychic harm represents damage to the mind. Can be caused by trauma over time, or sometimes suddenly by witnessing so incomprehensible that the mind cracks under the strain.
    ///
    /// Examples:
    /// Lesser: anxious, unsettled by intrusive thoughts
    /// Moderate: panicky, mild hallucination, mild compulsion
    /// Severe: paranoia, hallucination, delusion, minor stroke
    /// Fatal: catatonic, psychotic break, fatal stroke
    Psychic,
    /// Fear represents a strong emotional response to danger or threat.
    ///
    /// Examples:
    /// Lesser: startled, shaken
    /// Moderate: terrified, panicked
    /// Severe: hysterical, paranoid
    /// Fatal: catatonic, aneurysm, fatal stroke, heart attack
    Fear,
    /// Confusion represents a loss of clarity or coherence.
    ///
    /// Examples:
    /// Lesser: dizzy, distracted
    /// Moderate: confused, disoriented
    /// Severe: delirious, forgetful
    /// Fatal: demented, amnesiac
    Confusion,
    /// Charm represents a strong emotional response to attraction or pleasure.
    ///
    /// Examples:
    /// Lesser: distracted, swayed
    /// Moderate: fixated, infatuated
    /// Severe: devoted, addicted
    /// Fatal: severely addicted, fanatical
    Charm,
    /// Acid represents damage that corrodes the body.
    ///
    /// Examples:
    /// Lesser: irritated skin, stinging eyes
    /// Moderate: burned, blistered
    /// Severe: severely acid burned
    /// Fatal: fatal acid burns
    Acid,
    /// Cold represents damage that freezes the body.
    ///
    /// Examples:
    /// Lesser: chills, shivers
    /// Moderate: frozen, numb
    /// Severe: frostbite, hypothermia
    /// Fatal: frozen to death
    Cold,
    /// Fire represents damage that burns the body.
    ///
    /// Examples:
    /// Lesser: scalded, chafed
    /// Moderate: burned, blistered
    /// Severe: severely burned, charred
    /// Fatal: burned to death
    Fire,
    /// Electric represents damage that shocks the body.
    ///
    /// Examples:
    /// Lesser: shocked, jolted
    /// Moderate: burned
    /// Severe: severely burned, heart attack
    /// Fatal: electrocuted
    Electric,
    /// Poison represents damage that poisons the body.
    ///
    /// Examples:
    /// Lesser: nauseated, dizzy, tight chest
    /// Moderate: poisoned, cramps, vomiting
    /// Severe: internal bleeding, nearing collapse, paralysis
    /// Fatal: organ failure, comatose
    Poison,
    /// Disease represents an illness that affects the body.
    ///
    /// Examples:
    /// Lesser: shivers, headaches, mild fever
    /// Moderate: feverish, chills, weakness, vomiting
    /// Severe: delirious, severe weakness, collapse
    /// Fatal: organ failure, fatal illness, permanently disabled
    Disease,
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
pub struct Character<ACT: Actions, T: Traumas> {
    /// The name of the character.
    name: String,
    /// The skill ratings for the actions the character can perform.
    actions: ACT,
    /// The stress tracker for the character.
    stress: StressLevel,
    /// The trauma tracker for the character.
    traumas: T,
    /// The harm tracker for the character.
    harm: HarmTracker,
}

/// A specific instance of harm, including the level and type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Harm(HarmLevel, HarmType);

/// Default implementation of a character using the recommended dependencies.
pub type DefaultCharacter = Character<ActionsMap, SetTracker<Trauma, 4>>;

impl Display for Harm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.0, self.1)
    }
}

/// A specialized tracker for character harm that follows the game's rules for harm slots.
///
/// The tracker has 6 slots with specific allocations:
/// - Slots 0-1: Lesser harm
/// - Slots 2-3: Moderate harm
/// - Slot 4: Severe harm
/// - Slot 5: Fatal harm
#[derive(Debug, Default, PartialEq)]
pub struct HarmTracker(ArrayTracker<Harm, 6>);

impl HarmLevel {
    pub fn range(&self) -> Range<usize> {
        match self {
            HarmLevel::Lesser => 0..2,
            HarmLevel::Moderate => 2..4,
            HarmLevel::Severe => 4..5,
            HarmLevel::Fatal => 5..6,
        }
    }

    fn up(&self) -> Self {
        match self {
            HarmLevel::Lesser => HarmLevel::Moderate,
            HarmLevel::Moderate => HarmLevel::Severe,
            HarmLevel::Severe => HarmLevel::Fatal,
            HarmLevel::Fatal => HarmLevel::Fatal,
        }
    }

    fn down(&self) -> Option<Self> {
        match self {
            HarmLevel::Lesser => None,
            HarmLevel::Moderate => Some(HarmLevel::Lesser),
            HarmLevel::Severe => Some(HarmLevel::Moderate),
            HarmLevel::Fatal => Some(HarmLevel::Severe),
        }
    }
}

impl HarmTracker {
    /// Applies a harm to the character, following the slot allocation rules.
    ///
    /// If the slots for the given harm level are full, the harm is upgraded to the next level.
    /// Returns the actual harm that was applied, which may be different from the input harm
    /// if an upgrade occurred.
    pub fn apply(&mut self, harm: Harm) -> Result<Harm, HarmTrackerError> {
        let Harm(level, kind) = harm;
        let level_count = self.0.list().into_iter().filter(|h| h.0 == level).count();
        let level_capacity = match level {
            HarmLevel::Lesser => 2,
            HarmLevel::Moderate => 2,
            HarmLevel::Severe => 1,
            HarmLevel::Fatal => 1,
        };

        if level_count >= level_capacity && level == HarmLevel::Fatal {
            Err(HarmTrackerError::HarmErrorDead)
        } else if level_count >= level_capacity {
            self.apply(Harm(level.up(), kind))
        } else {
            self.0.append(harm).map_err(HarmTrackerError::TrackerError)?;
            Ok(harm)
        }
    }

    /// Removes all harm by downgrading each harm by one level.
    ///
    /// Lesser harm is completely removed, while higher level harm is downgraded.
    /// Returns the number of harm items that were downgraded or removed.
    pub fn heal(&mut self) -> Result<(), HarmTrackerError> {
        if self.0.is_empty() {
            return Err(HarmTrackerError::HealErrorHealthy);
        }
        if self.is_dead() {
            return Err(HarmTrackerError::HealErrorDead);
        }

        let mut new_tracker = ArrayTracker::<Harm, 6>::default();
        for &harm in self.list() {
            let Harm(level, kind) = harm;

            if let Some(downgraded_level) = level.down() {
                new_tracker.append(Harm(downgraded_level, kind))?;
            }
        }

        self.0 = new_tracker;

        Ok(())
    }

    pub fn is_dead(&self) -> bool {
        self.list().last().is_some_and(|harm| harm.0 == HarmLevel::Fatal)
    }
}

impl<ACT: Actions, T: Traumas> Character<ACT, T> {
    pub fn new(name: &str) -> Self {
        Character {
            name: name.to_string(),
            actions: ACT::default(),
            stress: StressLevel::default(),
            traumas: T::default(),
            harm: HarmTracker::default(),
        }
    }

    /// Returns a reference to the action ratings for the character
    pub fn actions(&self) -> &ACT {
        &self.actions
    }

    /// Returns a mutable reference to the action ratings for the character.
    pub fn actions_mut(&mut self) -> &mut ACT {
        &mut self.actions
    }

    /// Returns a reference to the stress tracker for the character.
    pub fn stress(&self) -> &StressLevel {
        &self.stress
    }

    /// Returns a mutable reference to the stress tracker for the character.
    pub fn stress_mut(&mut self) -> &mut StressLevel {
        &mut self.stress
    }

    /// Returns a reference to the trauma tracker for the character.
    pub fn traumas(&self) -> &T {
        &self.traumas
    }

    /// Returns a reference to the character's harm tracker.
    pub fn harm(&self) -> &HarmTracker {
        &self.harm
    }

    #[cfg(test)]
    fn harm_mut(&mut self) -> &mut HarmTracker {
        &mut self.harm
    }
}

impl Deref for HarmTracker {
    type Target = ArrayTracker<Harm, 6>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HarmTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use rand::prelude::*;
    use rstest::rstest;

    use super::*;
    use crate::{action::ActionsMap, data::tracker::Tracker};

    const LEVELS: &[HarmLevel] = &[HarmLevel::Lesser, HarmLevel::Moderate, HarmLevel::Severe];

    const KINDS: &[HarmType] = &[
        HarmType::Fatigue,
        HarmType::Hunger,
        HarmType::Thirst,
        HarmType::Piercing,
        HarmType::Slashing,
        HarmType::Blunt,
        HarmType::Psychic,
        HarmType::Fear,
        HarmType::Confusion,
        HarmType::Charm,
        HarmType::Acid,
        HarmType::Cold,
        HarmType::Fire,
        HarmType::Electric,
        HarmType::Poison,
        HarmType::Disease,
    ];

    proptest! {
        #[test]
        fn test_harm_is_added_to_empty_tracker(level in prop::sample::select(LEVELS), kind in prop::sample::select(KINDS)) {
            let mut character: Character<ActionsMap, SetTracker<Trauma, 4>> = Character::new("Test Character");
            let got = character.harm_mut().apply(Harm(level, kind)).expect("should have added harm");

            let expected = Harm(level, kind);

            assert_eq!(expected, got);
            assert_eq!(vec![expected], character.harm().list().into_iter().cloned().collect::<Vec<_>>());
        }

        #[test]
        fn test_harm_is_upgraded_when_tracker_is_full_for_that_level(level in prop::sample::select(LEVELS), kind in prop::sample::select(KINDS)) {
            let mut character: Character<ActionsMap, SetTracker<Trauma, 4>> = Character::new("Test Character");
            let mut expected_harm = vec![];
            for _ in level.range() {
                let h = Harm(level, KINDS.choose(&mut rand::rng()).cloned().expect("should have selected a random harm type"));
                expected_harm.push(h);
                character.harm_mut().apply(h).expect("should have applied initial harm");
            }

            let got = character.harm_mut().apply(Harm(level, kind)).expect("should have added harm");

            let expected = Harm(level.up(), kind);
            expected_harm.push(expected);
            let got_harm: Vec<Harm> = character.harm().list().into_iter().cloned().collect();

            assert_eq!(expected, got);
            assert_eq!(expected_harm, got_harm);
        }
    }

    #[rstest]
    #[case::tracker_full(
        vec![Harm(HarmLevel::Severe, HarmType::Blunt), Harm(HarmLevel::Moderate, HarmType::Piercing), Harm(HarmLevel::Moderate, HarmType::Slashing), Harm(HarmLevel::Lesser, HarmType::Fatigue), Harm(HarmLevel::Lesser, HarmType::Hunger), Harm(HarmLevel::Fatal, HarmType::Blunt)],
        HarmTrackerError::HarmErrorDead
    )]
    #[case::tracker_has_fatal_harm(
        vec![Harm(HarmLevel::Fatal, HarmType::Blunt)],
        HarmTrackerError::HarmErrorDead
    )]
    fn test_apply_harm_fails(#[case] initial_harms: Vec<Harm>, #[case] expect: HarmTrackerError) {
        let mut character: Character<ActionsMap, SetTracker<Trauma, 4>> = Character::new("Test Character");
        for harm in &initial_harms {
            character.harm_mut().apply(*harm).expect("should have added harm");
        }

        let got = character
            .harm_mut()
            .apply(Harm(HarmLevel::Fatal, HarmType::Blunt))
            .expect_err("should have failed to add harm");

        assert_eq!(got, expect);
    }

    #[rstest]
    #[case::one_of_each_except_fatal(
        vec![Harm(HarmLevel::Severe, HarmType::Blunt), Harm(HarmLevel::Moderate, HarmType::Piercing), Harm(HarmLevel::Lesser, HarmType::Fatigue)],
        vec![Harm(HarmLevel::Moderate, HarmType::Blunt), Harm(HarmLevel::Lesser, HarmType::Piercing)]
    )]
    #[case::saturated_tracker_except_fatal(
        vec![Harm(HarmLevel::Severe, HarmType::Blunt), Harm(HarmLevel::Moderate, HarmType::Piercing), Harm(HarmLevel::Moderate, HarmType::Slashing), Harm(HarmLevel::Lesser, HarmType::Fatigue), Harm(HarmLevel::Lesser, HarmType::Hunger)],
        vec![Harm(HarmLevel::Moderate, HarmType::Blunt), Harm(HarmLevel::Lesser, HarmType::Piercing), Harm(HarmLevel::Lesser, HarmType::Slashing)]
    )]
    #[case::severe_harm_only(
        vec![Harm(HarmLevel::Severe, HarmType::Blunt)],
        vec![Harm(HarmLevel::Moderate, HarmType::Blunt)])]
    #[case::moderate_harm_only(
        vec![Harm(HarmLevel::Moderate, HarmType::Piercing), Harm(HarmLevel::Moderate, HarmType::Slashing)],
        vec![Harm(HarmLevel::Lesser, HarmType::Piercing), Harm(HarmLevel::Lesser, HarmType::Slashing)]
    )]
    #[case::lesser_harm_only(
        vec![Harm(HarmLevel::Lesser, HarmType::Fatigue), Harm(HarmLevel::Lesser, HarmType::Hunger)],
        vec![]
    )]
    fn test_heal_downgrades_all_harm_and_removes_lesser_harm(#[case] initial_harms: Vec<Harm>, #[case] expected_harms: Vec<Harm>) {
        let mut character: Character<ActionsMap, SetTracker<Trauma, 4>> = Character::new("Test Character");
        for harm in &initial_harms {
            character.harm_mut().apply(*harm).expect("should have added harm");
        }
        assert_eq!(initial_harms.len(), character.harm().count());

        character.harm_mut().heal().expect("should have healed harm");
        assert_eq!(expected_harms.len(), character.harm().count());

        let got_harm: Vec<Harm> = character.harm().list().into_iter().cloned().collect();

        for expected_harm in &expected_harms {
            assert!(got_harm.contains(expected_harm), "Expected harm {expected_harm:?} not found");
        }

        assert_eq!(got_harm.len(), expected_harms.len(), "Got unexpected number of harms after removal");
    }

    #[rstest]
    #[case::empty_tracker(vec![], HarmTrackerError::HealErrorHealthy)]
    #[case::fatally_harmed(vec![Harm(HarmLevel::Fatal, HarmType::Poison)], HarmTrackerError::HealErrorDead)]
    #[case::full_tracker(
        vec![Harm(HarmLevel::Lesser, HarmType::Blunt), Harm(HarmLevel::Lesser, HarmType::Piercing), Harm(HarmLevel::Moderate, HarmType::Slashing), Harm(HarmLevel::Moderate, HarmType::Fatigue), Harm(HarmLevel::Severe, HarmType::Hunger), Harm(HarmLevel::Fatal, HarmType::Blunt)],
        HarmTrackerError::HealErrorDead
    )]
    fn test_heal_fails(#[case] init_state: Vec<Harm>, #[case] expect: HarmTrackerError) {
        let mut character: Character<ActionsMap, SetTracker<Trauma, 4>> = Character::new("Test Character");
        for harm in &init_state {
            character.harm_mut().apply(*harm).expect("should have added harm");
        }

        let got = character.harm_mut().heal().expect_err("should have failed to heal");

        assert_eq!(got, expect);
    }
}
