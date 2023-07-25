use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_world, despawn_world));
    }
}
