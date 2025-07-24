use macroquad::prelude::*;

const PADDLE_WIDTH: f32 = 24.0;
const PADDLE_HEIGHT: f32 = 128.0;
const PADDLE_COLOR: Color = WHITE;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_rectangle(
            32.0,
            screen_height() / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            WHITE,
        );

        draw_rectangle(
            screen_width() - 32.0,
            screen_height() / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            PADDLE_COLOR,
        );

        next_frame().await;
    }
}
