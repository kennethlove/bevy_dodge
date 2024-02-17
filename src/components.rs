use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

#[derive(Component)]
pub struct ColorText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Bullet;
