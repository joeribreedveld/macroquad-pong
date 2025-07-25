pub mod constants;

mod ball;
mod paddle;

use macroquad::prelude::*;

use crate::{ball::*, constants::*, paddle::*};

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_width: f32 = screen_width();
    let screen_height: f32 = screen_height();

    let screen_center: Vec2 = Vec2::new(screen_width / 2.0, screen_height / 2.0);

    let mut paddle_left: Paddle = Paddle::new(
        Vec2::new(PADDLE_SIDE_OFFSET, screen_center.y),
        0,
        PADDLE_SPEED.to_owned(),
        Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
        PaddleSide::LEFT,
        WHITE,
    );

    let mut paddle_right: Paddle = Paddle::new(
        Vec2::new(
            screen_width - PADDLE_SIDE_OFFSET - PADDLE_WIDTH,
            screen_center.y,
        ),
        1,
        PADDLE_SPEED.to_owned(),
        Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
        PaddleSide::RIGHT,
        WHITE,
    );

    let mut ball: Ball = Ball::new(
        screen_center,
        BALL_RADIUS,
        Vec2::new(1.0, 1.0).normalize(),
        BALL_SPEED,
    );

    let delta_time: f32 = get_frame_time();

    loop {
        clear_background(BLACK);

        paddle_left.update(delta_time, screen_height);
        paddle_right.update(delta_time, screen_height);

        ball.update(delta_time, screen_width, screen_height);

        paddle_left.draw();
        paddle_right.draw();

        ball.draw();

        next_frame().await;
    }
}
