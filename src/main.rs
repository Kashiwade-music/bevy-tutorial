//! Uses two windows to visualize a 3D model from different angles.

use bevy::prelude::*;

mod global_vars;
mod status_window;

fn setup_scene(mut commands: Commands) {
    let first_window_camera = commands.spawn((Camera2d::default(),)).id();
    //2Dテキストをspawnする
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

fn main() {
    App::new()
        // By default, a primary window gets spawned by `WindowPlugin`, contained in `DefaultPlugins`
        .add_plugins(DefaultPlugins)
        .add_plugins(status_window::StatusWindowPlugin)
        .init_state::<global_vars::AppState>()
        .add_systems(Startup, setup_scene)
        .run();
}
