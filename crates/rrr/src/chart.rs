use num_rational::Rational;
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
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

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// Stores the [notes](Note) that belong to a single row in a [beat](Beat).
pub struct NoteRow {
    offset: Rational,
    // TODO: This should be [Note; N] but const generics aren't stable yet.
    notes: Vec<Note>,
}

impl Default for NoteRow {
    fn default() -> Self {
        Self {
            offset: Rational::new(0, 1),
            notes: Vec::default(),
        }
    }
}

impl NoteRow {
    #[must_use]
    pub fn new(offset: Rational, notes: &[Note]) -> Self {
        Self {
            offset,
            notes: notes.into(),
        }
    }
}

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
/// Stores all of the [note rows](NoteRow) that represent a beat.
pub struct Beat {
    note_rows: Vec<NoteRow>,
}

impl Beat {
    #[must_use]
    pub fn new(note_rows: &[NoteRow]) -> Self {
        Self {
            note_rows: note_rows.into(),
        }
    }
}

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// Represents a BPM change in a [chart](Chart).
pub struct BpmChange {
    bpm: Rational,
    beat: usize,
}

impl Default for BpmChange {
    fn default() -> Self {
        Self {
            bpm: Rational::new(0, 1),
            beat: 0,
        }
    }
}

impl BpmChange {
    #[must_use]
    pub fn new(bpm: Rational, beat: usize) -> Self {
        Self { bpm, beat }
    }
}

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
/// A runtime efficient representation of a chart used by an [RRR](crate::RRR) instance.
pub struct CompiledChart {}

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
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
        let offset = Rational::new(0, 1);
        let note = Note::new(0);
        let note_row = NoteRow::new(offset, &[note]);

        let beat = Beat::new(&[note_row]);

        let bpm = Rational::approximate_float(120.).ok_or(())?;
        let bpm_change = BpmChange::new(bpm, 0);

        let _chart = Chart::new(&[beat], &[bpm_change]);

        Ok(())
    }

    #[test]
    fn compile_chart() {
        let chart = Chart::default();
        let _compiled_chart: CompiledChart = chart.compile();
    }
}
