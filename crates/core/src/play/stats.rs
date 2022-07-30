pub struct PlayStats {
    amazings: u32,
    perfects: u32,
    goods: u32,
    averages: u32,
    misses: u32,
    boos: u32,
}

impl PlayStats {
    #[must_use]
    pub fn default() -> Self {
        Self {
            amazings: 0,
            perfects: 0,
            goods: 0,
            averages: 0,
            misses: 0,
            boos: 0,
        }
    }
}
