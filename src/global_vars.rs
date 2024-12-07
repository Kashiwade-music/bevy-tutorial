use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Resource)]
pub struct GlobalSettings {
    pub midi_path: String,
    pub elapsed_time_from_start: Stopwatch,
    pub format: midly::Format,
    pub timing: midly::Timing,
    pub tempo_change_events: Vec<TempoChangeEvent>,
    pub time_signature_change_events: Vec<TimeSignatureChangeEvent>,
}

pub struct TempoChangeEvent {
    pub tempo: f32, // how many beats per minute
    pub time_sec: f32,
    pub seconds_per_tick: f32, // how many seconds per tick after the tempo change
}

pub struct TimeSignatureChangeEvent {
    pub numerator: u8,
    pub denominator: u8,
    pub midi_clocks_per_metronome_click: u8,
    pub thirty_seconds_notes_per_quarter_note: u8,
    pub time_sec: f32,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Stop,
    Playing,
}
