use raylib::math::Vector4;
use self::super::{Mood, Difficulty};

/// A arbitrary guest.
pub struct Guest {
    /// Guest's position (xyz) and x rotation (w)
    position: Vector4,
    /// Guest's current mood.
    mood: Mood,
    /// Guest's preferred difficulty
    difficulty: Difficulty,
}

impl Guest {
    /// Ticks the guest once.
    fn tick() {

    }
}