use crate::components::*;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, PlayerCamera {}));
}
