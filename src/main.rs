//! Uses two windows to visualize a 3D model from different angles.

use bevy::prelude::*;
use bevy::time::Stopwatch;

mod config_controller;
mod global_vars;
mod midi_loader;
mod plugin_midi_note_text;
mod plugin_status_window;

fn setup_scene(mut commands: Commands) {
    // 設定の読み込み
    let config = config_controller::load_config().unwrap();
    let loaded_midi_return = midi_loader::load_midi(&config.midi_file_path);
    commands.insert_resource(global_vars::GlobalSettings {
        midi_path: config.midi_file_path,
        format: loaded_midi_return.format,
        ppm: loaded_midi_return.ppm,
        time_axis_vec: loaded_midi_return.time_axis_vec,
        midi_notes_vec: loaded_midi_return.midi_notes_vec,
    });

    commands.insert_resource(global_vars::GlobalMonitorValues {
        elapsed_time_from_start: Stopwatch::default(),
        time_axis: global_vars::TimeAxis {
            ticks_index: 0,
            seconds: 0.0,
            measure: 0,
            beat: 1,
            tick: 0,
            tempo: 120.0,
            time_signature_numerator: 4,
            time_signature_denominator: 4,
            time_signature_midi_clocks_per_metronome_click: 24,
            time_signature_thirty_seconds_notes_per_quarter_note: 8,
        },
    });

    commands.spawn((Camera2d::default(), global_vars::MainWindowCamera));
}

fn toggle_play_or_stop(
    app_state: Res<State<global_vars::AppState>>,
    mut next_app_state: ResMut<NextState<global_vars::AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
    global_settings: Res<global_vars::GlobalSettings>,
    mut global_monitor_values: ResMut<global_vars::GlobalMonitorValues>,
) {
    let is_music_finished = global_monitor_values.elapsed_time_from_start.elapsed_secs()
        >= global_settings.time_axis_vec.last().unwrap().seconds;

    if keys.just_pressed(KeyCode::Space) || is_music_finished {
        match app_state.get() {
            global_vars::AppState::Stop => {
                next_app_state.set(global_vars::AppState::Playing);
                global_monitor_values.elapsed_time_from_start.reset();
            }
            global_vars::AppState::Playing => {
                next_app_state.set(global_vars::AppState::Stop);
                global_monitor_values.elapsed_time_from_start.reset();
            }
        }
    }
}

fn update_monitor_values(
    time: Res<Time>,
    mut global_monitor_values: ResMut<global_vars::GlobalMonitorValues>,
    global_settings: Res<global_vars::GlobalSettings>,
    app_state: Res<State<global_vars::AppState>>,
) {
    if app_state.get() == &global_vars::AppState::Playing {
        global_monitor_values
            .elapsed_time_from_start
            .tick(time.delta());
        let elapsed_time = global_monitor_values.elapsed_time_from_start.elapsed_secs();
        let time_axis = global_settings
            .time_axis_vec
            .iter()
            .rev()
            .find(|x| x.seconds <= elapsed_time);
        if let Some(time_axis) = time_axis {
            global_monitor_values.time_axis = *time_axis;
        }
    } else if app_state.get() == &global_vars::AppState::Stop {
        global_monitor_values.elapsed_time_from_start.reset();
        global_monitor_values.time_axis = global_settings.time_axis_vec[0];
    }
}

fn main() {
    App::new()
        // By default, a primary window gets spawned by `WindowPlugin`, contained in `DefaultPlugins`
        .add_plugins(DefaultPlugins)
        .add_plugins(plugin_status_window::StatusWindowPlugin)
        .add_plugins(plugin_midi_note_text::MidiNoteTextPlugin)
        .init_state::<global_vars::AppState>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            PreUpdate,
            (toggle_play_or_stop, update_monitor_values).chain(),
        )
        .run();
}
