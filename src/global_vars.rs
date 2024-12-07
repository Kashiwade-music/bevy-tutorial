use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Resource)]
pub struct GlobalSettings {
    // mainly constants
    pub midi_path: String,
    pub format: midly::Format,
    pub ppm: u16,
    pub time_axis_vec: Vec<TimeAxis>,
}

#[derive(Resource)]
pub struct GlobalMonitorValues {
    // mainly variables
    pub elapsed_time_from_start: Stopwatch,
    pub time_axis: TimeAxis,
}

#[derive(Clone, Copy, Debug)]
pub struct TimeAxis {
    pub ticks_index: u32,
    pub seconds: f32,
    pub measure: u32, // 小節数
    pub beat: u32,    // 拍
    pub tick: u32,    // tick
    pub tempo: f32,   // テンポ
    pub time_signature_numerator: u8,
    pub time_signature_denominator: u8,
    pub time_signature_midi_clocks_per_metronome_click: u8,
    pub time_signature_thirty_seconds_notes_per_quarter_note: u8,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Stop,
    Playing,
}
