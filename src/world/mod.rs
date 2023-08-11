use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod utils;

use crate::resources::*;
use resources::*;
use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ChunkLineRenderState>()
        .add_systems(OnEnter(GameState::Game), create_world)
            .add_systems(OnEnter(GameState::Menu), despawn_world)
            .add_systems(Update, toggle_chunk_outlines.run_if(in_state(GameState::Game)));
    }
}
