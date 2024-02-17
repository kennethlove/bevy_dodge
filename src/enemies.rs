use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand_core::RngCore;

pub fn spawn_enemy(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
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
        let y = WINDOW_SIZE.y / 2. - 20.;

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
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::from((20., 20.))),
                    ..default()
                },
                texture: asset_server.load("enemy.png"),
                transform: Transform::from_translation(Vec3::from((x, y, 0.)))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                ..default()
            },
            Enemy { speed },
        ));
    }
}

pub fn move_enemy(
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
