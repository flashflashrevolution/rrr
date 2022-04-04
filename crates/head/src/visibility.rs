#[cfg(target_arch = "wasm32")]
pub fn register_on_visibility_change_listener(window: &web_sys::Window) {
    use wasm_bindgen::{prelude::Closure, JsCast};

    let closure = Closure::wrap(Box::new(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        if document.hidden() {
            log::info!("document hidden");
        } else {
            log::info!("document visible");
        }
    }) as Box<dyn Fn()>);

    let document = window.document().unwrap();
    document.set_onvisibilitychange(Some(closure.as_ref().unchecked_ref()));

    closure.forget();
}
