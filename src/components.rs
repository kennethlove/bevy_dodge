use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

#[derive(Component)]
pub struct ColorText;
