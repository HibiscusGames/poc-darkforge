use std::fmt::Display;

use crate::data::tracker::ArrayTracker;

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

impl Display for Trauma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Traumas = ArrayTracker<Trauma, 4>;
