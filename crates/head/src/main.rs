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
#![feature(poll_ready)]

mod benchmark;
mod geo;
mod noteskin;
mod sprites;
mod visibility;

use anyhow::Error;
use benchmark::BenchmarkData;
use inter_struct::prelude::*;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use rrr_core::{
    fetch,
    math::lerp::Lerp,
    note::{self, Direction},
    parser::swf,
    platform::{platform, TimeTrait},
    play::{self, Play},
    query_settings,
    settings::{self, Settings},
    time::Time,
    Turntable,
};
use sprites::blit;
use std::f64;
use winit::{
    dpi::LogicalSize,
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, ModifiersState, TouchPhase, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 720;

struct GameRenderer {
    noteskin: noteskin::Definition,
    pixels: Pixels,
}

impl GameRenderer {
    fn new(pixels: Pixels) -> Self {
        Self {
            noteskin: noteskin::Definition::default(),
            pixels,
        }
    }
}

struct Action {
    direction: Direction,
    ts: i128,
}

struct Game<T: TimeTrait> {
    renderer: Option<GameRenderer>,
    play_stage: Option<Play<play::Active>>,
    fetcher: Option<platform::Fetcher>,
    start_instant: T,
    previous_instant: T,
    current_instant: T,
    action_queue: Vec<Action>,
    benchmark_data: BenchmarkData,
    settings: Settings,
}

impl<T> Game<T>
where
    T: TimeTrait,
{
    fn new(play_stage: Option<Play<play::Active>>) -> Self {
        Self {
            renderer: None,
            play_stage,
            fetcher: None,
            start_instant: T::now(),
            previous_instant: T::now(),
            current_instant: T::now(),
            action_queue: Vec::new(),
            benchmark_data: BenchmarkData::new(),
            settings: Settings::default(),
        }
    }

    pub(crate) fn with_game_renderer(&mut self, renderer: GameRenderer) {
        self.renderer = Some(renderer);
    }

    pub(crate) fn start(&mut self) {
        self.start_instant = T::now();
    }

    pub(crate) fn load(&mut self, chart_id: usize) {}

    fn update(&mut self) {
        self.current_instant = T::now();
        let delta = self.current_instant.sub(&self.previous_instant);
        self.benchmark_data.add_frame_time(delta);

        let current_progress = (self.start_instant.ms_since() * 1000.) as u64;

        if let Some(stage) = &mut self.play_stage {
            stage.tick(current_progress);

            for actions in self.action_queue.drain(..) {
                stage.do_action(actions.direction, actions.ts);
            }
        }

        if let Some(mut fetcher) = self.fetcher.take() {
            let result = fetcher.fetch();

            match result {
                Some(bytes) => match bytes {
                    fetch::BytesFetch::Ok(chart) => {
                        let parser_compressed = swf::SwfParser::new(chart.to_vec());
                        let record = if let Ok(ready_to_parse) = parser_compressed.decompress() {
                            let parsing = ready_to_parse.parse();
                            let parsed = parsing.tick();
                            Some(parsed.produce_tape())
                        } else {
                            None
                        };

                        let turntable = Turntable::load(record.unwrap());

                        let play = Play::new(turntable).with_settings(self.settings);
                        let play_started = play.start();
                        self.play_stage = Some(play_started);

                        self.start();
                    }
                    fetch::BytesFetch::Wait => todo!(),
                    fetch::BytesFetch::Err(err) => log::error!("{}", err),
                },
                None => {
                    self.fetcher.replace(fetcher);
                }
            }
        }
    }

    fn do_action(&mut self, direction: Direction) {
        if let Some(stage) = &mut self.play_stage {
            self.action_queue.push(Action {
                direction,
                ts: (self.start_instant.ms_since() * 1000.) as i128,
            });
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            let frame = renderer.pixels.get_frame_mut();
            clear(frame);

            if let Some(play) = &self.play_stage {
                if let noteskin = &renderer.noteskin {
                    let time_on_screen = u64::from(play.settings().scroll_speed);
                    let field_height = HEIGHT as f64;
                    let note_height = noteskin.note_height as f64;
                    let offset = WIDTH as f64 / 2.0 - noteskin.note_width as f64 * 0.5;

                    let start_position = match play.settings().scroll_direction {
                        settings::ScrollDirection::Down => -note_height,
                        settings::ScrollDirection::Up => field_height,
                    };
                    let end_position = match play.settings().scroll_direction {
                        settings::ScrollDirection::Down => field_height,
                        settings::ScrollDirection::Up => -note_height,
                    };

                    let chart_progress = play.progress();

                    let position = get_pos_from_ms(
                        play.settings().judge_zero_point,
                        start_position,
                        end_position,
                        time_on_screen,
                    );

                    draw_receptors(play, noteskin, frame, offset, position);

                    draw_notes(
                        play,
                        time_on_screen,
                        chart_progress,
                        end_position,
                        start_position,
                        offset,
                        frame,
                        noteskin,
                    );
                }
            }
        }
    }

    fn finish(&mut self) {
        self.previous_instant = self.current_instant;
    }

    fn with_settings(&mut self, settings: Option<query_settings::SettingsMerge>) {
        if let Some(settings) = settings {
            self.settings.merge(settings);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_notes(
    play: &Play<play::Active>,
    time_on_screen: u64,
    chart_progress: u64,
    end_position: f64,
    start_position: f64,
    offset: f64,
    frame: &mut [u8],
    noteskin: &noteskin::Definition,
) {
    let view = play.view(time_on_screen);
    for (&duration, note) in view.filter(|(_, note)| !play.judgements().contains_key(note)) {
        let note_progress = duration - chart_progress as i128;
        let normalized = note_progress as f64 / time_on_screen as f64;
        let position = end_position.lerp(start_position, normalized);
        let lane_offset = play.settings().lane_gap as f64;

        let lane_index = match note.direction {
            note::Direction::Left => -1.5,
            note::Direction::Down => -0.5,
            note::Direction::Up => 0.5,
            note::Direction::Right => 1.5,
        };
        let x = offset + (lane_offset * lane_index);
        let y = position;
        blit(frame, x, y, &note.direction, &noteskin.get_note(note.color));
    }
}

fn get_pos_from_ms(ms: i128, start_position: f64, end_position: f64, time_on_screen: u64) -> f64 {
    let normalized = ms as f64 / time_on_screen as f64;
    let pos = end_position.lerp(start_position, normalized as f64);
    pos
}

fn draw_receptors(
    play: &Play<play::Active>,
    noteskin: &noteskin::Definition,
    frame: &mut [u8],
    offset: f64,
    position: f64,
) {
    let receptor_skin = noteskin.get_note(note::Color::Receptor);
    let lane_offset = f64::from(play.settings().lane_gap);
    blit(
        frame,
        offset + (lane_offset * -1.5),
        position,
        &note::Direction::Left,
        &receptor_skin,
    );
    blit(
        frame,
        offset + (lane_offset * -0.5),
        position,
        &note::Direction::Down,
        &receptor_skin,
    );
    blit(
        frame,
        offset + (lane_offset * 0.5),
        position,
        &note::Direction::Up,
        &receptor_skin,
    );
    blit(
        frame,
        offset + (lane_offset * 1.5),
        position,
        &note::Direction::Right,
        &receptor_skin,
    );
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
        use wasm_bindgen::UnwrapThrowExt;
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_bindgen_futures::spawn_local(async {
            run().await.unwrap_throw();
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use futures::executor;
        use log::error;
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
            .build(event_loop)
    }
}

async fn run() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = build_window(&event_loop)?;

    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::{Element, HtmlCanvasElement};

        use std::rc::Rc;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
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

    #[cfg(target_arch = "wasm32")]
    let extracted_settings: Option<query_settings::SettingsMerge> =
        { Some(query_settings::get_optional_settings()) };
    #[cfg(not(target_arch = "wasm32"))]
    let extracted_settings: Option<query_settings::SettingsMerge> = { None };

    run_game_loop(window, event_loop, extracted_settings).await
}

fn do_toggle_game_state_debug(game: &mut Game<Time>) {
    if game.play_stage.is_none() {
        if game.fetcher.is_none() {
            game.fetcher.replace(fetch::download_chart(3348));
        }
    } else {
        if let Some(play) = &game.play_stage {
            log::info!("{:?}", play.judgements());
        }
        game.play_stage = None;
    }
}

async fn run_game_loop(
    window: winit::window::Window,
    event_loop: EventLoop<()>,
    settings: Option<query_settings::SettingsMerge>,
) -> Result<(), anyhow::Error> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let pixels = if let Ok(pixels) = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
        .clear_color(pixels::wgpu::Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.0,
        })
        .build_async()
        .await
    {
        pixels
    } else {
        Err(anyhow::anyhow!("Could not initialize Pixels renderer."))?
    };

    let mut game = Game::<Time>::new(None);
    game.with_game_renderer(GameRenderer::new(pixels));
    game.with_settings(settings);
    game.load(3348);
    window.focus_window();

    #[cfg(target_arch = "wasm32")]
    let elements = {
        use web_sys::{Element, Window};

        struct Elements {
            update_progress: Element,
            avg_frame_time: Element,
            max_frame_time: Element,
            min_frame_time: Element,
            skipped_frames: Element,
        }

        let update_progress: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("progress"));

        let avg_frame_time: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("avg_frame_time"));

        let max_frame_time: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("max_frame_time"));

        let min_frame_time: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("min_frame_time"));

        let skipped_frames: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("skipped_frames"));

        Elements {
            update_progress: update_progress.unwrap(),
            avg_frame_time: avg_frame_time.unwrap(),
            max_frame_time: max_frame_time.unwrap(),
            min_frame_time: min_frame_time.unwrap(),
            skipped_frames: skipped_frames.unwrap(),
        }
    };

    let mut modifiers = ModifiersState::default();
    event_loop.run(move |in_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match in_event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    if let Some(renderer) = &mut game.renderer {
                        renderer.pixels.resize_surface(size.width, size.height);
                    }
                }
                WindowEvent::Focused(focused) => {
                    log::info!("Window {:?} focused: {:?}", &window.id(), focused);
                }
                WindowEvent::Touch(touch) => {
                    if touch.phase == TouchPhase::Ended {
                        do_toggle_game_state_debug(&mut game);
                    }
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    handle_keyboard_input(key, control_flow, &mut game, &window, modifiers);
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
                if let Some(renderer) = &game.renderer {
                    if let Err(e) = renderer.pixels.render() {
                        log::error!("pixels.render() failed: {}", e);
                        *control_flow = ControlFlow::Exit;
                    }

                    game.update();
                    game.draw();

                    #[cfg(target_arch = "wasm32")]
                    if let Some(play) = &game.play_stage {
                        elements
                            .update_progress
                            .set_inner_html(format!("{:?}", &play.progress()).as_str());
                        elements.avg_frame_time.set_inner_html(
                            format!("{:.2?}ms", &game.benchmark_data.avg_frame_time * 1000.0)
                                .as_str(),
                        );
                        elements.max_frame_time.set_inner_html(
                            format!("{:.2?}ms", &game.benchmark_data.max_frame_time * 1000.0)
                                .as_str(),
                        );
                        elements.min_frame_time.set_inner_html(
                            format!("{:.2?}ms", &game.benchmark_data.min_frame_time * 1000.0)
                                .as_str(),
                        );
                        elements.skipped_frames.set_inner_html(
                            format!("{:?}", &game.benchmark_data.skipped_frames).as_str(),
                        );
                    }

                    window.request_redraw();
                }
            }
            Event::RedrawEventsCleared => {
                game.finish();
            }
            _ => (),
        }
    });
}

fn handle_keyboard_input(
    key: winit::event::VirtualKeyCode,
    control_flow: &mut ControlFlow,
    game: &mut Game<Time>,
    window: &winit::window::Window,
    modifiers: ModifiersState,
) {
    use winit::event::VirtualKeyCode::{
        Comma, Down, Escape, Left, Period, Right, Slash, Space, Up, G, H, M,
    };
    match key {
        Escape => *control_flow = ControlFlow::Exit,
        Left => game.do_action(Direction::Left),
        Down => game.do_action(Direction::Down),
        Up => game.do_action(Direction::Up),
        Right => game.do_action(Direction::Right),
        M => game.do_action(Direction::Left),
        Comma => game.do_action(Direction::Down),
        Period => game.do_action(Direction::Up),
        Slash => game.do_action(Direction::Right),
        Space => {
            do_toggle_game_state_debug(game);
        }
        _ => log::info!("Key: {:?}", key),
    }
}

fn clear(frame: &mut [u8]) {
    frame.fill(0);
}
