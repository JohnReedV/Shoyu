use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

mod menu;
mod systems;

use menu::MainMenuPlugin;
use systems::*;

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
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}
