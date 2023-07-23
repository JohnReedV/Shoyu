use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Paused,
    Game,
}

#[derive(Resource)]
pub struct FixMenuTimer {
    pub timer: Timer,
}
impl Default for FixMenuTimer {
    fn default() -> FixMenuTimer {
        FixMenuTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}
