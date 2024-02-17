mod components;
mod constants;
mod enemies;
mod menu;
mod player;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use components::*;
use constants::*;
use enemies::*;
use menu::*;
use player::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    #[default]
    Menu,
    Running,
    GameOver,
}

#[derive(Resource)]
pub struct Score {
    pub value: i32,
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
        .add_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Running), (spawn_player, show_score))
        .add_systems(OnExit(GameState::Running), cleanup_game)
        .add_systems(OnEnter(GameState::GameOver), game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .add_systems(
            FixedUpdate,
            (
                move_player,
                collide_player,
                fire_bullet,
                move_bullets,
                collide_bullets,
                move_enemy,
                spawn_enemy,
            )
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
    commands
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect {
                    left: Val::Px(WINDOW_PADDING),
                    top: Val::Px(WINDOW_PADDING),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
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
        });
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

fn cleanup_game_over(mut commands: Commands, menu_data: Res<MenuData>, mut score: ResMut<Score>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
    commands.entity(menu_data.text_entity).despawn_recursive();
    score.value = 0;
}

fn cleanup_game(
    mut commands: Commands,
    bullet_query: Query<Entity, With<Bullet>>,
    enemy_query: Query<Entity, With<Enemy>>,
    ship_query: Query<Entity, With<Ship>>,
) {
    commands.entity(ship_query.single()).despawn();
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
    for bullet in bullet_query.iter() {
        commands.entity(bullet).despawn();
    }
}
