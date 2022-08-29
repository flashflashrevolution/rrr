#[cfg(not(target_arch = "wasm32"))]
#[path = "native/mod.rs"]
pub mod platform;
#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

pub trait TimeTrait: Copy {
    fn now() -> Self;
    fn ms_since(&self) -> f64;
    fn sub(&self, other: &Self) -> f64;
}
