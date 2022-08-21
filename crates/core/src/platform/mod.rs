#[cfg(not(target_arch = "wasm32"))]
#[path = "native/mod.rs"]
pub mod platform;
#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;
