use std::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    note::{Color, CompiledNote, Direction, NoteRow},
    BpmChange,
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// A runtime efficient representation of a chart used by an [RRR](crate::RRR) instance.
pub struct CompiledChart {
    pub notes: Vec<CompiledNote>,
}

impl CompiledChart {
    #[must_use]
    pub fn new(notes: &[CompiledNote]) -> Self {
        Self {
            notes: notes.to_vec(),
        }
    }
}

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
        CompiledChart {
            notes: vec![
                CompiledNote {
                    beat_position: 100,
                    color: Color::Red,
                    direction: Direction::Up,
                    timestamp: Duration::from_millis(0)
                };
                100
            ],
        }
    }

    #[must_use]
    pub fn get_first_bpm(&self) -> Option<f32> {
        self.bpm_changes.first().map(|first_bpm| first_bpm.bpm())
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

#[cfg(test)]
mod tests {
    use crate::note::{Note, NoteRow};

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
