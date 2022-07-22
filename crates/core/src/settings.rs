#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Stores RRR settings to start charts with.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Settings {
    pub speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { speed: 1. }
    }
}

impl Settings {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings = Settings::new();
        assert!((settings.speed - 1.).abs() < f32::EPSILON);
    }
}
