use bevy::{
    prelude::*, render::camera::RenderTarget, window::EnabledButtons, window::WindowRef,
    window::WindowResolution,
};

use crate::global_vars::{AppState, GlobalMonitorValues, GlobalSettings};

pub struct StatusWindowPlugin;

impl Plugin for StatusWindowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(PostStartup, setup_status_window)
            .add_systems(
                Update,
                (
                    print_status_midi_path,
                    print_status_status,
                    print_status_elapsed_time,
                    print_status_midi_format,
                    print_status_midi_ppm,
                    print_status_midi_current_tempo,
                    print_status_midi_current_time_signature,
                    print_status_measure,
                    print_status_beat,
                    print_status_tick,
                ),
            );
    }
}

#[derive(Component)]
struct StatusMidiPathText;

#[derive(Component)]
struct StatusStatusText;

#[derive(Component)]
struct StatusElapsedTimeText;

#[derive(Component)]
struct StatusMidiFormatText;

#[derive(Component)]
struct StatusMidiPPMText;

#[derive(Component)]
struct StatusMidiCurrentTempoText;

#[derive(Component)]
struct StatusMidiCurrentTimeSignatureText;

#[derive(Component)]
struct StatusMeasureText;

#[derive(Component)]
struct StatusBeatText;

#[derive(Component)]
struct StatusTickText;

fn setup_status_window(mut commands: Commands) {
    // 2つ目のウィンドウを表示する
    let status_window = commands
        .spawn(Window {
            title: "Status Window".to_owned(),
            resizable: false,
            resolution: WindowResolution::new(1200.0, 200.0),
            enabled_buttons: EnabledButtons {
                close: true,
                minimize: false,
                maximize: false,
            },
            ..default()
        })
        .id();

    let status_window_camera = commands
        .spawn((
            Camera2d::default(),
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(status_window)),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            TargetCamera(status_window_camera),
        ))
        .with_children(|parent| {
            // header
            parent.spawn(Text::new("Current States List"));

            // two rows
            parent
                .spawn(Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            width: Val::Percent(50.),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("MIDI File Path: "));
                                    parent.spawn((Text::new(""), StatusMidiPathText));
                                });

                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("Status: "));
                                    parent.spawn((Text::new(""), StatusStatusText));
                                });

                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("Format: "));
                                    parent.spawn((Text::new(""), StatusMidiFormatText));
                                });

                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("PPM: "));
                                    parent.spawn((Text::new(""), StatusMidiPPMText));
                                });
                        });
                    parent
                        .spawn(Node {
                            width: Val::Percent(50.),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("Play Time: "));
                                    parent.spawn((Text::new(""), StatusElapsedTimeText));
                                });
                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("Current Tempo: "));
                                    parent.spawn((Text::new(""), StatusMidiCurrentTempoText));
                                });
                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("Current Time Signature: "));
                                    parent
                                        .spawn((Text::new(""), StatusMidiCurrentTimeSignatureText));
                                });
                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("Measure: "));
                                    parent.spawn((Text::new(""), StatusMeasureText));
                                });
                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("Beat: "));
                                    parent.spawn((Text::new(""), StatusBeatText));
                                });
                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(Text::new("Tick: "));
                                    parent.spawn((Text::new(""), StatusTickText));
                                });
                        });
                });
        });
}

fn print_status_midi_path(
    mut query: Query<&mut Text, With<StatusMidiPathText>>,
    global_settings: Res<GlobalSettings>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(global_settings.midi_path.as_str());
    }
}

fn print_status_status(
    mut query: Query<&mut Text, With<StatusStatusText>>,
    app_state: Res<State<AppState>>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(format!("{:?}", app_state.get()).as_str());
    }
}

fn print_status_elapsed_time(
    mut query: Query<&mut Text, With<StatusElapsedTimeText>>,
    global_monitor_values: Res<GlobalMonitorValues>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(
            format!(
                "{:.2}",
                global_monitor_values.elapsed_time_from_start.elapsed_secs()
            )
            .as_str(),
        );
    }
}

fn print_status_midi_format(
    mut query: Query<&mut Text, With<StatusMidiFormatText>>,
    global_settings: Res<GlobalSettings>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(format!("{:?}", global_settings.format).as_str());
    }
}

fn print_status_midi_ppm(
    mut query: Query<&mut Text, With<StatusMidiPPMText>>,
    global_settings: Res<GlobalSettings>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(format!("{:?}", global_settings.ppm).as_str());
    }
}

fn print_status_midi_current_tempo(
    mut query: Query<&mut Text, With<StatusMidiCurrentTempoText>>,
    global_monitor_values: Res<GlobalMonitorValues>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(format!("{:.2}", global_monitor_values.time_axis.tempo.clone()).as_str());
    }
}

fn print_status_midi_current_time_signature(
    mut query: Query<&mut Text, With<StatusMidiCurrentTimeSignatureText>>,
    global_settings: Res<GlobalSettings>,
    global_monitor_values: Res<GlobalMonitorValues>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(
            format!(
                "{:?}",
                global_monitor_values
                    .time_axis
                    .time_signature_numerator
                    .clone()
            )
            .as_str(),
        );
        text.push_str("/");
        text.push_str(
            format!(
                "{:?}",
                global_monitor_values
                    .time_axis
                    .time_signature_denominator
                    .clone()
            )
            .as_str(),
        );
    }
}

fn print_status_measure(
    mut query: Query<&mut Text, With<StatusMeasureText>>,
    global_monitor_values: Res<GlobalMonitorValues>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(format!("{:?}", global_monitor_values.time_axis.measure.clone()).as_str());
    }
}

fn print_status_beat(
    mut query: Query<&mut Text, With<StatusBeatText>>,
    global_monitor_values: Res<GlobalMonitorValues>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(format!("{:?}", global_monitor_values.time_axis.beat.clone()).as_str());
    }
}

fn print_status_tick(
    mut query: Query<&mut Text, With<StatusTickText>>,
    global_monitor_values: Res<GlobalMonitorValues>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(format!("{:?}", global_monitor_values.time_axis.tick.clone()).as_str());
    }
}