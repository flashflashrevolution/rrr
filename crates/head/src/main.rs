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
use game_loop::game_loop;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use rrr::{
    download_chart, play::Play, turntable, Color, Record, SwfParser, Turntable, TurntableState,
};
use sprites::blit;
use std::time::Duration;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000_u64.div_euclid(60));

const WIDTH: u32 = 512;
const HEIGHT: u32 = 768;

trait DeltaTime {
    fn update(&mut self) -> usize;

    fn update_dt(dest_dt: &mut Duration, step: Duration) -> usize {
        *dest_dt += TIME_STEP;
        let frames = dest_dt.as_nanos() / step.as_nanos();
        *dest_dt -= Duration::from_nanos((frames * step.as_nanos()) as u64);

        frames as usize
    }
}

struct Game {
    noteskin: Option<noteskin::Definition>,
    pixels: Pixels,
    play_stage: Option<Play<rrr::play::Active>>,
    input: WinitInputHelper,
}

impl Game {
    fn new(
        noteskin: Option<noteskin::Definition>,
        pixels: Pixels,
        play_stage: Option<Play<rrr::play::Active>>,
        input: WinitInputHelper,
    ) -> Self {
        Self {
            noteskin,
            pixels,
            play_stage,
            input,
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
        if let Some(stage) = &mut self.play_stage {
            stage.tick(Duration::ZERO);
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
                    let x = (i % x_limit) * 64;
                    let y = (i / x_limit) * 64;
                    blit(
                        frame,
                        &Point { x, y },
                        &note.direction,
                        &noteskin.get_note(note.color),
                    );
                }
            }
        }

        rect(frame, 150, 100, 32, 32);
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

        // Attach winit canvas to body element
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("canvas"))
            .and_then(|canvas_div| {
                let canvas = window.canvas();
                canvas.set_class_name("game");
                canvas.set_width(WIDTH);
                canvas.set_height(HEIGHT);
                canvas_div
                    .append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            });
    }

    run_game_loop(window, event_loop).await
}

async fn run_game_loop(
    window: winit::window::Window,
    event_loop: EventLoop<()>,
) -> Result<(), Error> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = if let Ok(pixels) = Pixels::new_async(WIDTH, HEIGHT, surface_texture).await {
        pixels
    } else {
        Err(anyhow::anyhow!("Could not initialize Pixels renderer."))?
    };

    let input = WinitInputHelper::new();
    let mut game = Game::new(None, pixels, None, input);
    game.init();
    game.load(3348);
    game_loop(
        event_loop,
        window,
        game,
        120,
        0.1,
        |g| {
            g.game.update();
        },
        |g| {
            g.game.draw();
            if let Err(e) = g.game.pixels.render() {
                error!("pixels.render() failed: {}", e);
                g.exit();
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                use game_loop::{Time, TimeTrait};

                // Sleep the main thread to limit drawing to the fixed time step.
                // See: https://github.com/parasyte/pixels/issues/174
                let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
                if dt > 0.0 {
                    std::thread::sleep(Duration::from_secs_f64(dt));
                }
            }
        },
        |g, event| {
            log::trace!("{:?}", event);

            if let Event::WindowEvent {
                ref event,
                window_id,
            } = event
            {
                if let WindowEvent::Focused(focused) = event {
                    log::info!("Window {:?} focused {:?}", window_id, focused);
                }
            }

            #[allow(clippy::collapsible_match, clippy::single_match)]
            match event {
                Event::WindowEvent {
                    window_id,
                    ref event,
                } => match event {
                    WindowEvent::Focused(focused) => {
                        log::info!("Window {:?} focused {:?}", window_id, focused);
                    }
                    _ => {}
                },
                _ => {}
            }

            if g.game.input.update(&event) {
                // Close events
                if g.game.input.key_pressed(VirtualKeyCode::Escape) || g.game.input.quit() {
                    g.exit();
                }

                if g.game.input.key_pressed(VirtualKeyCode::Space) {
                    if let Some(raw_chart) = download_chart(3348) {
                        let parser_compressed = SwfParser::new(*raw_chart);
                        let tape = if let Ok(ready_to_parse) = parser_compressed.decompress() {
                            let parsing = ready_to_parse.parse();
                            // TODO: Make this async, remove intermediate state and just await it.
                            let parsed = parsing.tick();
                            Some(parsed.produce_tape())
                        } else {
                            None
                        };

                        g.game.play_stage = Some(Play::new(Turntable::load(tape.unwrap())).start());
                    }
                }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    g.game.pixels.resize_surface(size.width, size.height);
                }
            }
        },
    );
}

fn clear(screen: &mut [u8]) {
    for (i, byte) in screen.iter_mut().enumerate() {
        *byte = if i % 4 == 3 { 255 } else { 0 };
    }
}

fn rect(screen: &mut [u8], x: u32, y: u32, width: u32, height: u32) {
    for row in y..(y + height) {
        for column in x..(x + width) {
            let i: usize = ((row * WIDTH + column) * 4).try_into().unwrap();
            screen[i] = 0x5e;
            screen[i + 1] = 0x48;
            screen[i + 2] = 0xe8;
            screen[i + 3] = 0xff;
        }
    }
}
