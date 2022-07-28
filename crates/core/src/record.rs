use std::{collections::BTreeMap, time::Duration};

use crate::{chart, note::CompiledNote, CompiledChart};

#[derive(Debug)]
pub struct Record {
    pub optimized_chart: BTreeMap<Duration, CompiledNote>,
    pub mp3: Vec<u8>,
    pub chart: CompiledChart,
    pub duration: Duration,
}

impl Record {
    /// # Errors
    /// If duration of the chart is invalid, returns an error.
    pub fn new(mp3: Vec<u8>, chart: CompiledChart) -> Result<Self, anyhow::Error> {
        if let Ok(duration) = chart.get_duration() {
            Ok(Self {
                optimized_chart: create_optimized_chart(&chart),
                mp3,
                chart,
                duration,
            })
        } else {
            Err(anyhow::anyhow!("Invalid chart of unknown length."))
        }
    }
}

fn create_optimized_chart(chart: &CompiledChart) -> BTreeMap<Duration, CompiledNote> {
    let mut optimized_chart = BTreeMap::new();
    for note in &chart.notes {
        optimized_chart.insert(note.timestamp, note.clone());
    }
    optimized_chart
}
