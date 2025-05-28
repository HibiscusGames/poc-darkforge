//! Implements a character sheet, including actions, harm, trauma, stress, etc.
use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut, Range},
};

use thiserror::Error;

use crate::{
    action::{Action, ActionValue, Actions},
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
pub struct Character<ACT: Actions> {
    /// The name of the character.
    name: String,
    /// The skill ratings for the actions the character can perform.
    actions: ACT,
    /// The stress tracker for the character.
    stress: Stress,
    /// The trauma tracker for the character.
    traumas: Traumas,
    /// The harm tracker for the character.
    harm: HarmTracker,
}

type Stress = UnsignedInteger<u8, 0, STRESS_MAX>;
type Traumas = ArrayTracker<Trauma, 4>;

/// A specific instance of harm, including the level and type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Harm(HarmLevel, HarmType);

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
    pub fn apply(&mut self, harm: Harm) -> Result<Harm, Error> {
        let Harm(level, kind) = harm;
        let level_count = self.list().into_iter().filter(|h| h.0 == level).count();
        let level_capacity = match level {
            HarmLevel::Lesser => 2,
            HarmLevel::Moderate => 2,
            HarmLevel::Severe => 1,
            HarmLevel::Fatal => 1,
        };

        if level_count >= level_capacity && level == HarmLevel::Fatal {
            Err(Error::HarmTrackerError(HarmTrackerError::HarmErrorDead))
        } else if level_count >= level_capacity {
            self.apply(Harm(level.up(), kind))
        } else {
            self.append(harm)?;
            Ok(harm)
        }
    }

    /// Removes all harm by downgrading each harm by one level.
    ///
    /// Lesser harm is completely removed, while higher level harm is downgraded.
    /// Returns the number of harm items that were downgraded or removed.
    pub fn heal(&mut self) -> Result<(), Error> {
        if self.is_empty() {
            return Err(Error::HarmTrackerError(HarmTrackerError::HealErrorHealthy));
        }
        if self.is_dead() {
            return Err(Error::HarmTrackerError(HarmTrackerError::HealErrorDead));
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

impl<ACT: Actions> Character<ACT> {
    pub fn new(name: &str) -> Self {
        Character {
            name: name.to_string(),
            actions: ACT::default(),
            stress: Stress::default(),
            traumas: Traumas::default(),
            harm: HarmTracker::default(),
        }
    }

    /// Returns a reference to the skill rating for the given action.
    pub fn action(&self, action: Action) -> u8 {
        self.actions.get(action)
    }

    /// Returns a mutable reference to the skill rating for the given action.
    pub fn action_mut(&mut self, action: Action) -> &mut ActionValue {
        self.actions.get_mut(action)
    }

    /// Returns a reference to the stress tracker for the character.
    pub fn stress(&self) -> &Stress {
        &self.stress
    }

    /// Returns a mutable reference to the stress tracker for the character.
    pub fn stress_mut(&mut self) -> &mut Stress {
        &mut self.stress
    }

    /// Returns a reference to the trauma tracker for the character.
    pub fn traumas(&self) -> &Traumas {
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

    /// Returns true if the character has pending trauma (stress level at maximum)
    pub fn has_pending_trauma(&self) -> bool {
        self.stress.get() >= STRESS_MAX as u8
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
    use crate::{
        action::ActionsMap,
        data::{Tracker, value::Error as ValueError},
    };

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
        fn test_set_and_get_stress_level_between_0_and_10(
            stress in 0u8..=10u8
        ) {
            let mut character: Character<ActionsMap> = Character::new("Test Character");

            character.stress_mut().set(stress).expect("should have set stress level");

            assert_eq!(stress, character.stress().get());
        }

        #[test]
        fn test_setting_stress_levels_above_max_clamp_to_max(
            stress in 11u8..=u8::MAX
        ) {
            let mut character: Character<ActionsMap> = Character::new("Test Character");

            match character.stress_mut().set(stress).expect_err("should have clamped") {
                DataError::Value(ValueError::ClampedMax) => assert!(stress > 10, "Stress level clamped when it was lower than max"),
                e => panic!("unexpected error: {e:?}")
            }

            assert_eq!(STRESS_MAX as u8, character.stress().get(), "Stress level should clamp precisely to MAX ({STRESS_MAX})");
        }

        #[test]
        fn test_incrementing_stress_levels_above_max_clamp_to_max(
            stress in 0u8..=10u8
        ) {
            let mut character: Character<ActionsMap> = Character::new("Test Character");
            let increment = (STRESS_MAX + 1) as u8 - stress;

            character.stress_mut().set(stress).expect("should have set stress level");
            match character.stress_mut().increment(increment).expect_err("should have clamped") {
                DataError::Value(ValueError::ClampedMax) => assert!(stress + increment > STRESS_MAX as u8, "Stress level clamped when it was lower than max ({stress} + {increment} < {STRESS_MAX})"),
                e => panic!("unexpected error: {e:?}")
            }

            assert_eq!(STRESS_MAX as u8, character.stress().get(), "Stress level should clamp precisely to MAX ({STRESS_MAX})");
        }

        #[test]
        fn test_harm_is_added_to_empty_tracker(level in prop::sample::select(LEVELS), kind in prop::sample::select(KINDS)) {
            let mut character: Character<ActionsMap> = Character::new("Test Character");
            let got = character.harm_mut().apply(Harm(level, kind)).expect("should have added harm");

            let expected = Harm(level, kind);

            assert_eq!(expected, got);
            assert_eq!(vec![expected], character.harm().list().into_iter().cloned().collect::<Vec<_>>());
        }

        #[test]
        fn test_harm_is_upgraded_when_tracker_is_full_for_that_level(level in prop::sample::select(LEVELS), kind in prop::sample::select(KINDS)) {
            let mut character: Character<ActionsMap> = Character::new("Test Character");
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
        Error::HarmTrackerError(HarmTrackerError::HarmErrorDead)
    )]
    #[case::tracker_has_fatal_harm(
        vec![Harm(HarmLevel::Fatal, HarmType::Blunt)],
        Error::HarmTrackerError(HarmTrackerError::HarmErrorDead)
    )]
    fn test_apply_harm_fails(#[case] initial_harms: Vec<Harm>, #[case] expect: Error) {
        let mut character: Character<ActionsMap> = Character::new("Test Character");
        for harm in &initial_harms {
            character.harm_mut().apply(*harm).expect("should have added harm");
        }

        match character
            .harm_mut()
            .apply(Harm(HarmLevel::Fatal, HarmType::Blunt))
            .expect_err("should have failed to add harm")
        {
            e => assert_eq!(e, expect),
        }
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
        let mut character: Character<ActionsMap> = Character::new("Test Character");
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
        let mut character: Character<ActionsMap> = Character::new("Test Character");
        for harm in &init_state {
            character.harm_mut().apply(*harm).expect("should have added harm");
        }

        match character.harm_mut().heal().expect_err("should have failed to heal") {
            Error::HarmTrackerError(got) => assert_eq!(got, expect),
            e => panic!("unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_returns_false_when_checking_for_pending_trauma_given_a_character_with_a_stress_level_below_10() {
        let mut character: Character<ActionsMap> = Character::new("Test Character");
        character.stress_mut().set(9).expect("should have set stress level");

        assert!(!character.has_pending_trauma());
    }

    #[test]
    fn test_returns_true_when_checking_for_pending_trauma_given_a_character_with_a_stress_level_of_10() {
        let mut character: Character<ActionsMap> = Character::new("Test Character");
        character.stress_mut().set(10).expect("should have set stress level");

        assert!(character.has_pending_trauma());
    }

    #[test]
    fn test_new_character_has_empty_trauma_list() {
        let character: Character<ActionsMap> = Character::new("Test Character");

        assert!(character.traumas().is_empty());
    }

    #[test]
    fn test_new_character_has_empty_harm_tracker() {
        let character: Character<ActionsMap> = Character::new("Test Character");

        assert!(character.harm().is_empty());
    }
}
