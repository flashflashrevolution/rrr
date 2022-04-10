extern crate rrr;

mod head;
mod visibility;

use game_loop::{game_loop, Time, TimeTrait};
use head::Head;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use std::time::Duration;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / 60 as u64);

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
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
    pub pixels: Pixels,
    pub world: World,
    pub head: Head,
    pub input: WinitInputHelper,
}

impl Game {
    fn update(&mut self) {
        self.head.tick();
        self.world.update();
    }
}

struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
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
    log::trace!("This is a trace");
    log::debug!("This is a debug");
    log::info!("This is an info");
    log::warn!("This is a warning");
    log::error!("This is an error");

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels + Web")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("WindowBuilder error")
    };

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        // Retrieve current width and height dimensions of browser client window
        let get_window_size = || {
            let client_window = web_sys::window().unwrap();
            LogicalSize::new(
                client_window.inner_width().unwrap().as_f64().unwrap(),
                client_window.inner_height().unwrap().as_f64().unwrap(),
            )
        };

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
                canvas.set_width(800);
                canvas.set_height(600);
                canvas_div
                    .append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");

        // let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
        //     let size = get_window_size();
        //     window.set_inner_size(size)
        // }) as Box<dyn FnMut(_)>);

        // client_window
        //     .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        //     .unwrap();

        // closure.forget();
    }

    let input = WinitInputHelper::new();

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new_async(WIDTH, HEIGHT, surface_texture)
            .await
            .expect("Pixels error")
    };

    let world = World::new();
    let head = head::Head::new();

    let game = Game {
        pixels,
        world,
        head,
        input,
    };

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
            // Drawing
            g.game.world.draw(g.game.pixels.get_frame());
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
        },
        |g, event| {
            log::trace!("{:?}", event);
            if g.game.input.update(&event) {
                // Close events
                if g.game.input.key_pressed(VirtualKeyCode::Escape) || g.game.input.quit() {
                    g.exit();
                }

                if g.game.input.key_pressed(VirtualKeyCode::Space) {
                    g.game.head.play_song();
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
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
            dt: Duration::default(),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        self.dt += TIME_STEP;

        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
