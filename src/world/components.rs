use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub pos: Position,
}

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TileType {
    Ground,
    Thud,
    Mountain,
    Water,
}

#[derive(Component)]
pub struct ChunkLine {}