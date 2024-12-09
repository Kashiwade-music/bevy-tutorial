use bevy::prelude::*;
use bevy::time::Stopwatch;
use serde::{Deserialize, Serialize};

// ==================== From Config File ====================
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub main_config: MainConfig,
    pub feature_and_layout: FeatureLayoutRoot,
    pub theme: Vec<Theme>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainConfig {
    pub midi_file_path: String,
    pub wave_file_path: String,
    pub window_height: u32,
    pub window_width: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeatureLayoutRoot {
    pub piano_roll: FeatureLayoutChild,
    pub transport_panel: FeatureLayoutChild,
    pub note_list: FeatureLayoutChild,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeatureLayoutChild {
    pub enabled: bool,
    pub left_percent: f32,
    pub top_percent: f32,
    pub right_percent: f32,
    pub bottom_percent: f32,
}

#[derive(Debug, Clone, Component)]
pub struct RectangleFromFeatureLayoutChild {
    pub left_top_abs_pixel: (f32, f32),
    pub right_top_abs_pixel: (f32, f32),
    pub right_bottom_abs_pixel: (f32, f32),
    pub left_bottom_abs_pixel: (f32, f32),
    pub width_pixel: f32,
    pub height_pixel: f32,
}

impl FeatureLayoutChild {
    /// ピクセル座標系の矩形情報を計算するメソッド
    pub fn calculate_rect(
        &self,
        window_width: u32,
        window_height: u32,
    ) -> Option<RectangleFromFeatureLayoutChild> {
        if !self.enabled {
            return None; // 無効な場合はNoneを返す
        }

        // 最小の次元を基準とする
        let min_dimension = window_width.min(window_height) as f32;

        // パーセントからピクセル値を計算
        let left_pixel = (self.left_percent / 100.0) * min_dimension;
        let top_pixel = window_height as f32 - (self.top_percent / 100.0) * min_dimension;
        let right_pixel = window_width as f32 - (self.right_percent / 100.0) * min_dimension;
        let bottom_pixel = (self.bottom_percent / 100.0) * min_dimension;

        // 中心を原点とする座標系に変換
        let x_center = window_width as f32 / 2.0;
        let y_center = window_height as f32 / 2.0;

        let left = left_pixel - x_center;
        let top = top_pixel - y_center;
        let right = right_pixel - x_center;
        let bottom = bottom_pixel - y_center;

        // 幅と高さを計算
        let width = right - left;
        let height = top - bottom;

        Some(RectangleFromFeatureLayoutChild {
            left_top_abs_pixel: (left, top),
            right_top_abs_pixel: (right, top),
            right_bottom_abs_pixel: (right, bottom),
            left_bottom_abs_pixel: (left, bottom),
            width_pixel: width,
            height_pixel: height,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub background_hex: String,
    pub note_channel_base_hex: String,
    pub note_channel_target_hex: String,
    pub main_base_hex: String,
    pub accent_base_hex: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main_config: MainConfig {
                midi_file_path: "C:\\Windows\\Media\\onestop.mid".to_string(),
                wave_file_path: "C:\\Windows\\Media\\onestop.wav".to_string(),
                window_height: 1080,
                window_width: 1920,
            },
            feature_and_layout: FeatureLayoutRoot {
                piano_roll: FeatureLayoutChild {
                    enabled: true,
                    left_percent: 5.0,
                    top_percent: 5.0,
                    right_percent: 5.0,
                    bottom_percent: 20.0,
                },
                transport_panel: FeatureLayoutChild {
                    enabled: true,
                    left_percent: 5.0,
                    top_percent: 80.0,
                    right_percent: 50.0,
                    bottom_percent: 5.0,
                },
                note_list: FeatureLayoutChild {
                    enabled: true,
                    left_percent: 50.0,
                    top_percent: 80.0,
                    right_percent: 5.0,
                    bottom_percent: 5.0,
                },
            },
            theme: vec![Theme {
                background_hex: "#2e3440".to_string(),
                note_channel_base_hex: "#eceff4".to_string(),
                note_channel_target_hex: "#2e3440".to_string(),
                main_base_hex: "#eceff4".to_string(),
                accent_base_hex: "#81a1c1".to_string(),
            }],
        }
    }
}
// ==================== From Config File ====================

// ==================== Bevy Global Resource ====================
#[derive(Resource)]
pub struct GlobalSettings {
    // mainly constants
    pub config: Config,
    pub format: midly::Format,
    pub ppm: u16,
    pub time_axis_vec: Vec<TimeAxis>,
    pub midi_notes_vec: Vec<Vec<MidiNote>>,
}

#[derive(Resource)]
pub struct GlobalMonitorValues {
    // mainly variables
    pub elapsed_time_from_start: Stopwatch,
    pub current_time_axis: TimeAxis,
}
// ==================== Bevy Global Resource ====================

#[derive(Clone, Copy, Debug)]
pub struct TimeAxis {
    pub ticks_total: u32,
    pub seconds_total: f32,
    pub measure: u32,                // 小節数
    pub ticks_reset_by_measure: u32, // 小節が変わるときにリセットされるtick
    pub beat: u32,                   // 拍
    pub ticks_reset_by_beat: u32,    // 拍が変わるときにリセットされるtick
    pub measure_length_ticks: u32,   // このTimeAxis時点での小節の長さ
    pub beat_length_ticks: u32,      // このTimeAxis時点での拍の長さ
    pub tempo: f32,                  // テンポ
    pub time_signature_numerator: u8,
    pub time_signature_denominator: u8,
    pub time_signature_midi_clocks_per_metronome_click: u8,
    pub time_signature_thirty_seconds_notes_per_quarter_note: u8,
}

impl Default for TimeAxis {
    fn default() -> Self {
        Self {
            ticks_total: 0,
            seconds_total: 0.0,
            measure: 0,
            ticks_reset_by_measure: 0,
            beat: 1,
            ticks_reset_by_beat: 0,
            measure_length_ticks: 0,
            beat_length_ticks: 0,
            tempo: 120.0,
            time_signature_numerator: 4,
            time_signature_denominator: 4,
            time_signature_midi_clocks_per_metronome_click: 24,
            time_signature_thirty_seconds_notes_per_quarter_note: 8,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MidiNote {
    pub note_on_time_axis: TimeAxis,
    pub note_off_time_axis: Option<TimeAxis>,

    pub note_length_ticks: Option<u32>,

    pub key: u32,
    pub key_cdefgab: String,    // C, C#, D, D#, E, F, F#, G, G#, A, A#, B
    pub key_octave_yamaha: i32, // -2 ~ 8
    pub key_octave_general_midi: i32, // -1 ~ 9
    pub key_and_octave_yamaha: String, // C-2 ~ G8
    pub velocity: u32,
    pub channel: u32, // 0 ~ 15
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Stop,
    Playing,
}

#[derive(Component)]
pub struct MainWindowCamera;
