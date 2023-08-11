use bevy::prelude::*;
use crate::world::systems::*;

pub fn render_chunk_outline(commands: &mut Commands, chunk_x: i32, chunk_y: i32) {
    // Define the positions of the chunk's corners
    let top_left = Vec2::new(
        (chunk_x as f32 * TILE_SIZE * CHUNK_SIZE as f32) - (WORLD_SIZE as f32 * TILE_SIZE / 2.0),
        (chunk_y as f32 * TILE_SIZE * CHUNK_SIZE as f32) + (CHUNK_SIZE as f32 * TILE_SIZE)
            - (WORLD_SIZE as f32 * TILE_SIZE / 2.0),
    );
    let bottom_right = Vec2::new(
        top_left.x + TILE_SIZE * CHUNK_SIZE as f32,
        top_left.y - TILE_SIZE * CHUNK_SIZE as f32,
    );

    // Render the top line
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(CHUNK_SIZE as f32 * TILE_SIZE, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(
            top_left.x + CHUNK_SIZE as f32 * TILE_SIZE / 2.0,
            top_left.y,
            1.0,
        ),
        ..default()
    },));

    // Render the bottom line
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(CHUNK_SIZE as f32 * TILE_SIZE, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(
            bottom_right.x - CHUNK_SIZE as f32 * TILE_SIZE / 2.0,
            bottom_right.y,
            1.0,
        ),
        ..default()
    },));

    // Render the left line
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(2.0, CHUNK_SIZE as f32 * TILE_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(
            top_left.x,
            top_left.y - CHUNK_SIZE as f32 * TILE_SIZE / 2.0,
            1.0,
        ),
        ..default()
    },));

    // Render the right line
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(2.0, CHUNK_SIZE as f32 * TILE_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(
            bottom_right.x,
            bottom_right.y + CHUNK_SIZE as f32 * TILE_SIZE / 2.0,
            1.0,
        ),
        ..default()
    },));
}


// fn beb(world: &mut Vec<Vec<Tile>>, chunk_biomes: &[Vec<TileType>]) {
//     for chunk_x in 0..WORLD_SIZE / CHUNK_SIZE {
//         for chunk_y in 0..WORLD_SIZE / CHUNK_SIZE {
//             for x in 0..CHUNK_SIZE {
//                 for y in 0..CHUNK_SIZE {
//                     let world_x: usize = (chunk_x * CHUNK_SIZE + x) as usize;
//                     let world_y: usize = (chunk_y * CHUNK_SIZE + y) as usize;

//                     if world_y as usize >= world.len()
//                         || world_x as usize >= world[world_y as usize].len()
//                     {
//                         continue;
//                     }
//                     let final_tile_type =
//                         world[world_y as usize][world_x as usize].tile_type.clone();
//                 }
//             }
//         }
//     }
// }
