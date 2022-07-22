use std::time::Duration;

use crate::CompiledChart;

#[derive(Debug)]
pub struct Record {
    mp3: Vec<u8>,
    chart: CompiledChart,
}

impl Record {
    #[must_use]
    pub fn new(mp3: Vec<u8>, chart: CompiledChart) -> Self {
        Self { mp3, chart }
    }

    #[must_use]
    pub fn mp3(&self) -> &[u8] {
        self.mp3.as_ref()
    }

    #[must_use]
    pub fn chart(&self) -> &CompiledChart {
        &self.chart
    }

    #[must_use]
    pub fn duration(&self) -> Duration {
        if let Some(last_note) = self.chart.notes.last() {
            last_note.timestamp
        } else {
            Duration::new(0, 0)
        }
    }
}
