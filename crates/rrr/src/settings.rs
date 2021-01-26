#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Stores RRR settings to play charts with.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Settings {}

/// A builder for a [Settings](Settings) instance.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SettingsBuilder {}

impl SettingsBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn build(&self) -> Settings {
        Settings {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_builder() {
        let settings_builder = SettingsBuilder::new();
        let _settings = settings_builder.build();
    }
}
