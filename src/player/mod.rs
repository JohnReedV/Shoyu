use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use systems::*;
//use resources::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_player, despawn_player, player_movement));
    }
}
