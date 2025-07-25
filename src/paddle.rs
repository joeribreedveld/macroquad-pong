use macroquad::prelude::*;

use crate::constants::*;

pub enum PaddleSide {
    LEFT,
    RIGHT,
}

pub struct Paddle {
    position: Vec2,
    dir: i32,
    size: Vec2,
    side: PaddleSide,
}

impl Paddle {
    pub fn new(position: Vec2, size: Vec2, side: PaddleSide, dir: i32) -> Paddle {
        Paddle {
            position,
            size,
            dir,
            side,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            PADDLE_COLOR,
        );
    }

    pub fn update(&mut self, delta_time: f32) {
        let min: f32 = 0.0;
        let max: f32 = screen_height() - self.size.y;

        match &mut self.side {
            PaddleSide::LEFT => {
                update_left(self, delta_time, min, max);
            }

            PaddleSide::RIGHT => {
                update_right(self, delta_time, min, max);
            }
        }
    }
}

fn update_left(paddle: &mut Paddle, delta_time: f32, min: f32, max: f32) {
    paddle.position += paddle.dir as f32 * PADDLE_SPEED * delta_time;
}

fn update_right(paddle: &mut Paddle, delta_time: f32, min: f32, max: f32) {
    paddle.position.y = clamp(
        paddle.position.y + PADDLE_SPEED * delta_time * paddle.dir as f32,
        min,
        max,
    );

    if paddle.position.y == min || paddle.position.y == max {
        paddle.dir *= -1;
    }
}
