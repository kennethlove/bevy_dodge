use bevy::prelude::*;
use crate::components::ColorText;
use crate::constants::*;
use crate::GameState;

#[derive(Resource)]
pub struct MenuData {
    pub text_entity: Entity,
    pub button_entity: Entity,
}

pub fn setup_menu(mut commands: Commands) {
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
                        padding: UiRect {
                            left: Val::Px(20.),
                            right: Val::Px(20.),
                            top: Val::Px(10.),
                            bottom: Val::Px(10.),
                        },
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 20.,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    let text_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                padding: UiRect {
                    left: Val::Px(WINDOW_PADDING),
                    right: Val::Px(WINDOW_PADDING),
                    top: Val::Px(WINDOW_PADDING),
                    bottom: Val::Px(WINDOW_PADDING),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Dodge",
                    TextStyle {
                        font_size: 80.,
                        color: Color::rgb(0.5, 0.0, 0.0),
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center),

                ColorText,
            ));
        })
        .id();

    commands.insert_resource(MenuData {
        button_entity,
        text_entity,
    });
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
    commands.entity(menu_data.text_entity).despawn_recursive();
}

pub fn menu(
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
