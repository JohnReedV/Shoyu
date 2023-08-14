use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera {}

#[derive(Component)]
pub struct Player {
    pub jump: Jump
}

pub struct Jump {
    pub is_jumping: bool,
    pub jump_timer: f32,
    pub jump_y: f32,
    pub jump_cooldown_timer: f32,
}