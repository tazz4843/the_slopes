use self::super::{Difficulty, Mood};
use raylib::math::Vector4;

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
    /// Creates a new guest at 0,0,0,0 and with default mood/difficulty
    pub fn new() -> Self {
        Self {
            position: Vector4::new(0_f32, 0_f32, 0_f32, 0_f32),
            mood: Mood::Neutral,
            difficulty: Difficulty::Medium,
        }
    }

    /// Ticks the guest once.
    pub fn tick(&mut self) {}
}

impl Default for Guest {
    fn default() -> Self {
        Guest::new()
    }
}
