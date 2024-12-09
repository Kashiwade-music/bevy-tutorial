use crate::global_vars::{GlobalMonitorValues, GlobalSettings, MainWindowCamera, MidiNote};
use crate::util_color;
use bevy::log::tracing_subscriber::fmt::time;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::Anchor;

pub struct TransportPanelPlugin;

impl Plugin for TransportPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup).add_systems(
            Update,
            (
                update_elapsed_time_minutes_text,
                update_elapsed_time_seconds_text,
                update_elapsed_time_millis_text,
                update_measure_text,
                update_beat_text,
                update_tick_reset_by_beat_text,
                update_beat_bar,
                update_measure_bar,
            ),
        );
    }
}

#[derive(Component)]
struct TransportPanelRoot;

#[derive(Component)]
struct ElapsedTimeMinutesText;

#[derive(Component)]
struct ElapsedTimeSecondsText;

#[derive(Component)]
struct ElapsedTimeMillisText;

#[derive(Component)]
struct MeasureText;

#[derive(Component)]
struct BeatText;

#[derive(Component)]
struct TickResetByBeatText;

#[derive(Component)]
struct BeatBarAnimation {
    pub max_length: f32,
}

#[derive(Component)]
struct BeatBar;

#[derive(Component)]
struct MeasureBarAnimation {
    pub max_length: f32,
}

#[derive(Component)]
struct MeasureBar;

fn setup(
    mut commands: Commands,
    query: Query<Entity, With<MainWindowCamera>>,
    global_settings: Res<GlobalSettings>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let main_window_camera = commands.entity(query.single()).id();
    let font: Handle<Font> = asset_server.load("fonts\\NotoSansJP-Thin.ttf");
    let color_levels: Vec<_> = (0..=4)
        .map(|level| {
            util_color::adjust_color(
                &global_settings.config.theme[0].main_base_hex,
                &global_settings.config.theme[0].background_hex,
                level,
                4,
            )
            .unwrap()
        })
        .collect();

    let transport_panel_layout = global_settings
        .config
        .feature_and_layout
        .transport_panel
        .calculate_rect(
            global_settings.config.main_config.window_width,
            global_settings.config.main_config.window_height,
        )
        .unwrap();

    let transport_panel_root_entity = commands
        .spawn((
            TransportPanelRoot,
            Transform::from_xyz(
                transport_panel_layout.left_bottom_abs_pixel.0,
                transport_panel_layout.left_bottom_abs_pixel.1,
                0.0,
            ),
            Visibility::default(),
            RenderLayers::layer(0),
            transport_panel_layout,
        ))
        .id();

    let time_text_y = 80.0;
    let time_text_font_size = 40.0;
    let measure_beat_ticks_text_y = 0.0;
    let measure_beat_ticks_font_size = 40.0;
    let column_start_vec = vec![0.0, 90.0, 160.0];

    commands
        .entity(transport_panel_root_entity)
        .with_children(|parent| {
            parent.spawn((
                Transform::from_xyz(column_start_vec[0] + 21.0, time_text_y, 0.0),
                GlobalTransform::default(),
                ElapsedTimeMinutesText,
                Text2d::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: time_text_font_size,
                    ..default()
                },
                TextColor(Color::srgb(
                    color_levels[0][0],
                    color_levels[0][1],
                    color_levels[0][2],
                )),
                Anchor::BottomLeft,
            ));
            parent.spawn((
                Transform::from_xyz(column_start_vec[1], time_text_y, 0.0),
                GlobalTransform::default(),
                ElapsedTimeSecondsText,
                Text2d::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: time_text_font_size,
                    ..default()
                },
                TextColor(Color::srgb(
                    color_levels[0][0],
                    color_levels[0][1],
                    color_levels[0][2],
                )),
                Anchor::BottomLeft,
            ));
            parent
                .spawn((
                    Transform::from_xyz(column_start_vec[2] - 9.0, time_text_y, 0.0),
                    GlobalTransform::default(),
                    ElapsedTimeMillisText,
                    Text2d::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: time_text_font_size,
                        ..default()
                    },
                    TextColor(Color::srgb(
                        color_levels[0][0],
                        color_levels[0][1],
                        color_levels[0][2],
                    )),
                    Anchor::BottomLeft,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Transform::from_xyz(26.0, 45.0, 0.0),
                        GlobalTransform::default(),
                        Text2d::new("Time"),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(
                            color_levels[1][0],
                            color_levels[1][1],
                            color_levels[1][2],
                        )),
                        Anchor::BottomLeft,
                    ));
                });

            parent.spawn((
                Transform::from_xyz(column_start_vec[0], measure_beat_ticks_text_y, 0.0),
                GlobalTransform::default(),
                MeasureText,
                Text2d::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: measure_beat_ticks_font_size,
                    ..default()
                },
                TextColor(Color::srgb(
                    color_levels[0][0],
                    color_levels[0][1],
                    color_levels[0][2],
                )),
                Anchor::BottomLeft,
            ));
            parent
                .spawn((
                    Transform::from_xyz(column_start_vec[1], measure_beat_ticks_text_y, 0.0),
                    GlobalTransform::default(),
                    BeatText,
                    Text2d::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: measure_beat_ticks_font_size,
                        ..default()
                    },
                    TextColor(Color::srgb(
                        color_levels[0][0],
                        color_levels[0][1],
                        color_levels[0][2],
                    )),
                    Anchor::BottomLeft,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MeasureBarAnimation { max_length: 40.0 },
                        MeasureBar,
                        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.0, 1.0, 1.0)),
                        GlobalTransform::default(),
                        Mesh2d(meshes.add(Rectangle::new(40.0, 1.0))),
                        MeshMaterial2d(materials.add(Color::srgb(
                            color_levels[0][0],
                            color_levels[0][1],
                            color_levels[0][2],
                        ))),
                        Anchor::BottomLeft,
                    ));
                });
            parent
                .spawn((
                    Transform::from_xyz(column_start_vec[2], measure_beat_ticks_text_y, 0.0),
                    GlobalTransform::default(),
                    TickResetByBeatText,
                    Text2d::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: measure_beat_ticks_font_size,
                        ..default()
                    },
                    TextColor(Color::srgb(
                        color_levels[0][0],
                        color_levels[0][1],
                        color_levels[0][2],
                    )),
                    Anchor::BottomLeft,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Transform::from_xyz(20.0, 45.0, 0.0),
                        GlobalTransform::default(),
                        Text2d::new("Beat"),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(
                            color_levels[1][0],
                            color_levels[1][1],
                            color_levels[1][2],
                        )),
                        Anchor::BottomLeft,
                    ));
                    parent.spawn((
                        BeatBarAnimation { max_length: 62.0 },
                        BeatBar,
                        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.0, 1.0, 1.0)),
                        GlobalTransform::default(),
                        Mesh2d(meshes.add(Rectangle::new(62.0, 1.0))),
                        MeshMaterial2d(materials.add(Color::srgb(
                            color_levels[0][0],
                            color_levels[0][1],
                            color_levels[0][2],
                        ))),
                        Anchor::BottomLeft,
                    ));
                });
        });
}

