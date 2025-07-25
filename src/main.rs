pub mod constants;
mod paddle;

use macroquad::prelude::*;

use crate::{constants::*, paddle::*};

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
    let screen_height_middle: f32 = screen_height() / 2.0;

    let mut paddle_left = Paddle::new(
        Vec2 {
            x: PADDLE_SIDE_OFFSET,
            y: screen_height_middle,
        },
        Vec2 {
            x: PADDLE_WIDTH,
            y: PADDLE_HEIGHT,
        },
        PaddleSide::LEFT,
        0,
    );

    let mut paddle_right = Paddle::new(
        Vec2 {
            x: screen_width() - PADDLE_SIDE_OFFSET - PADDLE_WIDTH,
            y: screen_height_middle,
        },
        Vec2 {
            x: PADDLE_WIDTH,
            y: PADDLE_HEIGHT,
        },
        PaddleSide::RIGHT,
        1,
    );

    let delta_time: f32 = get_frame_time();

    loop {
        clear_background(BLACK);

        paddle_left.update(delta_time);
        paddle_right.update(delta_time);

        paddle_left.draw();
        paddle_right.draw();

        next_frame().await;
    }
}
