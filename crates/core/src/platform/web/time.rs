#![allow(clippy::expect_used)]

use crate::platform::TimeTrait;
use std::ops::Sub;
use web_sys::window;

#[derive(Copy, Clone)]
pub struct Time(f64);

impl TimeTrait for Time {
    fn now() -> Self {
        Self(
            window()
                .expect("WASM Time cannot be used without an active window.")
                .performance()
                .expect("Performance feature must be present in WebSys dependency.")
                .now()
                / 1000.,
        )
    }

    fn ms_since(&self) -> f64 {
        Self::now() - *self
    }

    fn sub(&self, other: &Self) -> f64 {
        self.0 - other.0
    }
}

impl Sub for Time {
    type Output = f64;
    fn sub(self, other: Self) -> f64 {
        self.0 - other.0
    }
}
