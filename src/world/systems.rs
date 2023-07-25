use bevy::prelude::*;
use rand::prelude::*;

use crate::resources::*;
use crate::world::components::*;

const WORLD_SIZE: u32 = 100;
const TILE_SIZE: f32 = 16.0;

pub fn create_world(commands: Commands, mut reader: EventReader<GameStart>) {
    if let Some(_game_start) = reader.iter().last() {
        render_world(commands, generate_world());
    }
}

pub fn despawn_world(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    world_query: Query<Entity, With<Tile>>,
) {
    if let Some(_game_over) = reader.iter().last() {
        for world_entity in world_query.iter() {
            commands.entity(world_entity).despawn_recursive();
        }
    }
}

fn generate_world() -> Vec<Vec<Tile>> {
    let mut rng = rand::thread_rng();
    let mut world = Vec::new();

    for x in 0..WORLD_SIZE {
        let mut row = Vec::new();
        for y in 0..WORLD_SIZE {
            let tile_type = match rng.gen_range(0..3) {
                0 => TileType::Ground,
                1 => TileType::Water,
                _ => TileType::Mountain,
            };
            row.push(Tile {
                tile_type,
                pos: Position {
                    x: (x as f32 - WORLD_SIZE as f32 / 2.0) * TILE_SIZE,
                    y: (y as f32 - WORLD_SIZE as f32 / 2.0) * TILE_SIZE,
                },
            });
        }
        world.push(row);
    }
    world
}

fn render_world(mut commands: Commands, world: Vec<Vec<Tile>>) {
    for row in world {
        for tile in row {
            let color = match tile.tile_type {
                TileType::Ground => Color::rgb(0.4, 0.2, 0.1),
                TileType::Water => Color::rgb(0.1, 0.2, 0.8),
                TileType::Mountain => Color::rgb(0.5, 0.5, 0.5),
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(tile.pos.x, tile.pos.y, 0.0),
                    ..default()
                },
                tile,
            ));
        }
    }
}
