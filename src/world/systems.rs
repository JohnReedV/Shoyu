use bevy::prelude::*;
use rand::prelude::*;

use crate::resources::*;
use crate::world::components::*;

const WORLD_SIZE: u32 = 1000;
const TILE_SIZE: f32 = 64.0;
const CHUNK_SIZE: u32 = 8;

pub fn create_world(commands: Commands, mut reader: EventReader<GameStart>) {
    if let Some(_game_start) = reader.iter().last() {
        let (world, chunk_biomes) = generate_world();
        render_world(commands, world, &chunk_biomes);
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

fn render_world(mut commands: Commands, world: Vec<Vec<Tile>>, _chunk_biomes: &Vec<Vec<TileType>>) {
    let mut rng = rand::thread_rng();

    for row in world {
        for tile in row {
            let color = match tile.tile_type {
                TileType::Ground => {
                    let shade: f32 = rng.gen_range(0.0..=1.0);
                    Color::rgb(0.0 * shade, 0.8 * shade, 0.2 * shade)
                }
                TileType::Water => {
                    let shade: f32 = rng.gen_range(0.0..=1.0);
                    Color::rgb(0.1 * shade, 0.2 * shade, 0.8 * shade)
                }
                TileType::Mountain => {
                    let shade: f32 = rng.gen_range(0.0..=1.0);
                    Color::rgb(0.6 * shade, 0.0 * shade, 0.1 * shade)
                }
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

fn generate_world() -> (Vec<Vec<Tile>>, Vec<Vec<TileType>>) {
    let mut rng = rand::thread_rng();
    let mut world = Vec::new();

    let mut chunk_biomes: Vec<Vec<TileType>> =
        vec![
            vec![TileType::Ground; WORLD_SIZE as usize / CHUNK_SIZE as usize];
            WORLD_SIZE as usize / CHUNK_SIZE as usize
        ];

    for chunk_x in 0..WORLD_SIZE / CHUNK_SIZE {
        for chunk_y in 0..WORLD_SIZE / CHUNK_SIZE {
            let tile_type = match rng.gen_range(0..3) {
                0 => TileType::Ground,
                1 => TileType::Water,
                _ => TileType::Mountain,
            };

            chunk_biomes[chunk_x as usize][chunk_y as usize] = tile_type.clone();

            for x in 0..CHUNK_SIZE {
                let mut row = Vec::new();
                for y in 0..CHUNK_SIZE {
                    let mut final_tile_type = tile_type.clone();

                    if x == 0 || x == CHUNK_SIZE - 1 || y == 0 || y == CHUNK_SIZE - 1 {
                        let dx = if x == 0 {
                            -1
                        } else if x == CHUNK_SIZE - 1 {
                            1
                        } else {
                            0
                        };
                        let dy = if y == 0 {
                            -1
                        } else if y == CHUNK_SIZE - 1 {
                            1
                        } else {
                            0
                        };
                        let neighbor_biome = get_neighbor_biome(
                            &chunk_biomes,
                            chunk_x as i32,
                            chunk_y as i32,
                            dx,
                            dy,
                        );
                        if neighbor_biome != tile_type {
                            final_tile_type = blend(tile_type.clone(), neighbor_biome);
                        }
                    }

                    row.push(Tile {
                        tile_type: final_tile_type,
                        pos: Position {
                            x: ((chunk_x * CHUNK_SIZE + x) as f32 - WORLD_SIZE as f32 / 2.0)
                                * TILE_SIZE,
                            y: ((chunk_y * CHUNK_SIZE + y) as f32 - WORLD_SIZE as f32 / 2.0)
                                * TILE_SIZE,
                        },
                    });
                }
                world.push(row);
            }
        }
    }

    (world, chunk_biomes)
}

fn get_neighbor_biome(
    chunk_biomes: &Vec<Vec<TileType>>,
    chunk_x: i32,
    chunk_y: i32,
    dx: i32,
    dy: i32,
) -> TileType {
    let neighbor_x = (chunk_x + dx) as usize;
    let neighbor_y = (chunk_y + dy) as usize;

    if neighbor_x >= WORLD_SIZE as usize / CHUNK_SIZE as usize
        || neighbor_y >= WORLD_SIZE as usize / CHUNK_SIZE as usize
    {
        return TileType::Ground;
    }

    chunk_biomes[neighbor_x][neighbor_y].clone()
}

fn blend(tile1: TileType, tile2: TileType) -> TileType {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        tile1
    } else {
        tile2
    }
}
