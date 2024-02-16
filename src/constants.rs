use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const ENEMY_COLOR: Color = Color::RED;
pub const FAST_ENEMY_COLOR: Color = Color::CRIMSON;
pub const SLOW_ENEMY_COLOR: Color = Color::MAROON;
pub const ENEMY_SPEED: f32 = 200.;
pub const FAST_SPEED: f32 = 150.;
pub const SLOW_SPEED: f32 = 50.;
pub const MAX_ENEMIES: usize = 50;

pub const PLAYER_SIZE: Vec3 = Vec3 {
    x: 5.,
    y: 5.,
    z: 1.,
};
pub const PLAYER_FOCUS_SPEED: f32 = 50.;
pub const PLAYER_SPEED: f32 = 150.;
pub const PLAYER_COLOR: Color = Color::GREEN;

pub const SHIP_SIZE: Vec2 = Vec2 { x: 40., y: 40. };

pub const WINDOW_PADDING: f32 = 25.;
pub const WINDOW_SIZE: Vec2 = Vec2 { x: 300., y: 500. };
