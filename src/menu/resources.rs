use bevy::prelude::*;

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
