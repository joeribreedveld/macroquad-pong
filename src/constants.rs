use macroquad::prelude::*;

pub const WINDOW_TITLE: &str = "Pong";
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;

pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32 = 128.0;
pub const PADDLE_COLOR: Color = WHITE;
pub const PADDLE_SIDE_OFFSET: f32 = 32.0;
pub const PADDLE_SPEED: f32 = 200.0;

pub const BALL_RADIUS: f32 = 14.0;
pub const BALL_SPEED: f32 = 250.0;
