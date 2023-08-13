use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub pos: Position,
    pub structure: Structure
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, PartialEq, Copy)]
pub enum Structure {
    Tree,
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