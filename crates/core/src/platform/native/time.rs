use crate::platform::TimeTrait;
use std::time::Duration;
use std::time::Instant;

#[derive(Copy, Clone)]
pub struct Time(Instant);

impl TimeTrait for Time {
    fn now() -> Self {
        Self(Instant::now())
    }

    fn ms_since(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }

    fn sub(&self, other: &Self) -> f64 {
        self.0.duration_since(other.0).as_secs_f64()
    }
}
