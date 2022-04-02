use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "macroquad".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

fn draw_primitives() {
    draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
    draw_rectangle(-0.1, 0.1, 0.2, 0.2, GREEN);
    draw_circle(0., 0., 0.1, YELLOW);
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(RED);

        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });

        draw_primitives();

        next_frame().await
    }
}
