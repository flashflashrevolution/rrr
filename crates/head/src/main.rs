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
mod head;
mod noteskin;
mod sprites;
mod visibility;

use crate::geo::*;
use crate::sprites::*;
use game_loop::{game_loop, Time, TimeTrait};
use head::Head;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use rrr::Color;
use rrr::{Chart, CompiledChart};
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
const BOX_SIZE: i16 = 64;

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
    head: Head,
    pixels: Pixels,
    play_stage: Option<World>,
    input: WinitInputHelper,
}

impl Game {
    fn new(
        noteskin: Option<noteskin::Definition>,
        head: Head,
        pixels: Pixels,
        play_stage: Option<World>,
        input: WinitInputHelper,
    ) -> Self {
        Self {
            noteskin,
            head,
            pixels,
            play_stage,
            input,
        }
    }

    pub fn load(&mut self, chart_id: usize) {
        self.head.load_chart(chart_id);
        self.play_stage = Some(World::new(self.head.chart()));
    }

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
        self.head.tick();
        if let Some(stage) = &mut self.play_stage {
            stage.update();
        }
    }
}

struct World {
    active_chart: Option<CompiledChart>,
    dt: Duration,
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
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_bindgen_futures::spawn_local(run());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }
}

async fn run() {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        WindowBuilder::new()
            .with_title("Rust Rust Revolution")
            .with_inner_size(size)
            .with_resizable(false)
            .with_transparent(true)
            .build(&event_loop)
            .expect("WindowBuilder error")
    };

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        // Initialize winit window with current dimensions of browser client
        window.set_inner_size(LogicalSize::new(WIDTH, HEIGHT));

        let client_window = web_sys::window().unwrap();

        visibility::register_on_visibility_change_listener(&client_window);

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
            })
            .expect("couldn't append canvas to document body");
    }

    let input = WinitInputHelper::new();

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new_async(WIDTH, HEIGHT, surface_texture)
            .await
            .expect("Pixels error")
    };

    let mut game = Game::new(None, head::Head::new(), pixels, None, input);
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
            if let Some(stage) = &mut g.game.play_stage {
                stage.draw(g.game.pixels.get_frame(), &g.game.noteskin);
                if let Err(e) = g.game.pixels.render() {
                    error!("pixels.render() failed: {}", e);
                    g.exit();
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    // Sleep the main thread to limit drawing to the fixed time step.
                    // See: https://github.com/parasyte/pixels/issues/174
                    let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
                    if dt > 0.0 {
                        std::thread::sleep(Duration::from_secs_f64(dt));
                    }
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
                    g.game.head.play_song().ok();
                }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    g.game.pixels.resize_surface(size.width, size.height);
                }
            }
        },
    );
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new(chart: Option<CompiledChart>) -> Self {
        Self {
            active_chart: chart,
            dt: Duration::default(),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        self.dt += TIME_STEP;

        // TODO: Spawn arrows and begin to move them up the field at delta rate.
        // - Get chart.
        // - Spawn arrows in order based on time-to-target offset. (See how we do this in R^3).
        // - Destroy arrows when they hit the top of the screen.
        // - Destroy arrows when they are on a recepor when the player activates it.
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8], noteskin: &Option<noteskin::Definition>) {
        // Draw shit
        clear(frame);

        if let Some(noteskin) = noteskin {
            if let Some(chart) = &self.active_chart {
                let some_notes = &chart.notes[7..15];

                // Filter out notes that aren't on screen.
                // Render all notes.
                let mut i = 0;
                for note in some_notes {
                    blit(
                        frame,
                        &Point { x: 64 * i, y: 0 },
                        &note.direction,
                        &noteskin.get_note(note.color),
                    );
                    i += 1;
                }
            }
        }

        rect(frame, 150, 100, 32, 32);
    }
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
