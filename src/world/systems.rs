use bevy::prelude::*;
use rand::prelude::*;
use std::cmp::*;
use std::iter::*;
use std::ops::RangeInclusive;

use crate::resources::*;
use crate::world::components::*;
use crate::world::resources::*;
use crate::world::utils::*;

pub const CHUNK_SIZE: i32 = 20;
pub const TILE_SIZE: f32 = 32.0;
pub const WORLD_SIZE: i32 = (CHUNK_SIZE * (TILE_SIZE as i32) + 15) / 16 * 16;

pub fn create_world(mut reader: EventReader<GameStart>, mut the_world: ResMut<TheWorld>) {
    if let Some(_game_start) = reader.read().last() {
        let (world, chunk_biomes) = generate_world();
        the_world.new(world, chunk_biomes);
    }
}

pub fn despawn_world(
    mut commands: Commands,
    mut reader: EventReader<GameOver>,
    world_query: Query<Entity, With<Tile>>,
    chunk_line_query: Query<Entity, With<ChunkLine>>,
    palm_query: Query<Entity, With<PalmTree>>,
) {
    if let Some(_game_over) = reader.read().last() {
        for world_entity in world_query.iter() {
            commands.entity(world_entity).despawn_recursive();
        }
        for palm_entity in palm_query.iter() {
            commands.entity(palm_entity).despawn_recursive()
        }
        despawn_chunk_outlines(chunk_line_query, commands);
    }
}

pub fn toggle_chunk_outlines(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chunk_line_render_state: ResMut<NextState<ChunkLineRenderState>>,
    chunk_line_render_state_const: Res<State<ChunkLineRenderState>>,
    chunk_line_query: Query<Entity, With<ChunkLine>>,
) {
    match *chunk_line_render_state_const.get() {
        ChunkLineRenderState::Off => {
            if keyboard_input.just_pressed(KeyCode::KeyB) {
                for chunk_x in 0..WORLD_SIZE / CHUNK_SIZE {
                    for chunk_y in 0..WORLD_SIZE / CHUNK_SIZE {
                        render_chunk_outline(&mut commands, chunk_x, chunk_y);
                    }
                }
                chunk_line_render_state.set(ChunkLineRenderState::On)
            }
        }
        ChunkLineRenderState::On => {
            if keyboard_input.just_pressed(KeyCode::KeyB) {
                despawn_chunk_outlines(chunk_line_query, commands);
                chunk_line_render_state.set(ChunkLineRenderState::Off)
            }
        }
    }
}

pub fn render_world(
    mut commands: Commands,
    the_world: ResMut<TheWorld>,
    assets: Res<AssetServer>,
    mut reader: EventReader<GameStart>,
) {
    if let Some(_game_over) = reader.read().last() {
        let ground_material: Handle<Image> = assets.load("sprites/ground.png").into();
        let thud_material: Handle<Image> = assets.load("sprites/thud.png").into();
        let grass_material: Handle<Image> = assets.load("sprites/grass.png").into();
        let water_material: Handle<Image> = assets.load("sprites/water.png").into();

        for row in the_world.world.clone() {
            for tile in row {
                commands.spawn((
                    SpriteBundle {
                        texture: match tile.tile_type {
                            TileType::Ground => ground_material.clone(),
                            TileType::Thud => thud_material.clone(),
                            TileType::Grass => grass_material.clone(),
                            TileType::Water => water_material.clone(),
                        },
                        transform: Transform::from_xyz(tile.pos.x, tile.pos.y, 0.0),
                        ..default()
                    },
                    tile,
                ));
                spawn_structure(tile, &mut commands, &assets);
            }
        }
    }
}

fn spawn_structure(tile: Tile, commands: &mut Commands, assets: &Res<AssetServer>) {
    let double_palm_handle: Handle<Image> = assets.load("sprites/palmtree2.png").into();
    let single_palm_handle: Handle<Image> = assets.load("sprites/palmtree.png").into();
    let mut rng = rand::thread_rng();
    let structure = tile.structure;

    match structure {
        Structure::Tree => {
            let palm_handle: Handle<Image>;
            if rng.gen::<f32>() < 0.05 {
                palm_handle = single_palm_handle.clone();
            } else {
                palm_handle = double_palm_handle.clone()
            }
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(tile.pos.x, tile.pos.y, 0.2),
                    texture: palm_handle.clone(),
                    ..default()
                },
                PalmTree {},
            ));
        }
        Structure::None => {}
    }
}

fn generate_world() -> (Vec<Vec<Tile>>, Vec<Vec<TileType>>) {
    const POTENTIAL_BIOMES_MULTI: f32 = 100.9;
    const RANDOM_BIOME_CHANCE: f32 = 0.0001;
    const WATER_CHANCE: f32 = 0.005;

    let mut rng = rand::thread_rng();
    let mut world: Vec<Vec<Tile>> = vec![vec![]; WORLD_SIZE as usize];

    let spawnable_biomes: Vec<TileType> = vec![TileType::Ground, TileType::Thud, TileType::Grass];
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
                        structure: Structure::None,
                    };
                    world[(chunk_y as i32 * CHUNK_SIZE + y) as usize].push(tile);
                }
            }
        }
    }
    blend_biomes(&mut world, &chunk_biomes);
    fill_world(&mut world, &chunk_biomes);
    return (world, chunk_biomes);
}

fn fill_world(world: &mut Vec<Vec<Tile>>, chunk_biomes: &[Vec<TileType>]) {
    pub const SPAWN_TREE_CHANCE: f32 = 0.01;
    let mut rng = rand::thread_rng();

    for chunk_x in 0..WORLD_SIZE / CHUNK_SIZE {
        for chunk_y in 0..WORLD_SIZE / CHUNK_SIZE {
            if chunk_biomes[chunk_x as usize][chunk_y as usize] == TileType::Grass {
                for x in 0..CHUNK_SIZE {
                    for y in 0..CHUNK_SIZE {
                        if rng.gen::<f32>() < SPAWN_TREE_CHANCE {
                            let world_x: f32 = (chunk_x as i32 * CHUNK_SIZE + x) as f32;
                            let world_y: f32 = (chunk_y as i32 * CHUNK_SIZE + y) as f32;

                            world[world_y as usize][world_x as usize].structure = Structure::Tree;
                        }
                    }
                }
            }
        }
    }
}

fn blend_biomes(world: &mut Vec<Vec<Tile>>, chunk_biomes: &[Vec<TileType>]) {
    const BLEND_RANGE: RangeInclusive<i32> = -5..=5;
    const BLEND_CHANCE: f32 = 0.41;

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
                    let mut possible_tile_types =
                        vec![TileType::Ground, TileType::Thud, TileType::Grass, TileType::Water];
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
                                let chance: f32 = BLEND_CHANCE - tile_distance as f32 / 8.0;
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
