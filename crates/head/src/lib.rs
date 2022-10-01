use crate::{
    platform::{platform::time::Time, platform::Fetcher, TimeTrait},
    query::SettingsMerge,
    settings::Settings,
};
use inter_struct::prelude::StructMerge;
use rrr_core::{
    chart::{NoteColor, NoteDirection, SwfParser},
    math::lerp::Lerp,
    play::{
        self,
        field::{self, Field},
        turntable::Turntable,
        Play,
    },
};
use rrr_render::{
    sprites::{self, DirectionValue},
    Renderer, RendererBuilder,
};
use winit::{
    dpi::PhysicalSize,
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, ModifiersState, TouchPhase, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen::prelude::wasm_bindgen(module = "\\/src\\/judgement.js")
)]
extern "C" {
    fn update_judgement(judgement_report: play::JudgementReport);
}

pub mod prelude {
    pub use anyhow;
    pub use futures;
    pub use log;
    pub use rrr_core::play;
    pub use winit;
}

pub mod benchmark;
pub mod fetch;
pub mod noteskin;
pub mod platform;
pub mod query;
pub mod settings;

struct Direction<'a>(&'a NoteDirection);
impl DirectionValue for Direction<'_> {
    fn value(&self) -> sprites::Direction {
        match self.0 {
            NoteDirection::Left => sprites::Direction::Left,
            NoteDirection::Down => sprites::Direction::Down,
            NoteDirection::Up => sprites::Direction::Up,
            NoteDirection::Right => sprites::Direction::Right,
        }
    }
}

pub fn build_window(
    event_loop: &EventLoop<()>,
    screen_width: u32,
    screen_height: u32,
) -> Result<winit::window::Window, winit::error::OsError> {
    {
        let size = PhysicalSize::new(screen_width, screen_height);
        WindowBuilder::new()
            .with_title("Rust Rust Revolution")
            .with_inner_size(size)
            .with_resizable(false)
            .build(event_loop)
    }
}

