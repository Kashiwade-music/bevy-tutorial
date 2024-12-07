use bevy::{
    prelude::*, render::camera::RenderTarget, window::EnabledButtons, window::WindowRef,
    window::WindowResolution,
};

use crate::global_vars::{AppState, GlobalSettings};

pub struct StatusWindowPlugin;

impl Plugin for StatusWindowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(Startup, setup_status_window)
            .add_systems(
                Update,
                (
                    print_status_midi_path,
                    print_status_status,
                    print_status_elapsed_time,
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

fn setup_status_window(mut commands: Commands) {
    // 2つ目のウィンドウを表示する
    let status_window = commands
        .spawn(Window {
            title: "Status Window".to_owned(),
            resizable: false,
            resolution: WindowResolution::new(1000.0, 100.0),
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
            parent.spawn(Text::new("Current States List"));
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
                    parent.spawn(Text::new("Play Time: "));
                    parent.spawn((Text::new(""), StatusElapsedTimeText));
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
    global_settings: Res<GlobalSettings>,
) {
    for mut text in &mut query {
        text.clear();
        text.push_str(
            format!(
                "{:?}",
                global_settings.elapsed_time_from_start.elapsed_secs()
            )
            .as_str(),
        );
    }
}
