pub trait TimeTrait: Copy {
    fn now() -> Self;
    fn sub(&self, other: &Self) -> f64;
}

#[cfg(not(target_arch = "wasm32"))]
pub mod performance {
    use super::TimeTrait;
    use std::time::Instant;

    #[derive(Copy, Clone)]
    pub struct Time(Instant);

    impl TimeTrait for Time {
        fn now() -> Self {
            Self(Instant::now())
        }

        fn sub(&self, other: &Self) -> f64 {
            self.0.duration_since(other.0).as_secs_f64()
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod performance {
    use super::*;
    use web_sys::window;

    #[derive(Copy, Clone)]
    pub struct Time(f64);

    impl TimeTrait for Time {
        fn now() -> Self {
            Self(window().unwrap().performance().unwrap().now() / 1000.)
        }

        fn sub(&self, other: &Self) -> f64 {
            self.0 - other.0
        }
    }
}
