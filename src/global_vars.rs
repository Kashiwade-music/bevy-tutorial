use bevy::prelude::*;
use bevy::time::Stopwatch;

pub struct theme {
    pub background_hex: String,
    pub note_channel_1_hex: String,
    pub main_text_color: String,
    pub secondary_text_color: String,
}

#[derive(Resource)]
pub struct GlobalSettings {
    // mainly constants
    pub midi_path: String,
    pub format: midly::Format,
    pub ppm: u16,
    pub time_axis_vec: Vec<TimeAxis>,
    pub midi_notes_vec: Vec<Vec<MidiNote>>,
    pub window_height: u32,
    pub window_width: u32,
    pub theme: theme,
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
    pub tick: u32,    // tick reset by beat
    pub tick_reset_by_measure: u32,
    pub measure_length_ticks: u32,
    pub tempo: f32, // テンポ
    pub time_signature_numerator: u8,
    pub time_signature_denominator: u8,
    pub time_signature_midi_clocks_per_metronome_click: u8,
    pub time_signature_thirty_seconds_notes_per_quarter_note: u8,
}

#[derive(Clone, Debug)]
pub struct MidiNote {
    pub note_on_ticks: u32,
    pub note_on_seconds: Option<f32>,
    pub note_on_measure: Option<u32>, // 小節数
    pub note_on_beat: Option<u32>,    // 拍
    pub note_on_tick: Option<u32>,    // tick
    pub note_on_tick_reset_by_measure: Option<u32>,

    pub note_off_ticks: Option<u32>,
    pub note_off_seconds: Option<f32>,
    pub note_off_measure: Option<u32>, // 小節数
    pub note_off_beat: Option<u32>,    // 拍
    pub note_off_tick: Option<u32>,    // tick
    pub note_off_tick_reset_by_measure: Option<u32>,

    pub note_length_ticks: Option<u32>,

    pub measure_length_ticks: Option<u32>,

    pub key: u32,
    pub key_cdefgab: String,    // C, C#, D, D#, E, F, F#, G, G#, A, A#, B
    pub key_octave_yamaha: i32, // -2 ~ 8
    pub key_octave_general_midi: i32, // -1 ~ 9
    pub key_and_octave_yamaha: String, // C-2 ~ G8
    pub velocity: u32,
    pub channel: u32,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Stop,
    Playing,
}

#[derive(Component)]
pub struct MainWindowCamera;
