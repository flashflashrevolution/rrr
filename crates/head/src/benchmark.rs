pub(crate) struct BenchmarkData {
    pub frame_times: Vec<f64>,
    pub min_frame_time: f64,
    pub max_frame_time: f64,
    pub avg_frame_time: f64,
    pub skipped_frames: u64,
}

impl BenchmarkData {
    pub(crate) fn new() -> Self {
        BenchmarkData {
            frame_times: Vec::new(),
            min_frame_time: f64::MAX,
            max_frame_time: f64::MIN,
            avg_frame_time: 0.0,
            skipped_frames: 0,
        }
    }

    pub(crate) fn add_frame_time(&mut self, frame_time: f64) {
        self.frame_times.push(frame_time);
        self.min_frame_time = self.min_frame_time.min(frame_time);
        self.max_frame_time = self.max_frame_time.max(frame_time);

        if self.frame_times.len() > 60 {
            self.avg_frame_time = self.frame_times[self.frame_times.len() - 60..]
                .iter()
                .fold(0.0, |a, b| a + b)
                / 60.0;

            if frame_time > self.avg_frame_time * 1.1 {
                if let Some(res) = self.skipped_frames.checked_add(1) {
                    self.skipped_frames = res;
                }
            }
        }
    }
}
