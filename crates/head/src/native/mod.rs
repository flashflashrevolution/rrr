use crate::head::Head;
use simple_logger::SimpleLogger;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

struct MyHandler {
    current_mouse_pos: Vector2<f32>,
    head: Head,
}

impl WindowHandler for MyHandler {
    fn on_mouse_move(&mut self, _h: &mut WindowHelper, pos: Vector2<f32>) {
        self.current_mouse_pos = pos;
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        graphics.draw_circle(self.current_mouse_pos, 50.0, Color::BLUE);
        self.head.tick();
        helper.request_redraw();
    }

    fn on_keyboard_char(&mut self, _helper: &mut WindowHelper<()>, _unicode_codepoint: char) {
        self.head.play_song();
    }
}

pub fn launch_native() {
    SimpleLogger::new().init().unwrap();

    let window = Window::new_centered("Speedy2D: Hello World", (640, 240)).unwrap();

    let handler = MyHandler {
        current_mouse_pos: Vector2::ZERO,
        head: Head::new(),
    };

    window.run_loop(handler);
}
