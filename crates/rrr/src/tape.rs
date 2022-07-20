use crate::CompiledChart;

#[derive(Debug)]
pub struct Tape {
    mp3: Vec<u8>,
    chart: CompiledChart,
}

impl Tape {
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
}
