#![deny(rust_2018_idioms)]
#![warn(
    elided_lifetimes_in_paths,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    variant_size_differences,
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::pattern_type_mismatch,
    clippy::print_stdout,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::shadow_reuse,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads
)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]
#![feature(array_chunks)]

mod geo;
mod noteskin;
mod sprites;
mod visibility;

use crate::geo::Point;
use anyhow::{Error, Result};
use log::error;
use pixels::{Pixels, SurfaceTexture};
use rrr_core::{
    download_chart,
    note::Color,
    play,
    play::Play,
    time::{performance::Time, TimeTrait},
    turntable, Record, SwfParser, Turntable, TurntableState,
};
use sprites::blit;
use sprites::trumpet_blit;

use winit::{
    dpi::LogicalSize,
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::f64;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 768;

struct Game<T: TimeTrait> {
    noteskin: Option<noteskin::Definition>,
    pixels: Pixels,
    play_stage: Option<Play<play::Active>>,
    previous_instant: T,
    current_instant: T,
    rect_x: f64,
    rect_y: f64,
    drift_x: f64,
    drift_y: f64,
}

impl<T: TimeTrait> Game<T> {
    fn new(
        noteskin: Option<noteskin::Definition>,
        pixels: Pixels,
        play_stage: Option<Play<play::Active>>,
    ) -> Self {
        Self {
            noteskin,
            pixels,
            play_stage,
            previous_instant: T::now(),
            current_instant: T::now(),
            rect_x: 100.,
            rect_y: 150.,
            drift_x: 0.,
            drift_y: 0.,
        }
    }

    pub fn load(&mut self, chart_id: usize) {}

    pub fn init(&mut self) {
        let noteskin_bytes = get_noteskin();
        let noteskin_image = match image::load_from_memory(noteskin_bytes) {
            Ok(image) => image,
            Err(err) => {
                error!("Could not load noteskin: {}", err);
                return;
            }
        };
        let rgba_representation = noteskin_image.to_rgba8();
        let image_bytes = rgba_representation.into_raw();

        // Definition for the default noteskin.
        self.noteskin.replace(noteskin::Definition::new(
            64,
            64,
            [
                Color::Blue,
                Color::Orange,
                Color::Red,
                Color::Cyan,
                Color::Pink,
                Color::White,
                Color::Green,
                Color::Purple,
                Color::Yellow,
                Color::Receptor,
            ]
            .to_vec(),
            [0, 90, 180, 270].to_vec(),
            noteskin_image,
            3,
        ));
    }

    fn update(&mut self) {
        self.current_instant = T::now();

        self.rect_x += 1.;
        self.rect_y += 1.;
        self.drift_x += 0.;
        self.drift_y -= 0.;

        let delta_time = self.current_instant.sub(&self.previous_instant);

        if let Some(stage) = &mut self.play_stage {
            stage.tick(delta_time);
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self) {
        let frame = self.pixels.get_frame();
        clear(frame);

        if let Some(play) = &self.play_stage {
            if let Some(noteskin) = &self.noteskin {
                let view = play.view();
                // Filter out notes that aren't on screen.
                // Render all notes.
                let x_limit = WIDTH as usize / 64 as usize;
                for (i, note) in view.iter().enumerate() {
                    let x = (((i % x_limit) * 64) as f64) + self.drift_x;
                    let y = (((i / x_limit) * 64) as f64) + self.drift_y;
                    trumpet_blit(
                        frame,
                        x,
                        y,
                        &note.direction,
                        &noteskin.get_note(note.color),
                    );
                }
            }
        }

        rect(frame, self.rect_x, self.rect_y, 32., 32.);
        // if let Some(play) = &self.play_stage {
        //     if let Some(noteskin) = &self.noteskin {
        //         trumpet_blit(frame, 200., 74., &noteskin.get_note(Color::Red))
        //     }
        // }
    }

    fn finish(&mut self) {
        self.previous_instant = self.current_instant;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "console_log", target_arch = "wasm32"))] {
        fn init_log() {
            console_log::init().unwrap();
        }
    } else {
        fn init_log() { simple_logger::init().unwrap(); }
    }
}

fn get_noteskin() -> &'static [u8] {
    include_bytes!("../../../data/default_noteskin.png")
}

fn main() {
    init_log();

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::UnwrapThrowExt;
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_bindgen_futures::spawn_local(async {
            run().await.unwrap_throw();
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use futures::executor;
        use std::env;

        match executor::block_on(run()) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "{:?} exited with bad form: {}",
                    env::current_exe().ok(),
                    err
                );
            }
        }
    }
}

fn build_window(
    event_loop: &EventLoop<()>,
) -> Result<winit::window::Window, winit::error::OsError> {
    {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        WindowBuilder::new()
            .with_title("Rust Rust Revolution")
            .with_inner_size(size)
            .with_resizable(false)
            .with_transparent(true)
            .build(event_loop)
    }
}

