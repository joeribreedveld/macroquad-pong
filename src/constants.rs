use macroquad::prelude::*;

pub const WINDOW_TITLE: &str = "Pong";
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;

pub const PADDLE_WIDTH: f32 = 24.0;
pub const PADDLE_HEIGHT: f32 = 128.0;
pub const PADDLE_COLOR: Color = WHITE;
pub const PADDLE_SIDE_OFFSET: f32 = 32.0;
pub const PADDLE_SPEED: f32 = 100.0;

pub const BALL_RADIUS: f32 = 16.0;
pub const BALL_SPEED: f32 = 150.0;
