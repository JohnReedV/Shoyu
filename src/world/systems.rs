use bevy::prelude::*;
use rand::prelude::*;
use std::cmp::*;
use std::iter::*;
use std::ops::RangeInclusive;

use crate::resources::*;
use crate::world::components::*;
use crate::world::resources::*;
use crate::world::utils::*;

pub const CHUNK_SIZE: i32 = 16;
pub const TILE_SIZE: f32 = 32.0;
pub const WORLD_SIZE: i32 = (CHUNK_SIZE * (TILE_SIZE as i32) + 15) / 16 * 16;

pub fn create_world(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<GameStart>,
) {
    if let Some(_game_start) = reader.iter().last() {
        render_world(commands, generate_world(), asset_server);
    }
}

pub fn despawn_world(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    world_query: Query<Entity, With<Tile>>,
    chunk_line_query: Query<Entity, With<ChunkLine>>,
) {
    if let Some(_game_over) = reader.iter().last() {
        for world_entity in world_query.iter() {
            commands.entity(world_entity).despawn_recursive();
        }

        if let Some(_game_over) = reader.iter().last() {
            despawn_chunk_outlines(chunk_line_query, commands);
        }
    }
}

pub fn toggle_chunk_outlines(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut chunk_line_render_state: ResMut<NextState<ChunkLineRenderState>>,
    chunk_line_render_state_const: Res<State<ChunkLineRenderState>>,
    chunk_line_query: Query<Entity, With<ChunkLine>>,
) {
    match *chunk_line_render_state_const.get() {
        ChunkLineRenderState::Off => {
            if keyboard_input.just_pressed(KeyCode::B) {
                for chunk_x in 0..WORLD_SIZE / CHUNK_SIZE {
                    for chunk_y in 0..WORLD_SIZE / CHUNK_SIZE {
                        render_chunk_outline(&mut commands, chunk_x, chunk_y);
                    }
                }
                chunk_line_render_state.set(ChunkLineRenderState::On)
            }
        }
        ChunkLineRenderState::On => {
            if keyboard_input.just_pressed(KeyCode::B) {
                despawn_chunk_outlines(chunk_line_query, commands);
                chunk_line_render_state.set(ChunkLineRenderState::Off)
            }
        }
    }
}

fn render_world(mut commands: Commands, world: Vec<Vec<Tile>>, assets: Res<AssetServer>) {
    for row in world {
        for tile in row {
            let thud_handle = assets.load("sprites/thud.png").into();
            let grass_handle = assets.load("sprites/grass.png").into();
            let ground_handle = assets.load("sprites/ground.png").into();
            let water_handle = assets.load("sprites/water.png").into();

            let texture_handle = match tile.tile_type {
                TileType::Ground => ground_handle,
                TileType::Thud => thud_handle,
                TileType::Mountain => grass_handle,
                TileType::Water => water_handle,
            };

            commands.spawn((
                SpriteBundle {
                    texture: texture_handle,
                    transform: Transform::from_xyz(tile.pos.x, tile.pos.y, 0.0),
                    ..default()
                },
                tile,
            ));
        }
    }
}

