use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

mod menu;
mod player;
mod resources;
mod systems;
mod world;
mod components;

use menu::MainMenuPlugin;
use player::PlayerPlugin;
use resources::*;
use systems::*;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Shoyu".into(),
                    resolution: (1920., 1080.).into(),
                    mode: WindowMode::Fullscreen,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
            MainMenuPlugin,
            WorldPlugin,
            PlayerPlugin,
        ))
        .add_event::<GameStart>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .run();
}