fn update_elapsed_time_minutes_text(
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,
    mut query: Query<&mut Text2d, With<ElapsedTimeMinutesText>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for mut text in &mut query {
        text.clear();
        text.push_str(&format!(
            "{:02}",
            time_axis.seconds_total.floor() as u32 / 60
        ));
    }
}

fn update_elapsed_time_seconds_text(
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,
    mut query: Query<&mut Text2d, With<ElapsedTimeSecondsText>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for mut text in &mut query {
        text.clear();
        text.push_str(&format!(
            "{:02}",
            time_axis.seconds_total.floor() as u32 % 60
        ));
    }
}

fn update_elapsed_time_millis_text(
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,
    mut query: Query<&mut Text2d, With<ElapsedTimeMillisText>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for mut text in &mut query {
        text.clear();
        text.push_str(&format!(
            ".{:03}",
            (time_axis.seconds_total.fract() * 1000.0).floor() as u32
        ));
    }
}

fn update_measure_text(
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,
    mut query: Query<&mut Text2d, With<MeasureText>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for mut text in &mut query {
        text.clear();
        text.push_str(&format!("{:03}", time_axis.measure));
    }
}

fn update_beat_text(
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,
    mut query: Query<&mut Text2d, With<BeatText>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for mut text in &mut query {
        text.clear();
        text.push_str(&format!("{:02}", time_axis.beat));
    }
}

fn update_tick_reset_by_beat_text(
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,
    mut query: Query<&mut Text2d, With<TickResetByBeatText>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for mut text in &mut query {
        text.clear();
        text.push_str(&format!("{:03}", time_axis.ticks_reset_by_beat));
    }
}

fn update_beat_bar(
    global_monitor_values: Res<GlobalMonitorValues>,
    mut query: Query<(&mut Transform, &mut BeatBarAnimation), With<BeatBar>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for (mut transform, beat_bar) in query.iter_mut() {
        transform.scale = Vec3::new(
            time_axis.ticks_reset_by_beat as f32 / time_axis.beat_length_ticks as f32,
            1.0,
            1.0,
        );
        transform.translation.x = beat_bar.max_length * transform.scale.x / 2.0;
    }
}

fn update_measure_bar(
    global_monitor_values: Res<GlobalMonitorValues>,
    mut query: Query<(&mut Transform, &mut MeasureBarAnimation), With<MeasureBar>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for (mut transform, measure_bar) in query.iter_mut() {
        transform.scale = Vec3::new(
            time_axis.ticks_reset_by_measure as f32 / time_axis.measure_length_ticks as f32,
            1.0,
            1.0,
        );
        transform.translation.x = measure_bar.max_length * transform.scale.x / 2.0;
    }
}
