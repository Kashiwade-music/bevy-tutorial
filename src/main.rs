//! Uses two windows to visualize a 3D model from different angles.

use bevy::prelude::*;
use bevy::time::Stopwatch;

mod config_controller;
mod global_vars;
mod midi_loader;
mod status_window;

#[derive(Component)]
struct MidiNoteCh1Text;

fn setup_scene(mut commands: Commands) {
    // 設定の読み込み
    let config = config_controller::load_config().unwrap();
    let loaded_midi_return = midi_loader::load_midi(&config.midi_file_path, &mut commands);
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

    let first_window_camera = commands.spawn((Camera2d::default(),)).id();
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("First window"),
                // Since we are using multiple cameras, we need to specify which camera UI should be rendered to
                TargetCamera(first_window_camera),
            ));
            parent
                .spawn(Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(Text::new("Notes: "));
                    parent.spawn((Text::new(""), MidiNoteCh1Text));
                });
        });
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

fn update_midinote_ch1_text(
    global_monitor_values: Res<global_vars::GlobalMonitorValues>,
    global_settings: Res<global_vars::GlobalSettings>,
    mut query: Query<&mut Text, With<MidiNoteCh1Text>>,
) {
    let time_axis = global_monitor_values.time_axis;

    for mut text in &mut query {
        text.clear();
        for (i, midi_notes) in global_settings.midi_notes_vec.iter().enumerate() {
            let current_note_on_notes_vec = midi_notes
                .iter()
                .filter(|x| {
                    x.note_on_ticks <= time_axis.ticks_index
                        && x.note_off_ticks.unwrap() >= time_axis.ticks_index
                })
                .collect::<Vec<&global_vars::MidiNote>>();
            let mut text_str = String::new();
            text_str.push_str(&format!("ch{}: ", i + 1));
            for note in current_note_on_notes_vec {
                text_str.push_str(&format!(
                    "(Note: {}, Velocity: {}) ",
                    note.key_and_octave_yamaha, note.velocity
                ));
            }
            text_str.push_str("\n");
            text.push_str(text_str.as_str());
        }
    }
}

fn main() {
    App::new()
        // By default, a primary window gets spawned by `WindowPlugin`, contained in `DefaultPlugins`
        .add_plugins(DefaultPlugins)
        .add_plugins(status_window::StatusWindowPlugin)
        .init_state::<global_vars::AppState>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                toggle_play_or_stop,
                update_monitor_values,
                update_midinote_ch1_text,
            )
                .chain(),
        )
        .run();
}
