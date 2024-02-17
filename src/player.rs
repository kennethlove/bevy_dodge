use crate::components::*;
use crate::constants::*;
use crate::{GameState, Score};
use bevy::{
    input::keyboard::KeyCode, prelude::*, sprite::collide_aabb::collide,
    sprite::MaterialMesh2dBundle,
};

pub fn spawn_player(
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
        Ship,
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

pub fn move_player(
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

pub fn collide_player(
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

pub fn fire_bullet(
    mut commands: Commands,
    ship: Query<&Transform, With<Ship>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for transform in ship.iter() {
            let bullet_pos = transform.translation + Vec3::from((0., 20., 0.));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(bullet_pos),
                    ..default()
                },
                Bullet,
            ));
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.y += time.delta_seconds() * 300.;
        if transform.translation.y > WINDOW_SIZE.y / 2. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn collide_bullets(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let collision = collide(
                bullet_transform.translation, // pos a
                Vec2::from((2., 2.)),         // radius a
                enemy_transform.translation,  // pos b
                Vec2::from((20., 20.)),       // radius b
            );
            if collision.is_some() {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                score.value += 10;
            }
        }
    }
}
