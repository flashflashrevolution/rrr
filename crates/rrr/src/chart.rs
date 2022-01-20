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
    beats: Vec<Beat>,
    bpm_changes: Vec<BpmChange>,
}

impl Chart {
    #[must_use]
    pub fn new(beats: &[Beat], bpm_changes: &[BpmChange]) -> Self {
        Self {
            beats: beats.into(),
            bpm_changes: bpm_changes.into(),
        }
    }

    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn compile(&self) -> CompiledChart {
        CompiledChart {}
    }

    #[must_use]
    pub fn get_first_bpm(&self) -> Option<f32> {
        self.bpm_changes.first().map(|first_bpm| first_bpm.bpm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_chart() {
        const FIRST_BPM: f32 = 120.;
        let chart: Chart = Chart::new(
            &[{
                let note_rows: &[NoteRow] = &[NoteRow::new(0, &[Note::new(0)])];
                Beat {
                    note_rows: note_rows.into(),
                    subdivisions: 4,
                }
            }],
            &[BpmChange::new(FIRST_BPM, 0)],
        );

        assert_eq!(chart.get_first_bpm(), Some(FIRST_BPM));
    }

    #[test]
    fn compile_chart() {
        let chart = Chart::default();
        let _compiled_chart: CompiledChart = chart.compile();
    }
}
