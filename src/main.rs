use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::keyboard::KeyCode,
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
    window::{PresentMode, WindowTheme},
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand_core::RngCore;

const ENEMY_SPEED: f32 = 100.;
const MAX_ENEMIES: usize = 100;
const PLAYER_SIZE: Vec3 = Vec3 {
    x: 5.,
    y: 5.,
    z: 1.,
};
const PLAYER_FOCUS_SPEED: f32 = 50.;
const PLAYER_SPEED: f32 = 150.;
const PLAYER_COLOR: Color = Color::rgb(100., 100., 100.);
const WINDOW_PADDING: f32 = 25.;
const WINDOW_SIZE: Vec2 = Vec2 { x: 300., y: 500. };

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    resizable: true,
                    resolution: WINDOW_SIZE.into(),
                    title: "Dodge".to_string(),
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, (setup, spawn_player))
        .add_systems(
            FixedUpdate,
            (move_player, collide_player, move_enemy, spawn_enemy),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn spawn_enemy(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Transform, With<Enemy>>,
) {
    if query.iter().len() < MAX_ENEMIES {
        let mut x = rng.next_u32() as f32 % WINDOW_SIZE.x;
        x = if rng.next_u32() % 2 == 0 { -x } else { x };
        let y = WINDOW_SIZE.y / 2.;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(255., 0., 0.))),
                transform: Transform::from_translation(Vec3::from((x, y, 1.))),
                ..default()
            },
            Enemy,
        ));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(PLAYER_SIZE.x).into()).into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            transform: Transform::from_translation(Vec3::from((
                0.,
                -(WINDOW_SIZE.y / 2.) + WINDOW_PADDING,
                1.,
            ))),
            ..default()
        },
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        PLAYER_FOCUS_SPEED
    } else {
        PLAYER_SPEED
    };
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        for mut transform in query.iter_mut() {
            transform.translation.x -= time.delta_seconds() * speed;
        }
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        for mut transform in query.iter_mut() {
            transform.translation.x += time.delta_seconds() * speed;
        }
    }
}

fn move_enemy(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<Enemy>>) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.y -= ENEMY_SPEED * 0.016;

        if transform.translation.y < -(WINDOW_SIZE.y / 2.) {
            commands.entity(entity).despawn();
        }
    }
}

fn collide_player(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (player_entity, player_transform) in query.iter_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let collision = collide(
                player_transform.translation, // pos a
                PLAYER_SIZE.truncate(),       // radius a
                enemy_transform.translation,  // pos b
                Vec2::from((7., 7.)),         // radius b
            );
            if collision.is_some() {
                commands.entity(player_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
