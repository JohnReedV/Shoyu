use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub pos: Position,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub enum TileType {
    Ground,
    Water,
    Mountain,
}