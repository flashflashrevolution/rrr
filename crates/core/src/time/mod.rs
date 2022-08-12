pub mod time_trait;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native_time/mod.rs"]
pub mod performance;

#[cfg(target_arch = "wasm32")]
#[path = "wasm_time/mod.rs"]
pub mod performance;
