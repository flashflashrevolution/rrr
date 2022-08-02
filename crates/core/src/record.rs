use crate::{note::CompiledNote, CompiledChart};
use btreemultimap::BTreeMultiMap;

#[derive(Debug)]
pub struct Record {
    pub optimized_chart: BTreeMultiMap<i128, CompiledNote>,
    pub mp3: Vec<u8>,
    pub chart: CompiledChart,
    pub duration: i128,
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

fn create_optimized_chart(chart: &CompiledChart) -> BTreeMultiMap<i128, CompiledNote> {
    let mut optimized_chart = BTreeMultiMap::new();
    for note in &chart.notes {
        optimized_chart.insert(note.timestamp, note.clone());
    }
    optimized_chart
}
