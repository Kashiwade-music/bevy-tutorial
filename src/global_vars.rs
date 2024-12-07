use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalSettings {
    pub midi_path: String,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Init,
    Playing,
}
