mod components;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::keyboard::KeyCode,
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
    window::{PresentMode, WindowTheme},
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use components::*;
use rand_core::RngCore;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const ENEMY_COLOR: Color = Color::RED;
const FAST_ENEMY_COLOR: Color = Color::CRIMSON;
const SLOW_ENEMY_COLOR: Color = Color::MAROON;
const ENEMY_SPEED: f32 = 200.;
const FAST_SPEED: f32 = 150.;
const SLOW_SPEED: f32 = 50.;
const MAX_ENEMIES: usize = 50;

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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    #[default]
    Menu,
    Running,
    GameOver,
}

#[derive(Resource)]
struct MenuData {
    text_entity: Entity,
    button_entity: Entity,
}

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
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Running), spawn_player)
        .add_systems(OnExit(GameState::Running), cleanup_game)
        .add_systems(OnEnter(GameState::GameOver), game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .add_systems(
            FixedUpdate,
            (move_player, collide_player, move_enemy, spawn_enemy)
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(Update, menu.run_if(in_state(GameState::Menu)))
        .add_systems(Update, menu.run_if(in_state(GameState::GameOver)))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn setup_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    let text_entity = commands.spawn((
        TextBundle::from_section(
            "Dodge",
            TextStyle {
                font_size: 100.,
                color: Color::rgb(0.5, 0.0, 0.0),
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(WINDOW_PADDING),
            right: Val::Px(WINDOW_PADDING),
            ..default()
        }),
        ColorText,
    )).id();

    commands.insert_resource(MenuData { button_entity, text_entity });
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
    commands.entity(menu_data.text_entity).despawn_recursive();
}

fn menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Running);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn game_over(mut commands: Commands) {
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font_size: 40.,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    let text_entity = commands.spawn((
        TextBundle::from_section(
            "You Failed",
            TextStyle {
                font_size: 90.,
                color: Color::rgb(0.5, 0.0, 0.0),
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(WINDOW_PADDING),
            right: Val::Px(WINDOW_PADDING),
            ..default()
        }),
        ColorText,
    )).id();
    commands.insert_resource(MenuData { button_entity, text_entity });
}

fn cleanup_game_over(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
    commands.entity(menu_data.text_entity).despawn_recursive();
}

fn cleanup_game(mut commands: Commands, enemy_query: Query<(Entity, &Transform), With<Enemy>>) {
    for (entity, _) in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
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
        x = x.clamp(
            -WINDOW_SIZE.x / 2. + WINDOW_PADDING,
            WINDOW_SIZE.x / 2. - WINDOW_PADDING,
        );
        let y = WINDOW_SIZE.y / 2.;

        let speed = rng.next_u32() as f32 % ENEMY_SPEED;
        let color = {
            if speed >= FAST_SPEED {
                FAST_ENEMY_COLOR
            } else if speed <= SLOW_SPEED {
                SLOW_ENEMY_COLOR
            } else {
                ENEMY_COLOR
            }
        };

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(color.into()),
                transform: Transform::from_translation(Vec3::from((x, y, 0.))),
                ..default()
            },
            Enemy { speed },
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
    for mut transform in query.iter_mut() {
        transform.translation.x = transform.translation.x.clamp(
            -WINDOW_SIZE.x / 2. + WINDOW_PADDING,
            WINDOW_SIZE.x / 2. - WINDOW_PADDING,
        );
    }
}

fn move_enemy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (entity, mut transform, enemy) in query.iter_mut() {
        transform.translation.y -= enemy.speed * time.delta_seconds();

        if transform.translation.y < -(WINDOW_SIZE.y / 2.) {
            commands.entity(entity).despawn();
        }
    }
}

fn collide_player(
    mut next_state: ResMut<NextState<GameState>>,
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
                next_state.set(GameState::GameOver);
            }
        }
    }
}
