#[cfg(target_arch = "wasm32")]
use gloo_worker::Registrable;
#[cfg(target_arch = "wasm32")]
use rrr_head::platform::platform::FetchWorker;

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    FetchWorker::registrar().register();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Not implemented for native.
}
