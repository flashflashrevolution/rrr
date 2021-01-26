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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
/// Stores all of the [note rows](NoteRow) that represent a beat.
pub struct Beat {
    note_rows: Vec<NoteRow>,
    subdivisions: u32,
}

impl Beat {
    #[must_use]
    pub fn new(note_rows: &[NoteRow], subdivisions: u32) -> Self {
        Self {
            note_rows: note_rows.into(),
            subdivisions,
        }
    }
}

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
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
/// A runtime efficient representation of a chart used by an [RRR](crate::RRR) instance.
pub struct CompiledChart {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
/// A space/memory efficient representation of a chart.
///
/// Contains a collection of [beats](Beat) and [BPM changes](BpmChange).
/// These are not used by [RRR](crate::RRR) directly.
pub struct Chart {
    bpm_changes: Vec<BpmChange>,
    beats: Vec<Beat>,
}

impl Chart {
    #[must_use]
    pub fn new(beats: &[Beat], bpm_changes: &[BpmChange]) -> Self {
        Self {
            beats: beats.into(),
            bpm_changes: bpm_changes.into(),
        }
    }

    pub fn compile(&self) -> CompiledChart {
        CompiledChart {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_chart() -> Result<(), ()> {
        let note = Note::new(0);
        let note_row = NoteRow::new(0, &[note]);

        let beat = Beat::new(&[note_row], 4);

        let bpm_change = BpmChange::new(120., 0);

        let _chart = Chart::new(&[beat], &[bpm_change]);

        Ok(())
    }

    #[test]
    fn compile_chart() {
        let chart = Chart::default();
        let _compiled_chart: CompiledChart = chart.compile();
    }
}