pub async fn run_game_loop(
    window: winit::window::Window,
    size: PhysicalSize<u32>,
    event_loop: EventLoop<()>,
    mut game: Game<Time>,
) -> Result<(), anyhow::Error> {
    let renderer = RendererBuilder::new(size.width, size.height, &window)
        .build()
        .await?;
    game.with_renderer(renderer);
    game.load(3378);
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
            amazings: Element,
            perfects: Element,
            goods: Element,
            averages: Element,
            misses: Element,
            boos: Element,
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

        let amazings: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("amazing"));
        let perfects: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("perfect"));
        let goods: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("good"));
        let averages: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("average"));
        let misses: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("miss"));
        let boos: Option<Element> = web_sys::window()
            .and_then(|win: Window| win.document())
            .and_then(|doc| doc.get_element_by_id("boo"));

        Elements {
            update_progress: update_progress.unwrap(),
            avg_frame_time: avg_frame_time.unwrap(),
            max_frame_time: max_frame_time.unwrap(),
            min_frame_time: min_frame_time.unwrap(),
            skipped_frames: skipped_frames.unwrap(),
            amazings: amazings.unwrap(),
            perfects: perfects.unwrap(),
            goods: goods.unwrap(),
            averages: averages.unwrap(),
            misses: misses.unwrap(),
            boos: boos.unwrap(),
        }
    };

    let mut modifiers = ModifiersState::default();
    event_loop.run(move |in_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match in_event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
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
                    if let Err(e) = renderer.renderer.pixels.render() {
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
                        elements.amazings.set_inner_html(
                            format!("{:?}", play.judgement_results().amazings).as_str(),
                        );
                        elements.perfects.set_inner_html(
                            format!("{:?}", play.judgement_results().perfects).as_str(),
                        );
                        elements.goods.set_inner_html(
                            format!("{:?}", play.judgement_results().goods).as_str(),
                        );
                        elements.averages.set_inner_html(
                            format!("{:?}", play.judgement_results().averages).as_str(),
                        );
                        elements.misses.set_inner_html(
                            format!("{:?}", play.judgement_results().misses).as_str(),
                        );
                        elements.boos.set_inner_html(
                            format!("{:?}", play.judgement_results().boos).as_str(),
                        );
                    }

                    window.set_inner_size(size);
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

pub struct Game<T: TimeTrait> {
    renderer: Option<GameRenderer>,
    note_height: usize,
    note_width: usize,
    screen_width: u32,
    screen_height: u32,
    play_stage: Option<Play<play::Active>>,
    fetcher: Option<Fetcher>,
    start_instant: T,
    previous_instant: T,
    current_instant: T,
    action_queue: Vec<Action>,
    benchmark_data: benchmark::BenchmarkData,
    settings: settings::Settings,
}

impl<T> Game<T>
where
    T: TimeTrait,
{
    pub fn new(
        play_stage: Option<Play<play::Active>>,
        screen_width: u32,
        screen_height: u32,
    ) -> Self {
        Self {
            renderer: None,
            play_stage,
            fetcher: None,
            start_instant: T::now(),
            previous_instant: T::now(),
            current_instant: T::now(),
            action_queue: Vec::new(),
            benchmark_data: benchmark::BenchmarkData::new(),
            settings: settings::Settings::default(),
            note_height: 1,
            note_width: 1,
            screen_width,
            screen_height,
        }
    }

    pub(crate) fn with_renderer(&mut self, renderer: Renderer) {
        self.renderer = Some(GameRenderer {
            noteskin: noteskin::Definition::default(),
            renderer,
        });

        if let Some(renderer) = &self.renderer {
            self.note_height = renderer.noteskin.note_height;
            self.note_width = renderer.noteskin.note_width;
        }
    }

    pub(crate) fn start(&mut self) {
        self.start_instant = T::now();
    }

    pub(crate) fn load(&mut self, chart_id: usize) {}

    fn update(&mut self) {
        self.current_instant = T::now();
        let delta = self.current_instant.sub(&self.previous_instant);
        self.benchmark_data.add_frame_time(delta);

        let current_progress = (self.start_instant.ms_since() * 1000.) as u32;

        if let Some(stage) = &mut self.play_stage {
            for actions in self.action_queue.drain(..) {
                stage.do_action(&actions.direction, actions.ts, self.settings.offset);
            }

            stage.tick(current_progress);
        }

        if let Some(mut fetcher) = self.fetcher.take() {
            let result = fetcher.fetch();

            match result {
                Some(bytes) => match bytes {
                    fetch::BytesFetch::Ok(chart) => {
                        let parser_compressed = SwfParser::new(chart.to_vec());
                        let record = if let Ok(ready_to_parse) = parser_compressed.decompress() {
                            let parsing = ready_to_parse.parse();
                            let parsed = parsing.tick();
                            Some(parsed.produce_tape())
                        } else {
                            None
                        };

                        let turntable = Turntable::load(record.unwrap());

                        // Calculate start and end points.
                        let field_height = self.screen_height as f32;
                        let note_height = self.note_height as f32;

                        let start_position = match self.settings.scroll_direction {
                            settings::ScrollDirection::Down => -note_height,
                            settings::ScrollDirection::Up => field_height,
                        };
                        let judge_position = match self.settings.scroll_direction {
                            settings::ScrollDirection::Down => {
                                self.screen_height as f32
                                    - self.settings.judge_position as f32
                                    - note_height as f32
                            }
                            settings::ScrollDirection::Up => self.settings.judge_position as f32,
                        };
                        let play = Play::new(
                            turntable,
                            Field {
                                start_position,
                                judge_position,
                            },
                        );
                        let play_started = if self.settings.muted {
                            play.start()
                        } else {
                            play.start_with_audio()
                        };
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

    fn do_action(&mut self, direction: NoteDirection) {
        if let Some(stage) = &mut self.play_stage {
            self.action_queue.push(Action {
                direction,
                ts: ((self.start_instant.ms_since() * 1000.) - self.settings.offset as f64) as u32,
            });
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            let frame = renderer.renderer.pixels.get_frame_mut();
            clear(frame);

            if let Some(play) = &self.play_stage {
                if let noteskin = &renderer.noteskin {
                    let time_on_screen = u32::from(self.settings.scroll_speed);
                    let offset = self.screen_width as f32 / 2.0 - noteskin.note_width as f32 * 0.5;
                    let chart_progress = play.progress();

                    let receptor_position = play.field().judge_position + 32.;

                    // Draw the zero point of the notes.
                    // These offsets don't make much sense.
                    draw_line(
                        frame,
                        get_pos_from_ms(
                            self.settings.offset.into(),
                            receptor_position,
                            play.field().start_position,
                            time_on_screen,
                        ),
                        self.screen_height,
                        self.screen_width,
                    );

                    draw_receptors(
                        play,
                        noteskin,
                        frame,
                        offset,
                        receptor_position - noteskin.note_height as f32 * 0.5,
                        self.settings.lane_gap,
                        self.screen_width,
                        self.screen_height,
                    );

                    draw_notes(
                        play,
                        time_on_screen,
                        chart_progress,
                        play.field(),
                        self.settings.offset,
                        offset,
                        frame,
                        noteskin,
                        self.settings.lane_gap,
                        self.screen_width,
                        self.screen_height,
                    );
                }

                unsafe {
                    update_judgement(*play.state.judgement_report());
                }
            }
        }
    }

    fn finish(&mut self) {
        self.previous_instant = self.current_instant;
    }

    pub fn with_settings(&mut self, settings: Option<SettingsMerge>) {
        if let Some(settings) = settings {
            self.settings.merge(settings);
        }
    }
}

pub struct GameRenderer {
    noteskin: noteskin::Definition,
    renderer: Renderer,
}

impl GameRenderer {
    fn new(renderer: Renderer) -> Self {
        Self {
            noteskin: noteskin::Definition::default(),
            renderer,
        }
    }
}

fn do_toggle_game_state_debug(game: &mut Game<Time>) {
    if game.play_stage.is_none() {
        if game.fetcher.is_none() {
            game.fetcher.replace(fetch::download_chart(3378));
        }
    } else {
        if let Some(play) = &game.play_stage {
            log::info!("{:?}", play.judgements());
        }
        game.play_stage = None;
    }
}

fn handle_keyboard_input(
    key: winit::event::VirtualKeyCode,
    control_flow: &mut ControlFlow,
    game: &mut Game<Time>,
    window: &winit::window::Window,
    modifiers: ModifiersState,
) {
    use winit::event::VirtualKeyCode::{
        Comma, Down, Escape, Left, Period, Right, Slash, Space, Up, M,
    };
    match key {
        Escape => *control_flow = ControlFlow::Exit,
        Left => game.do_action(NoteDirection::Left),
        Down => game.do_action(NoteDirection::Down),
        Up => game.do_action(NoteDirection::Up),
        Right => game.do_action(NoteDirection::Right),
        M => game.do_action(NoteDirection::Left),
        Comma => game.do_action(NoteDirection::Down),
        Period => game.do_action(NoteDirection::Up),
        Slash => game.do_action(NoteDirection::Right),
        Space => {
            do_toggle_game_state_debug(game);
        }
        _ => log::info!("Key: {:?}", key),
    }
}

fn clear(frame: &mut [u8]) {
    frame.fill(0);
}

struct Action {
    direction: NoteDirection,
    ts: u32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Default)]
pub struct Engine {
    settings: Settings,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl Engine {
    #[cfg_attr(
        target_arch = "wasm32",
        wasm_bindgen::prelude::wasm_bindgen(constructor)
    )]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn modify_settings(&mut self, partial_settings: SettingsMerge) -> bool {
        self.settings.merge(partial_settings);
        true
    }

    #[cfg_attr(
        target_arch = "wasm32",
        wasm_bindgen::prelude::wasm_bindgen(js_name = toJSON)
    )]
    pub fn to_json(&self) -> settings::Settings {
        self.settings
    }
}

fn get_pos_from_ms(ms: i64, end_position: f32, start_position: f32, time_on_screen: u32) -> f32 {
    end_position.lerp(start_position, ms as f32 / time_on_screen as f32)
}

#[allow(clippy::too_many_arguments)]
fn draw_notes(
    play: &Play<play::Active>,
    time_on_screen: u32,
    ms_chart_progress: u32,
    field: &Field,
    ms_note_render_offset: i8,
    offset: f32,
    frame: &mut [u8],
    noteskin: &noteskin::Definition,
    lane_gap: u8,
    screen_width: u32,
    screen_height: u32,
) {
    let end_position = field.judge_position;
    let view = play.view(
        u32::from(time_on_screen / 2),
        time_on_screen + u32::from(i8::unsigned_abs(ms_note_render_offset)),
    );
    for (&ms_when_note_at_receptor, note) in
        view.filter(|(_, note)| !play.judgements().contains_key(note))
    {
        // Calculate "time_on_screen" as from off-screen to receptor, and then continue on with the lerp. (lerp can fall off)
        // Rendering should carry on past the zero point but it should arrive at 0 at the receptor point rather than the beginning of the screen.

        let note_progress = (ms_when_note_at_receptor as f32 + f32::from(ms_note_render_offset))
            - ms_chart_progress as f32;
        let normalized = note_progress as f32 / time_on_screen as f32;
        let position = end_position.lerp(field.start_position, normalized.into());

        let lane_offset = lane_gap as f32;

        let lane_index = match note.direction {
            NoteDirection::Left => -1.5,
            NoteDirection::Down => -0.5,
            NoteDirection::Up => 0.5,
            NoteDirection::Right => 1.5,
        };
        let x = offset + (lane_offset * lane_index);
        let y = position as f32;
        sprites::blit(
            frame,
            screen_width,
            screen_height,
            x,
            y,
            &Direction(&note.direction),
            &noteskin.get_note(note.color),
        );
    }
}

fn draw_line(frame: &mut [u8], position: f32, screen_height: u32, screen_width: u32) {
    let color = [0x5e, 0x48, 0xe8, 0xff];
    let rounded: i16 = position.round() as i16;
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let y = (i / screen_width as usize) as i16;
        if rounded == y {
            pixel.copy_from_slice(&color);
        }
    }
}

fn draw_receptors(
    play: &Play<play::Active>,
    noteskin: &noteskin::Definition,
    frame: &mut [u8],
    offset: f32,
    receptor_y: f32,
    lane_gap: u8,
    screen_width: u32,
    screen_height: u32,
) {
    let receptor_skin = noteskin.get_note(NoteColor::Receptor);
    let lane_offset = f32::from(lane_gap);
    sprites::blit(
        frame,
        screen_width,
        screen_height,
        offset + (lane_offset * -1.5),
        receptor_y,
        &Direction(&NoteDirection::Left),
        &receptor_skin,
    );
    sprites::blit(
        frame,
        screen_width,
        screen_height,
        offset + (lane_offset * -0.5),
        receptor_y,
        &Direction(&NoteDirection::Down),
        &receptor_skin,
    );
    sprites::blit(
        frame,
        screen_width,
        screen_height,
        offset + (lane_offset * 0.5),
        receptor_y,
        &Direction(&NoteDirection::Up),
        &receptor_skin,
    );
    sprites::blit(
        frame,
        screen_width,
        screen_height,
        offset + (lane_offset * 1.5),
        receptor_y,
        &Direction(&NoteDirection::Right),
        &receptor_skin,
    );
}
