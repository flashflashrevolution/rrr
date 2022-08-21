mod fetch;
pub mod time;

pub use fetch::Fetcher;

#[cfg(target_arch = "wasm32")]
pub use fetch::FetchWorker;
