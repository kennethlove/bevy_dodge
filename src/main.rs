mod components;
mod constants;
mod menu;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::keyboard::KeyCode,
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
    window::{PresentMode, WindowTheme}
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use bevy_vox::VoxPlugin;
use components::*;
use constants::*;
use menu::*;
use rand_core::RngCore;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    #[default]
    Menu,
    Running,
    GameOver,
}

#[derive(Resource)]
struct Score {
    value: i32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .insert_resource(Score { value: 0 })
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
        .add_plugins(VoxPlugin::default())
        .add_state::<GameState>()
        .add_systems(Startup, setup_camera)
        // .add_systems(Startup, spawn_player)
        // .add_systems(FixedUpdate, move_player)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Running), (spawn_player, show_score))
        .add_systems(OnExit(GameState::Running), cleanup_game)
        .add_systems(OnEnter(GameState::GameOver), game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .add_systems(
            FixedUpdate,
            (move_player, collide_player, move_enemy, spawn_enemy)
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(Update, update_score)
        .add_systems(Update, menu.run_if(in_state(GameState::Menu)))
        .add_systems(Update, menu.run_if(in_state(GameState::GameOver)))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let translation = Vec3::ZERO;

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(translation),
            ..Default::default()
        },
        MainCamera,
    ));
}

fn show_score(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                format!("Score: {}", score.value),
                TextStyle {
                    font_size: 15.,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            ..default()
        },
        ScoreText,
    ));
}

fn update_score(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.value);
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

    let text_entity = commands
        .spawn((
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
        ))
        .id();
    commands.insert_resource(MenuData {
        button_entity,
        text_entity,
    });
}

fn cleanup_game_over(
    mut commands: Commands,
    menu_data: Res<MenuData>,
    mut score: ResMut<Score>,
) {
    commands.entity(menu_data.button_entity).despawn_recursive();
    commands.entity(menu_data.text_entity).despawn_recursive();
    score.value = 0;
}

fn cleanup_game(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    ship_query: Query<Entity, With<Ship>>
) {
    commands.entity(ship_query.single()).despawn();
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
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
            SpriteBundle {
                sprite: Sprite { custom_size: Some(Vec2::from((20., 20.))), ..default() },
                texture: asset_server.load("enemy.png"),
                transform: Transform::from_translation(Vec3::from((x, y, 0.))).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                ..default()
            },
            // MaterialMesh2dBundle {
            //     mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            //     material: materials.add(color.into()),
            //     ..default()
            // },
            Enemy { speed },
        ));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let ship_pos = Vec3::from((0., -(WINDOW_SIZE.y / 2.) + WINDOW_PADDING, 0.));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(SHIP_SIZE),
                ..default()
            },
            texture: asset_server.load("ship.png"),
            transform: Transform::from_translation(ship_pos),
            ..default()
        },
        Ship
    ));
    let player_pos = ship_pos + Vec3::from((0., -5., 1.));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(PLAYER_SIZE.x).into()).into(),
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            transform: Transform::from_translation(player_pos),
            ..default()
        },
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<&mut Transform, With<Ship>>,
    )>,
) {
    let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        PLAYER_FOCUS_SPEED
    } else {
        PLAYER_SPEED
    };


    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        for mut transform in query.p0().iter_mut() {
            transform.translation.x -= time.delta_seconds() * speed;
        }
        for mut transform in query.p1().iter_mut() {
            transform.translation.x -= time.delta_seconds() * speed;
        }
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        for mut transform in query.p0().iter_mut() {
            transform.translation.x += time.delta_seconds() * speed;
        }
        for mut transform in query.p1().iter_mut() {
            transform.translation.x += time.delta_seconds() * speed;
        }
    }

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        for mut transform in query.p0().iter_mut() {
            transform.translation.y += time.delta_seconds() * speed;
        }
        for mut transform in query.p1().iter_mut() {
            transform.translation.y += time.delta_seconds() * speed;
        }
    }

    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        for mut transform in query.p0().iter_mut() {
            transform.translation.y -= time.delta_seconds() * speed;
        }
        for mut transform in query.p1().iter_mut() {
            transform.translation.y -= time.delta_seconds() * speed;
        }
    }

    for mut transform in query.p0().iter_mut() {
        transform.translation.x = transform.translation.x.clamp(
            -WINDOW_SIZE.x / 2. + WINDOW_PADDING,
            WINDOW_SIZE.x / 2. - WINDOW_PADDING,
        );
        transform.translation.y = transform.translation.y.clamp(
            -WINDOW_SIZE.y / 2. + WINDOW_PADDING,
            WINDOW_SIZE.y / 2. - WINDOW_PADDING,
        );
    }
    for mut transform in query.p1().iter_mut() {
        transform.translation.x = transform.translation.x.clamp(
            -WINDOW_SIZE.x / 2. + WINDOW_PADDING,
            WINDOW_SIZE.x / 2. - WINDOW_PADDING,
        );
        transform.translation.y = transform.translation.y.clamp(
            -WINDOW_SIZE.y / 2. + WINDOW_PADDING,
            WINDOW_SIZE.y / 2. - WINDOW_PADDING,
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
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    graze_query: Query<&Transform, With<Ship>>,
    mut score: ResMut<Score>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (player_entity, player_transform) in player_query.iter() {
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
    for graze_transform in graze_query.iter() {
        for (_, enemy_transform) in enemy_query.iter() {
            let collision = collide(
                graze_transform.translation, // pos a
                SHIP_SIZE,                   // radius a
                enemy_transform.translation, // pos b
                Vec2::from((7., 7.)),        // radius b
            );
            if collision.is_some() {
                score.value += {
                    if keyboard_input.pressed(KeyCode::ShiftLeft) {
                        10
                    } else {
                        1
                    }
                };
            }
        }
    }
}
