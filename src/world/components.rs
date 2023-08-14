use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub pos: Position,
    pub structure: Structure
}
impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::Grass,
            pos: Position::default(),
            structure: Structure::default()
        }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Structure {
    Tree,
    #[default]
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TileType {
    Ground,
    Thud,
    Grass,
    Water,
}

#[derive(Component)]
pub struct ChunkLine {}

#[derive(Component)]
pub struct PalmTree {}