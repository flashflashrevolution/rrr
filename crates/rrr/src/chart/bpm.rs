#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
/// Represents a BPM change in a [chart](Chart).
pub struct BpmChange {
    bpm: f32,
    beat: usize,
}

impl BpmChange {
    #[must_use]
    pub fn new(bpm: f32, beat: usize) -> Self {
        Self { bpm, beat }
    }

    /// Get the bpm change's bpm.
    #[must_use]
    pub fn bpm(&self) -> f32 {
        self.bpm
    }
}
