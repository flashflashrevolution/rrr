#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
