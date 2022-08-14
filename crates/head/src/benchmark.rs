pub struct BenchmarkData {
    pub frame_times: Vec<f64>,
    pub min_frame_time: f64,
    pub max_frame_time: f64,
    pub avg_frame_time: f64,
}

impl BenchmarkData {
    pub fn new() -> Self {
        BenchmarkData {
            frame_times: Vec::new(),
            min_frame_time: f64::MAX,
            max_frame_time: f64::MIN,
            avg_frame_time: 0.0,
        }
    }

    pub fn add_frame_time(&mut self, frame_time: f64) {
        self.frame_times.push(frame_time);
        self.min_frame_time = self.min_frame_time.min(frame_time);
        self.max_frame_time = self.max_frame_time.max(frame_time);
        self.avg_frame_time = self
            .frame_times
            .iter()
            .rev()
            .take(60)
            .fold(0.0, |a, b| a + b)
            / 60.0;
    }
}