fn generate_world() -> Vec<Vec<Tile>> {
    const POTENTIAL_BIOMES_MULTI: f32 = 100.9;
    const RANDOM_BIOME_CHANCE: f32 = 0.0001;
    const WATER_CHANCE: f32 = 0.005;

    let mut rng = rand::thread_rng();
    let mut world: Vec<Vec<Tile>> = vec![vec![]; WORLD_SIZE as usize];

    let spawnable_biomes: Vec<TileType> = vec![TileType::Ground, TileType::Thud, TileType::Mountain];
    let all_biomes: Vec<TileType> = spawnable_biomes
        .iter()
        .cloned()
        .chain(once(TileType::Water))
        .collect();

    let mut chunk_biomes: Vec<Vec<TileType>> = vec![vec![]; WORLD_SIZE as usize / CHUNK_SIZE as usize];
    for chunk_row in chunk_biomes.iter_mut() {
        for _ in 0..WORLD_SIZE / CHUNK_SIZE {
            chunk_row.push(all_biomes[rng.gen_range(0..all_biomes.len())]);
        }
    }

    for chunk_x in 0..chunk_biomes.len() {
        for chunk_y in 0..chunk_biomes[chunk_x].len() {
            let mut potential_biomes: Vec<TileType> = vec![];

            let mut check_and_add_neighbor = |x_offset: isize, y_offset: isize| {
                let neighbor_x = (chunk_x as isize + x_offset) as usize;
                let neighbor_y = (chunk_y as isize + y_offset) as usize;

                if neighbor_x < chunk_biomes.len()
                    && neighbor_y < chunk_biomes[neighbor_x].len()
                    && spawnable_biomes.contains(&chunk_biomes[neighbor_x][neighbor_y])
                {
                    for _ in 0..=(rng.gen::<f32>() * POTENTIAL_BIOMES_MULTI) as usize {
                        potential_biomes.push(chunk_biomes[neighbor_x][neighbor_y].clone());
                    }
                }
            };

            check_and_add_neighbor(-1, 0);
            check_and_add_neighbor(0, -1);
            check_and_add_neighbor(1, 0);
            check_and_add_neighbor(0, 1);

            potential_biomes.shuffle(&mut rng);

            let tile_type = if rng.gen::<f32>() < WATER_CHANCE {
                TileType::Water
            } else {
                if rng.gen::<f32>() < RANDOM_BIOME_CHANCE {
                    *spawnable_biomes.choose(&mut rng).unwrap()
                } else {
                    if potential_biomes.len() > 0 {
                        *potential_biomes.choose(&mut rng).unwrap()
                    } else {
                        *spawnable_biomes.choose(&mut rng).unwrap()
                    }
                }
            };

            if chunk_x < chunk_biomes.len() && chunk_y < chunk_biomes[chunk_x].len() {
                chunk_biomes[chunk_x][chunk_y] = tile_type.clone();
            }

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let tile = Tile {
                        tile_type: tile_type.clone(),
                        pos: Position {
                            x: ((chunk_x as i32 * CHUNK_SIZE + x) as f32 - WORLD_SIZE as f32 / 2.0
                                + 0.5)
                                * TILE_SIZE,
                            y: ((chunk_y as i32 * CHUNK_SIZE + y) as f32 - WORLD_SIZE as f32 / 2.0
                                + 0.5)
                                * TILE_SIZE,
                        },
                    };
                    world[(chunk_y as i32 * CHUNK_SIZE + y) as usize].push(tile);
                }
            }
        }
    }
    blend_biomes(&mut world, &chunk_biomes);
    world.reverse(); //blend both chunk sides
    blend_biomes(&mut world, &chunk_biomes);
    return world;
}

fn blend_biomes(world: &mut Vec<Vec<Tile>>, chunk_biomes: &[Vec<TileType>]) {
    const BLEND_RANGE: RangeInclusive<i32> = 1..=5;

    let mut rng = rand::thread_rng();
    for chunk_x in 0..WORLD_SIZE / CHUNK_SIZE {
        for chunk_y in 0..WORLD_SIZE / CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let world_x: usize = (chunk_x * CHUNK_SIZE + x) as usize;
                    let world_y: usize = (chunk_y * CHUNK_SIZE + y) as usize;

                    let current_tile_type: TileType = world[world_y][world_x].tile_type.clone();

                    if current_tile_type == TileType::Water {
                        continue;
                    }
                    let mut possible_tile_types = vec![
                        TileType::Ground,
                        TileType::Thud,
                        TileType::Mountain,
                        TileType::Water,
                    ];
                    possible_tile_types.shuffle(&mut rng);

                    for &possible_tile_type in &possible_tile_types {
                        if possible_tile_type == current_tile_type {
                            continue;
                        }

                        let mut is_near_desired_chunk = false;

                        'outer: for dx in -1..=1 {
                            for dy in -1..=1 {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }

                                let check_chunk_x = (chunk_x as i32 + dx) as usize;
                                let check_chunk_y = (chunk_y as i32 + dy) as usize;

                                if check_chunk_x < chunk_biomes.len()
                                    && check_chunk_y < chunk_biomes[chunk_x as usize].len()
                                    && chunk_biomes[check_chunk_x][check_chunk_y] == possible_tile_type
                                {
                                    is_near_desired_chunk = true;
                                    break 'outer;
                                }
                            }
                        }

                        if is_near_desired_chunk {
                            let mut tile_distance = i32::MAX;

                            for dx in BLEND_RANGE.clone() {
                                let check_x = world_x.wrapping_add(dx as usize);
                                if check_x < world[world_y].len()
                                    && world[world_y][check_x].tile_type == possible_tile_type
                                {
                                    tile_distance = tile_distance.min(dx.abs());
                                }
                            }

                            for dy in BLEND_RANGE.clone() {
                                let check_y = world_y.wrapping_add(dy as usize);
                                if check_y < world.len()
                                    && world[check_y][world_x].tile_type == possible_tile_type
                                {
                                    tile_distance = tile_distance.min(dy.abs());
                                }
                            }

                            if BLEND_RANGE.contains(&tile_distance) {
                                let chance: f32 = 0.5 - tile_distance as f32 / 8.0;
                                if rng.gen::<f32>() < chance {
                                    world[world_y][world_x].tile_type = possible_tile_type;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
