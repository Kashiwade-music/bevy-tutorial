//! Uses two windows to visualize a 3D model from different angles.

use bevy::prelude::*;
use bevy::time::Stopwatch;

mod config_controller;
mod global_vars;
mod midi_loader;
mod status_window;

fn setup_scene(mut commands: Commands) {
    // 設定の読み込み
    let config = config_controller::load_config().unwrap();
    let loaded_midi_return = midi_loader::load_midi(&config.midi_file_path, &mut commands);
    commands.insert_resource(global_vars::GlobalSettings {
        midi_path: config.midi_file_path,
        elapsed_time_from_start: Stopwatch::default(),
        format: loaded_midi_return.format,
        timing: loaded_midi_return.timing,
        tempo_change_events: loaded_midi_return.tempo_change_events,
        time_signature_change_events: loaded_midi_return.time_signature_change_events,
    });

    let first_window_camera = commands.spawn((Camera2d::default(),)).id();
    let node = Node {
        position_type: PositionType::Absolute,
        top: Val::Px(12.0),
        left: Val::Px(12.0),
        ..default()
    };
    commands.spawn((
        Text::new("First window"),
        node.clone(),
        // Since we are using multiple cameras, we need to specify which camera UI should be rendered to
        TargetCamera(first_window_camera),
    ));
}

fn toggle_play_or_stop(
    app_state: Res<State<global_vars::AppState>>,
    mut next_app_state: ResMut<NextState<global_vars::AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut global_settings: ResMut<global_vars::GlobalSettings>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match app_state.get() {
            global_vars::AppState::Stop => {
                global_settings.elapsed_time_from_start.reset();
                next_app_state.set(global_vars::AppState::Playing);
            }
            global_vars::AppState::Playing => {
                global_settings.elapsed_time_from_start.reset();
                next_app_state.set(global_vars::AppState::Stop);
            }
        }
    }
}

fn update_elapsed_time_from_start(
    time: Res<Time>,
    mut global_settings: ResMut<global_vars::GlobalSettings>,
) {
    global_settings.elapsed_time_from_start.tick(time.delta());
}

fn main() {
    App::new()
        // By default, a primary window gets spawned by `WindowPlugin`, contained in `DefaultPlugins`
        .add_plugins(DefaultPlugins)
        .add_plugins(status_window::StatusWindowPlugin)
        .init_state::<global_vars::AppState>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, toggle_play_or_stop)
        .add_systems(
            Update,
            update_elapsed_time_from_start.run_if(in_state(global_vars::AppState::Playing)),
        )
        .run();
}
