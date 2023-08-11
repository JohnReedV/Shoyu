use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod utils;

use systems::*;
use resources::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ChunkLineRenderState>()
        .add_systems(Update, (create_world, despawn_world, toggle_chunk_outlines));
    }
}
