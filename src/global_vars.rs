use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Resource)]
pub struct GlobalSettings {
    pub midi_path: String,
    pub elapsed_time_from_start: Stopwatch,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Init,
    Playing,
}
