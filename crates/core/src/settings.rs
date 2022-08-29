#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Stores RRR settings to start charts with.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {}

impl Default for Settings {
    #[must_use]
    fn default() -> Self {
        Self {}
    }
}

impl Settings {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings = Settings::default();
    }
}
