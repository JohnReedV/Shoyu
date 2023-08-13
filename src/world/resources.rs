use bevy::prelude::*;
use crate::world::components::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ChunkLineRenderState {
    #[default]
    Off,
    On,
}

#[derive(Resource, Clone)]
pub struct TheWorld {
    pub world: Vec<Vec<Tile>>,
    pub chunk_biomes: Vec<Vec<TileType>>
}

impl Default for TheWorld {
    fn default() -> Self {
        TheWorld {
            world: Vec::new(),
            chunk_biomes: Vec::new(),
        }
    }
}

impl TheWorld {
    pub fn new(&mut self, world: Vec<Vec<Tile>>, chunk_biomes: Vec<Vec<TileType>>) {
        self.world = world;
        self.chunk_biomes = chunk_biomes;
    }
}