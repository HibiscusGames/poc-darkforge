use std::fmt::Display;

use crate::data::tracker::{Error as TrackerError, Tracker};

/// A trauma is a persistent emotional or psychological condition that affects a character's behaviour and outlook.
/// It is gained when maxing out the stress meter during a heist.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    /// The character has no traumas.
    Fresh,
    /// The character has traumas, but is not broken.
    Scarred,
    /// The character is broken.
    Broken,
}

impl Display for Trauma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Trauma {
    /// Creates a new traumas tracker from a tracker implementation.
    pub fn tracker<const N: usize>(tracker: impl Tracker<Trauma>) -> impl Traumas {
        tracker
    }
}

/// Represents a collection of traumas.
pub trait Traumas: Default {
    /// Adds a new trauma to the character.
    /// If the character is already broken, the call will fail.
    ///
    /// # Returns
    ///
    /// * `Ok(State::Scarred)` - The trauma was successfully added, but the character is not yet broken.
    /// * `Ok(State::Broken)` - The trauma was successfully added, and the character is now broken.
    /// * `Err(TrackerError::TooManyItems)` - The character is already broken.
    fn scar(&mut self, trauma: Trauma) -> Result<State, TrackerError<Trauma>>;

    /// Returns the mental state of the character.
    ///
    /// # Returns
    ///
    /// * `State::Fresh` - The character has no traumas.
    /// * `State::Scarred` - The character has traumas, but is not broken.
    /// * `State::Broken` - The character is broken.
    fn state(&self) -> State;

    /// Returns true if the character has the specified trauma.
    fn has_trauma(&self, trauma: Trauma) -> bool;

    /// Returns the number of traumas the character has.
    fn count(&self) -> usize;
}

impl<T: Tracker<Trauma>> Traumas for T {
    fn scar(&mut self, trauma: Trauma) -> Result<State, TrackerError<Trauma>> {
        self.append(trauma)?;

        Ok(self.state())
    }

    fn state(&self) -> State {
        if self.is_empty() {
            State::Fresh
        } else if self.is_full() {
            State::Broken
        } else {
            State::Scarred
        }
    }

    fn has_trauma(&self, trauma: Trauma) -> bool {
        self.list().iter().any(|t| trauma.eq(t))
    }

    fn count(&self) -> usize {
        self.count()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use prop::sample;
    use proptest::{collection, prelude::*};

    use super::*;
    use crate::data::tracker::SetTracker;

    const TRAUMAS: [Trauma; 8] = [
        Trauma::Cold,
        Trauma::Haunted,
        Trauma::Obsessed,
        Trauma::Paranoid,
        Trauma::Reckless,
        Trauma::Soft,
        Trauma::Unstable,
        Trauma::Vicious,
    ];

    fn trauma_strategy() -> impl Strategy<Value = Trauma> {
        prop_oneof![
            Just(TRAUMAS[0]),
            Just(TRAUMAS[1]),
            Just(TRAUMAS[2]),
            Just(TRAUMAS[3]),
            Just(TRAUMAS[4]),
            Just(TRAUMAS[5]),
            Just(TRAUMAS[6]),
            Just(TRAUMAS[7])
        ]
    }

    fn unique_traumas_vec(exact_size: usize) -> impl Strategy<Value = Vec<Trauma>> {
        collection::vec(trauma_strategy(), 1..=8)
            .prop_map(move |traumas| {
                let mut unique = HashSet::new();
                let filtered: Vec<Trauma> = traumas.into_iter().filter(|t| unique.insert(*t)).collect();

                filtered.into_iter().take(exact_size).collect()
            })
            .prop_filter(format!("vector size must be {}", exact_size), move |v: &Vec<Trauma>| {
                v.len() == exact_size
            })
    }

    fn unique_traumas_vec_range(min_size: usize, max_size: usize) -> impl Strategy<Value = Vec<Trauma>> {
        (min_size..=max_size).prop_flat_map(|size| unique_traumas_vec(size))
    }

    #[test]
    fn test_traumas_initialise_default() {
        let traumas = Trauma::tracker::<4>(SetTracker::<Trauma, 4>::default());

        assert_eq!(traumas.count(), 0, "New traumas tracker should be empty");
        assert!(traumas.state() == State::Fresh, "New character should not be broken");
    }

    proptest! {
        #[test]
        fn prop_four_traumas_means_broken(traumas in unique_traumas_vec(4)) {
            let tracker = Trauma::tracker::<4>(SetTracker::<Trauma, 4>::new(&traumas).unwrap());
            prop_assert_eq!(tracker.state(), State::Broken);
        }

        #[test]
        fn prop_fewer_than_four_traumas_not_broken(traumas in unique_traumas_vec_range(1, 3)) {
            let tracker = Trauma::tracker::<4>(SetTracker::<Trauma, 4>::new(&traumas).unwrap());
            prop_assert_eq!(tracker.state(), State::Scarred);
        }

        #[test]
        fn prop_cannot_exceed_capacity(traumas_base in unique_traumas_vec(5)) {
            let mut tracker = Trauma::tracker::<4>(SetTracker::<Trauma, 4>::new(&traumas_base[..4]).unwrap());

            let extra_trauma = traumas_base[4];
            let got = tracker.scar(extra_trauma).expect_err("should have failed");

            match got {
                TrackerError::TooManyItems(capacity, _) => {
                    prop_assert_eq!(capacity, 4);
                },
                _ => prop_assert!(false, "Unexpected result: {:?}", got),
            }
        }

        #[test]
        fn prop_cannot_add_duplicates(traumas in unique_traumas_vec_range(1, 3)) {
            let mut tracker = Trauma::tracker::<4>(SetTracker::<Trauma, 4>::new(&traumas).unwrap());
            let duplicate = traumas[0];

            let got = tracker.scar(duplicate).expect_err("should have failed");

            if let TrackerError::Duplicate(trauma) = got {
                prop_assert_eq!(trauma, duplicate);
            }
        }

        #[test]
        fn test_returns_true_when_has_trauma(trauma in sample::select(&TRAUMAS)){
            let traumas = Trauma::tracker::<4>(SetTracker::<Trauma, 4>::new(&[trauma]).unwrap());

            prop_assert!(traumas.has_trauma(trauma), "Character should have trauma");
        }

        #[test]
        fn test_returns_false_when_does_not_have_trauma(traumas in unique_traumas_vec(2)){
            let tracker = Trauma::tracker::<4>(SetTracker::<Trauma, 4>::new(&traumas[..1]).unwrap());

            prop_assert!(!tracker.has_trauma(traumas[1]), "Character should not have {:?} trauma", traumas[1]);
        }

        #[test]
        fn prop_count_matches_unique_traumas(traumas in unique_traumas_vec_range(0, 4)) {
            let tracker = Trauma::tracker::<4>(SetTracker::<Trauma, 4>::new(&traumas).unwrap());
            prop_assert_eq!(tracker.count(), traumas.len());
        }
    }
}
