use anyhow::Error;
use rrr_head::prelude::winit::{
    self,
    dpi::PhysicalSize,
    event_loop::EventLoop,
    platform::web::WindowBuilderExtWebSys,
    window::{Window, WindowBuilder},
};
use rrr_head::{platform::platform::time::Time, query};
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast};
use web_sys::HtmlCanvasElement;

pub fn build_window(
    event_loop: &EventLoop<()>,
    canvas: Option<HtmlCanvasElement>,
    size: PhysicalSize<u32>,
) -> Result<winit::window::Window, winit::error::OsError> {
    {
        log::debug!("Inner Size {:?}", size);
        WindowBuilder::new()
            .with_title("Rust Rust Revolution")
            .with_canvas(canvas)
            .with_inner_size(size)
            .with_resizable(false)
            .with_max_inner_size(size)
            .with_min_inner_size(size)
            .build(event_loop)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn initialize() {
    console_log::init().unwrap();
    log::info!("RRR loaded.");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn play(canvas: Option<HtmlCanvasElement>, width: u32, height: u32) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_bindgen_futures::spawn_local(async move {
        let size = PhysicalSize::new(width, height);
        let event_loop = EventLoop::new();
        if let Ok(window) = initialize_window(canvas, size, &event_loop).await {
            let extracted_settings: Option<query::SettingsMerge> =
                { Some(query::get_optional_settings()) };

            let mut game = rrr_head::Game::<Time>::new(None, width, height);
            game.with_settings(extracted_settings);

            rrr_head::run_game_loop(window, size, event_loop, game).await;
        }
    });
}

async fn initialize_window(
    canvas: Option<HtmlCanvasElement>,
    size: PhysicalSize<u32>,
    event_loop: &EventLoop<()>,
) -> Result<Window, Error> {
    let window = build_window(&event_loop, canvas.clone(), size)?;

    if let Some(client_window) = web_sys::window() {
        register_on_visibility_change_listener(&client_window);
    } else {
        return Err(anyhow::anyhow!(
            "Could not get window from web_sys".to_string()
        ));
    }

    {
        let onblur = Closure::wrap(Box::new(move |e: web_sys::FocusEvent| {
            if let Some(target) = e.current_target() {
                if let Some(canvas_element) = target.dyn_ref::<HtmlCanvasElement>() {
                    let canvas = Rc::new(canvas_element.to_owned());
                    let focus = Closure::wrap(Box::new(move || {
                        canvas.to_owned().focus().ok();
                    }) as Box<dyn Fn()>);
                    web_sys::window().and_then(|win| {
                        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                            focus.as_ref().unchecked_ref(),
                            0,
                        );
                        Some(win)
                    });
                    focus.forget();
                } else {
                    log::error!("Could not get canvas from target");
                }
            } else {
                log::error!("Could not get current target");
            }
        }) as Box<dyn FnMut(web_sys::FocusEvent)>);

        if let Some(canvas) = canvas {
            canvas.set_onblur(Some(onblur.as_ref().unchecked_ref()));
            canvas.set_tab_index(1);
            canvas.focus().ok();
        }

        onblur.forget();
    }
    Ok(window)
}

pub fn register_on_visibility_change_listener(window: &web_sys::Window) {
    use wasm_bindgen::JsCast;

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
