use macroquad::prelude::*;

pub enum PaddleSide {
    LEFT,
    RIGHT,
}

pub struct Paddle {
    position: Vec2,
    dir: i32,
    speed: f32,
    size: Vec2,
    side: PaddleSide,
    color: Color,
}

impl Paddle {
    pub fn new(
        position: Vec2,
        dir: i32,
        speed: f32,
        size: Vec2,
        side: PaddleSide,
        color: Color,
    ) -> Self {
        Self {
            position,
            dir,
            speed,
            size,
            side,
            color,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }

    pub fn update(&mut self, delta_time: f32, screen_height: f32) {
        let min: f32 = 0.0;
        let max: f32 = screen_height - self.size.y;

        match self.side {
            PaddleSide::LEFT => {
                self.update_left(delta_time, min, max);
            }

            PaddleSide::RIGHT => {
                self.update_right(delta_time, min, max);
            }
        }
    }

    fn update_left(&mut self, delta_time: f32, min: f32, max: f32) {
        if is_key_down(KeyCode::W) {
            self.dir = -1;
        }

        if is_key_down(KeyCode::S) {
            self.dir = 1;
        }

        self.position.y = clamp(
            self.position.y + self.dir as f32 * self.speed * delta_time,
            min,
            max,
        );

        self.dir = 0;
    }

    fn update_right(&mut self, delta_time: f32, min: f32, max: f32) {
        self.position.y = clamp(
            self.position.y + self.speed * delta_time * self.dir as f32,
            min,
            max,
        );

        if self.position.y == min || self.position.y == max {
            self.dir *= -1;
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }
}
