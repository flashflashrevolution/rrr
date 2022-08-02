use std::fmt::{Display, Formatter};
use strum::{EnumCount, EnumIter};

use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Eq,
    EnumCount,
    EnumIter,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Serialize,
    Deserialize,
)]
pub enum Color {
    Red,
    Yellow,
    Blue,
    Orange,
    Green,
    Pink,
    Purple,
    Cyan,
    White,
    Receptor,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[repr(usize)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct CompiledNote {
    pub beat_position: i32,
    pub color: Color,
    pub direction: Direction,
    pub timestamp: i128,
}

impl Display for CompiledNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {}",
            self.color, self.direction, self.beat_position
        )
    }
}

impl Ord for CompiledNote {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.beat_position.cmp(&other.beat_position)
    }
}

impl PartialOrd for CompiledNote {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.beat_position.partial_cmp(&other.beat_position) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.color.partial_cmp(&other.color) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.direction.partial_cmp(&other.direction) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
/// Represents a single note in a [note row](NoteRow).
pub struct Note {
    lane: usize,
}

impl Note {
    #[must_use]
    pub fn new(lane: usize) -> Self {
        Self { lane }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
/// Stores the [notes](Note) that belong to a single row in a [beat](Beat).
pub struct NoteRow {
    offset: u32,
    // TODO: This should be [Note; N] but const generics aren't stable yet.
    notes: Vec<Note>,
}

impl NoteRow {
    #[must_use]
    pub fn new(offset: u32, notes: &[Note]) -> Self {
        Self {
            offset,
            notes: notes.into(),
        }
    }
}

// rust test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lane_value() {
        let lane_descriminant = Direction::Left as usize;
        assert_eq!(lane_descriminant, 0);

        let lane_descriminant = Direction::Down as usize;
        assert_eq!(lane_descriminant, 1);

        let lane_descriminant = Direction::Up as usize;
        assert_eq!(lane_descriminant, 2);

        let lane_descriminant = Direction::Right as usize;
        assert_eq!(lane_descriminant, 3);
    }
}
