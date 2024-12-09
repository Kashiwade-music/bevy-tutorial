//! Uses two windows to visualize a 3D model from different angles.

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::time::Stopwatch;
use bevy::window::{EnabledButtons, PrimaryWindow, WindowResolution};

mod config_controller;
mod cubic_bezier;
mod global_vars;
mod midi_loader;
mod plugin_midi_note_animater;
mod plugin_midi_note_text;
mod plugin_status_window;
mod util_color;

fn setup_scene(mut commands: Commands, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    // 設定の読み込み
    let config = config_controller::load_config().unwrap();
    let loaded_midi_return = midi_loader::load_midi(&config.main_config.midi_file_path);
    commands.insert_resource(global_vars::GlobalSettings {
        config: config.clone(),
        format: loaded_midi_return.format,
        ppm: loaded_midi_return.ppm,
        time_axis_vec: loaded_midi_return.time_axis_vec,
        midi_notes_vec: loaded_midi_return.midi_notes_vec,
    });

    commands.insert_resource(global_vars::GlobalMonitorValues {
        elapsed_time_from_start: Stopwatch::default(),
        current_time_axis: global_vars::TimeAxis::default(),
    });

    // ウィンドウの設定
    for mut window in windows.iter_mut() {
        window.title = "MIDI Visualizer".to_string();
        window.resizable = false;
        window.resolution = WindowResolution::new(
            config.main_config.window_width as f32,
            config.main_config.window_height as f32,
        );
        window.enabled_buttons = EnabledButtons {
            close: true,
            minimize: false,
            maximize: false,
        };
    }

    // カメラの設定
    let bg_color = util_color::hex_to_srgb(&config.theme[0].background_hex).unwrap();
    commands.spawn((
        Camera2d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(
                bg_color[0],
                bg_color[1],
                bg_color[2],
            )),
            ..default()
        },
        global_vars::MainWindowCamera,
        RenderLayers::layer(0),
    ));
}

fn toggle_play_or_stop(
    app_state: Res<State<global_vars::AppState>>,
    mut next_app_state: ResMut<NextState<global_vars::AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
    global_settings: Res<global_vars::GlobalSettings>,
    mut global_monitor_values: ResMut<global_vars::GlobalMonitorValues>,
) {
    let is_music_finished = global_monitor_values.elapsed_time_from_start.elapsed_secs()
        >= global_settings.time_axis_vec.last().unwrap().seconds_total;

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
            .find(|x| x.seconds_total <= elapsed_time);
        if let Some(time_axis) = time_axis {
            global_monitor_values.current_time_axis = *time_axis;
        }
    } else if app_state.get() == &global_vars::AppState::Stop {
        global_monitor_values.elapsed_time_from_start.reset();
        global_monitor_values.current_time_axis = global_settings.time_axis_vec[0];
    }
}

fn main() {
    App::new()
        // By default, a primary window gets spawned by `WindowPlugin`, contained in `DefaultPlugins`
        .add_plugins(DefaultPlugins)
        .add_plugins(plugin_status_window::StatusWindowPlugin)
        .add_plugins(plugin_midi_note_text::MidiNoteTextPlugin)
        .add_plugins(plugin_midi_note_animater::MidiNoteAnimatePlugin)
        .init_state::<global_vars::AppState>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            PreUpdate,
            (toggle_play_or_stop, update_monitor_values).chain(),
        )
        .run();
}