async fn run() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = build_window(&event_loop)?;

    #[cfg(target_arch = "wasm32")]
    {
        use std::rc::Rc;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use web_sys::{Element, HtmlCanvasElement, HtmlElement};
        use winit::platform::web::WindowExtWebSys;

        // Initialize winit window with current dimensions of browser client
        window.set_inner_size(LogicalSize::new(WIDTH, HEIGHT));

        if let Some(client_window) = web_sys::window() {
            visibility::register_on_visibility_change_listener(&client_window);
        } else {
            return Err(anyhow::anyhow!(
                "Could not get window from web_sys".to_string()
            ));
        }

        {
            let onblur = Closure::wrap(Box::new(move |e: web_sys::FocusEvent| {
                if let Some(target) = e.current_target() {
                    if let canvas =
                        Rc::new(target.dyn_ref::<HtmlCanvasElement>().unwrap().to_owned())
                    {
                        let focus = Closure::wrap(Box::new(move || {
                            canvas.to_owned().focus().ok();
                        }) as Box<dyn Fn()>);
                        web_sys::window().and_then(|win| {
                            win.set_timeout_with_callback_and_timeout_and_arguments_0(
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

            // Attach winit canvas to body element
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.get_element_by_id("canvas"))
                .and_then(|canvas_div: Element| {
                    let canvas: HtmlCanvasElement = window.canvas();
                    canvas.set_class_name("game");
                    canvas.set_id("rrr");
                    canvas.set_width(WIDTH);
                    canvas.set_height(HEIGHT);
                    let res = canvas_div
                        .append_child(&web_sys::Element::from(window.canvas()))
                        .ok();
                    canvas.set_onblur(Some(onblur.as_ref().unchecked_ref()));
                    canvas.set_tab_index(1);
                    canvas.focus().ok();
                    res
                });

            onblur.forget();
        }
    }

    run_game_loop(window, event_loop).await
}

async fn run_game_loop(
    window: winit::window::Window,
    event_loop: EventLoop<()>,
) -> Result<(), anyhow::Error> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = if let Ok(pixels) = Pixels::new_async(WIDTH, HEIGHT, surface_texture).await {
        pixels
    } else {
        Err(anyhow::anyhow!("Could not initialize Pixels renderer."))?
    };

    let mut game = Game::<Time>::new(None, pixels, None);

    game.init();
    game.load(3348);

    window.focus_window();

    let mut modifiers = ModifiersState::default();

    event_loop.run(move |in_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match in_event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    game.pixels.resize_surface(size.width, size.height);
                }
                WindowEvent::Focused(focused) => {
                    log::info!("Window {:?} focused: {:?}", &window.id(), focused);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    use winit::event::VirtualKeyCode::{Escape, Space, G, H};
                    match key {
                        Escape => *control_flow = ControlFlow::Exit,
                        G => window.set_cursor_grab(!modifiers.shift()).unwrap(),
                        H => window.set_cursor_visible(modifiers.shift()),
                        Space => {
                            if game.play_stage.is_none() {
                                if let Some(raw_chart) = download_chart(3348) {
                                    let parser_compressed = SwfParser::new(*raw_chart);
                                    let record = if let Ok(ready_to_parse) =
                                        parser_compressed.decompress()
                                    {
                                        let parsing = ready_to_parse.parse();
                                        // TODO: Make this async, remove intermediate state and just await it.
                                        let parsed = parsing.tick();
                                        Some(parsed.produce_tape())
                                    } else {
                                        None
                                    };

                                    game.play_stage =
                                        Some(Play::new(Turntable::load(record.unwrap())).start());
                                }
                            } else {
                                game.play_stage = None;
                            }
                        }
                        _ => log::info!("Key: {:?}", key),
                    }
                }
                WindowEvent::ModifiersChanged(m) => modifiers = m,
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta: _ } => (),
                DeviceEvent::Button { button: _, state } => match state {
                    ElementState::Pressed => (),
                    ElementState::Released => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                game.draw();
                if let Err(e) = game.pixels.render() {
                    log::error!("pixels.render() failed: {}", e);
                    *control_flow = ControlFlow::Exit;
                }
                game.update();
                window.request_redraw();
            }
            Event::RedrawEventsCleared => {
                game.finish();
            }
            _ => (),
        }
    });
}

fn clear(screen: &mut [u8]) {
    for (i, byte) in screen.iter_mut().enumerate() {
        *byte = if i % 4 == 3 { 255 } else { 0 };
    }
}

fn rect(screen: &mut [u8], x: f64, y: f64, width: f64, height: f64) {
    let x_min: f64 = f64::max(0., x);
    let x_max: f64 = f64::min(WIDTH as f64, x + width);
    let y_min: f64 = f64::max(0., y);
    let y_max: f64 = f64::min(HEIGHT as f64, y + height);

    let x_min_u: usize = x_min.round() as usize;
    let x_max_u: usize = x_max.round() as usize;
    let y_min_u: usize = y_min.round() as usize;
    let y_max_u: usize = y_max.round() as usize;

    for row in y_min_u..y_max_u {
        for column in x_min_u..x_max_u {
            let i: usize = (row * (WIDTH as usize) + column) * 4;
            screen[i] = 0x5e;
            screen[i + 1] = 0x48;
            screen[i + 2] = 0xe8;
            screen[i + 3] = 0xff;
        }
    }
}
