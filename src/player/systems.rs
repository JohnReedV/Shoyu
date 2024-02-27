use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub const PLAYER_SPEED: i32 = 1000;
const JUMP_SCALE: f32 = 1.2;
const JUMP_DURATION: f32 = 0.5;
const JUMP_PEAK_HEIGHT: f32 = 5.0 * 32.0;
const JUMP_HALF_DURATION: f32 = JUMP_DURATION / 2.0;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<GameStart>,
) {
    if let Some(_game_start) = reader.read().last() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                texture: asset_server.load("sprites/player.png"),
                ..default()
            },
            Player {
                jump: Jump {
                is_jumping: false,
                jump_timer: 0.0,
                jump_y: 0.0,
                jump_cooldown_timer: 0.0,
                }
            },
        ));
    }
}

pub fn despawn_player(
    mut reader: EventReader<GameOver>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    if let Some(_game_over) = reader.read().last() {
        for player_entity in player_query.iter() {
            commands.entity(player_entity).despawn_recursive();
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let move_distance = direction * PLAYER_SPEED as f32 * time.delta_seconds();
        transform.translation += move_distance;

        if keyboard_input.just_pressed(KeyCode::Space) && !player.jump.is_jumping && player.jump.jump_cooldown_timer <= 0.0
            || keyboard_input.pressed(KeyCode::Space) && !player.jump.is_jumping && player.jump.jump_cooldown_timer <= 0.0
        {
            transform.translation.z = 2.1;
            player.jump.is_jumping = true;
            transform.scale *= Vec3::splat(JUMP_SCALE);
            player.jump.jump_y = transform.translation.y;
            player.jump.jump_cooldown_timer = 0.14;
        }

        if player.jump.is_jumping {
            player.jump.jump_timer += time.delta_seconds();

            if player.jump.jump_timer <= JUMP_HALF_DURATION {
                transform.translation.y = player.jump.jump_y
                    + (4.0 * JUMP_PEAK_HEIGHT / (JUMP_DURATION * JUMP_DURATION))
                        * player.jump.jump_timer
                        * player.jump.jump_timer;
            } else {
                transform.translation.y = player.jump.jump_y
                    + (4.0 * JUMP_PEAK_HEIGHT / (JUMP_DURATION * JUMP_DURATION))
                        * (player.jump.jump_timer - JUMP_DURATION)
                        * (player.jump.jump_timer - JUMP_DURATION);
            }

            if player.jump.jump_timer >= JUMP_DURATION {
                transform.translation.z = 1.0;
                player.jump.is_jumping = false;
                transform.scale /= Vec3::splat(JUMP_SCALE);
                player.jump.jump_timer = 0.0;
                transform.translation.y = player.jump.jump_y;
            }
        } else {
            if player.jump.jump_cooldown_timer > 0.0 {
                player.jump.jump_cooldown_timer -= time.delta_seconds();
            }
        }

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = transform.translation.x;
            camera_transform.translation.y = transform.translation.y;
        }
    }
}
