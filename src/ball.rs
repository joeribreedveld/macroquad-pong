use macroquad::prelude::*;

pub struct Ball {
    position: Vec2,
    velocity: Vec2,
    speed: f32,
    radius: f32,
}

impl Ball {
    pub fn new(position: Vec2, radius: f32, velocity: Vec2, speed: f32) -> Self {
        Self {
            position,
            radius,
            velocity,
            speed,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }

    pub fn update(&mut self, delta_time: f32, screen_width: f32, screen_height: f32) {
        let min: Vec2 = Vec2::new(self.radius, self.radius);
        let max: Vec2 = Vec2::new(screen_width - self.radius, screen_height - self.radius);

        self.position.x = clamp(
            self.position.x + self.velocity.x * delta_time * self.speed,
            min.x,
            max.x,
        );

        self.position.y = clamp(
            self.position.y + self.velocity.y * delta_time * self.speed,
            min.y,
            max.y,
        );

        self.reflect(min, max);
    }

    fn reflect(&mut self, min: Vec2, max: Vec2) {
        if self.position.x >= max.x {
            self.velocity.x *= -1.0;
        }

        if self.position.x <= min.x {
            self.velocity.x *= -1.0;
        }

        if self.position.y >= max.y {
            self.velocity.y *= -1.0;
        }

        if self.position.y <= min.y {
            self.velocity.y *= -1.0;
        }
    }
}
