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

    pub fn update(&mut self, dt: f32) {
        let min: f32 = 0.0;
        let max: f32 = screen_height() - self.size.y;

        match &mut self.side {
            PaddleSide::LEFT => {
                self.position += self.dir as f32 * PADDLE_SPEED * dt;
            }

            PaddleSide::RIGHT => {
                self.position.y = clamp(
                    self.position.y + PADDLE_SPEED * dt * self.dir as f32,
                    min,
                    max,
                );

                if self.position.y == min || self.position.y == max {
                    self.dir *= -1;
                }
            }
        }
    }
}
