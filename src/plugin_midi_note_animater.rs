use crate::cubic_bezier;
use crate::global_vars::{
    AppState, GlobalMonitorValues, GlobalSettings, MainWindowCamera, MidiNote,
};
use bevy::log::tracing_subscriber::fmt::time;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::time::Stopwatch;

pub struct MidiNoteAnimatePlugin;

impl Plugin for MidiNoteAnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup).add_systems(
            Update,
            (
                update_midi_note_state_1,
                update_midi_note_state_2,
                update_midi_note_state_3,
                update_midi_note_state_4,
            ),
        );
    }
}

#[derive(PartialEq, Eq)]
enum AnimateState {
    In,
    Visible,
    Out,
    Invisible,
}

#[derive(Component)]
struct MidiNoteForAnimate {
    midi_note: MidiNote,
    cubic_bezier: cubic_bezier::CubicBezier,
    elapsed_time: Stopwatch,
    total_animation_time_sec: f32,
    state: AnimateState,
    full_note_length: f32,
    x_pos_of_note: f32,
}

#[derive(Component)]
struct MidiNoteParallel1;

#[derive(Component)]
struct MidiNoteParallel2;

#[derive(Component)]
struct MidiNoteParallel3;

#[derive(Component)]
struct MidiNoteParallel4;

#[derive(Component)]
struct MidiPianoRollRoot;

fn setup(
    mut commands: Commands,
    query_camera: Query<Entity, With<MainWindowCamera>>,
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let main_window_camera = commands.entity(query_camera.single()).id();

    // midiピアノロールの領域等の定数
    let width_piano_roll = 1720.0;
    let height_piano_roll = 677.0;

    // 表示する縦方向の分解能
    let note_height = height_piano_roll / 88.0;

    // 最低音と最高音のkey u32
    let min_key = 21;
    let max_key = 108;

    // 表示する横方向の分解能
    let mut width_per_tick = width_piano_roll / (4.0 * global_settings.ppm as f32);

    let piano_roll_root_entity = commands
        .spawn((
            MidiPianoRollRoot,
            Transform::from_xyz(
                -(global_settings.window_width as f32) / 2.0 + 25.0 * 4.0,
                -(global_settings.window_height as f32) / 2.0 + 25.0 * 9.0,
                0.0,
            ),
            Visibility::default(),
            RenderLayers::layer(0),
        ))
        .id();

    // spawn midi notes
    for midi_notes in &global_settings.midi_notes_vec {
        for midi_note in midi_notes {
            let cubic_bezier = cubic_bezier::CubicBezier::new(
                cubic_bezier::Vec2 { x: 0.85, y: 0.0 },
                cubic_bezier::Vec2 { x: 0.15, y: 1.0 },
            );
            width_per_tick = width_piano_roll / midi_note.measure_length_ticks.unwrap() as f32;
            let note_width = width_per_tick * midi_note.note_length_ticks.unwrap() as f32;
            let x_pos_of_note =
                width_per_tick * midi_note.note_on_tick_reset_by_measure.unwrap() as f32;

            let default_bundle = (
                Transform::from_xyz(
                    x_pos_of_note,
                    (midi_note.key - min_key) as f32 * note_height + note_height / 2.0,
                    0.0,
                )
                .with_scale(Vec3::new(0.0, 1.0, 1.0)),
                GlobalTransform::default(),
                Mesh2d(meshes.add(Rectangle::new(note_width, note_height))),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                MidiNoteForAnimate {
                    midi_note: midi_note.clone(),
                    cubic_bezier,
                    elapsed_time: Stopwatch::new(),
                    total_animation_time_sec: 0.3,
                    state: AnimateState::Invisible,
                    full_note_length: note_width,
                    x_pos_of_note,
                },
                Visibility::Hidden,
                TargetCamera(main_window_camera),
            );

            // channelによって、spawnするentityを変える。MidiNoteParallel1 ~ 4
            match midi_note.channel % 4 {
                0 => {
                    commands
                        .entity(piano_roll_root_entity)
                        .with_children(|parent| {
                            parent.spawn((MidiNoteParallel1, default_bundle));
                        });
                }
                1 => {
                    commands
                        .entity(piano_roll_root_entity)
                        .with_children(|parent| {
                            parent.spawn((MidiNoteParallel2, default_bundle));
                        });
                }
                2 => {
                    commands
                        .entity(piano_roll_root_entity)
                        .with_children(|parent| {
                            parent.spawn((MidiNoteParallel3, default_bundle));
                        });
                }
                3 => {
                    commands
                        .entity(piano_roll_root_entity)
                        .with_children(|parent| {
                            parent.spawn((MidiNoteParallel4, default_bundle));
                        });
                }
                _ => {}
            };
        }
    }
}

fn update_midi_note_state_logic(
    time: &Res<Time>,
    midi_note_for_animate: &mut MidiNoteForAnimate,
    transform: &mut Transform,
    visibility: &mut Visibility,
    global_monitor_values: &GlobalMonitorValues,
    app_state: &Res<State<AppState>>,
) {
    if app_state.get() == &AppState::Stop {
        midi_note_for_animate.state = AnimateState::Invisible;
        midi_note_for_animate.elapsed_time.reset();
        if visibility.clone() == Visibility::Visible {
            visibility.toggle_visible_hidden();
        }
        transform.scale = Vec3::new(0.0, 1.0, 1.0);
        transform.translation.x = midi_note_for_animate.x_pos_of_note;
        return;
    }
    if midi_note_for_animate.state == AnimateState::Invisible {
        if (midi_note_for_animate.midi_note.note_on_ticks
            <= global_monitor_values.time_axis.ticks_index)
            && (midi_note_for_animate.midi_note.note_on_measure.unwrap()
                == global_monitor_values.time_axis.measure)
        {
            midi_note_for_animate.state = AnimateState::In;
            visibility.toggle_visible_hidden();
        }
    } else if midi_note_for_animate.state == AnimateState::In {
        midi_note_for_animate.elapsed_time.tick(time.delta());
        let x = midi_note_for_animate.elapsed_time.elapsed_secs()
            / midi_note_for_animate.total_animation_time_sec;
        let y = midi_note_for_animate.cubic_bezier.solve_y(x).unwrap();
        transform.scale = Vec3::new(y, 1.0, 1.0);
        transform.translation.x =
            midi_note_for_animate.x_pos_of_note + midi_note_for_animate.full_note_length * y / 2.0;
        if midi_note_for_animate.elapsed_time.elapsed_secs()
            >= midi_note_for_animate.total_animation_time_sec
        {
            midi_note_for_animate.state = AnimateState::Visible;
            midi_note_for_animate.elapsed_time.reset();
        }
    } else if midi_note_for_animate.state == AnimateState::Visible {
        if (midi_note_for_animate.midi_note.note_off_ticks.unwrap()
            <= global_monitor_values.time_axis.ticks_index)
            && (midi_note_for_animate.midi_note.note_off_measure.unwrap()
                < global_monitor_values.time_axis.measure)
        {
            midi_note_for_animate.state = AnimateState::Out;
        }
    } else if midi_note_for_animate.state == AnimateState::Out {
        midi_note_for_animate.elapsed_time.tick(time.delta());
        let x = midi_note_for_animate.elapsed_time.elapsed_secs()
            / midi_note_for_animate.total_animation_time_sec;
        let y = midi_note_for_animate.cubic_bezier.solve_y(x).unwrap();
        transform.scale = Vec3::new(1.0 - y, 1.0, 1.0);
        transform.translation.x = midi_note_for_animate.x_pos_of_note
            + midi_note_for_animate.full_note_length * y
            + midi_note_for_animate.full_note_length * (1.0 - y) / 2.0;
        if midi_note_for_animate.elapsed_time.elapsed_secs()
            >= midi_note_for_animate.total_animation_time_sec
        {
            midi_note_for_animate.state = AnimateState::Invisible;
            midi_note_for_animate.elapsed_time.reset();
            visibility.toggle_visible_hidden();
        }
    }
}

fn update_midi_note_state_1(
    time: Res<Time>,
    global_monitor_values: Res<GlobalMonitorValues>,
    mut query: Query<
        (&mut MidiNoteForAnimate, &mut Transform, &mut Visibility),
        With<MidiNoteParallel1>,
    >,
    app_state: Res<State<AppState>>,
) {
    for (mut midi_note_for_animate, mut transform, mut visibility) in query.iter_mut() {
        update_midi_note_state_logic(
            &time,
            &mut midi_note_for_animate,
            &mut transform,
            &mut visibility,
            &global_monitor_values,
            &app_state,
        );
    }
}

fn update_midi_note_state_2(
    time: Res<Time>,
    global_monitor_values: Res<GlobalMonitorValues>,
    mut query: Query<
        (&mut MidiNoteForAnimate, &mut Transform, &mut Visibility),
        With<MidiNoteParallel2>,
    >,
    app_state: Res<State<AppState>>,
) {
    for (mut midi_note_for_animate, mut transform, mut visibility) in query.iter_mut() {
        update_midi_note_state_logic(
            &time,
            &mut midi_note_for_animate,
            &mut transform,
            &mut visibility,
            &global_monitor_values,
            &app_state,
        );
    }
}

fn update_midi_note_state_3(
    time: Res<Time>,
    global_monitor_values: Res<GlobalMonitorValues>,
    mut query: Query<
        (&mut MidiNoteForAnimate, &mut Transform, &mut Visibility),
        With<MidiNoteParallel3>,
    >,
    app_state: Res<State<AppState>>,
) {
    for (mut midi_note_for_animate, mut transform, mut visibility) in query.iter_mut() {
        update_midi_note_state_logic(
            &time,
            &mut midi_note_for_animate,
            &mut transform,
            &mut visibility,
            &global_monitor_values,
            &app_state,
        );
    }
}

fn update_midi_note_state_4(
    time: Res<Time>,
    global_monitor_values: Res<GlobalMonitorValues>,
    mut query: Query<
        (&mut MidiNoteForAnimate, &mut Transform, &mut Visibility),
        With<MidiNoteParallel4>,
    >,
    app_state: Res<State<AppState>>,
) {
    for (mut midi_note_for_animate, mut transform, mut visibility) in query.iter_mut() {
        update_midi_note_state_logic(
            &time,
            &mut midi_note_for_animate,
            &mut transform,
            &mut visibility,
            &global_monitor_values,
            &app_state,
        );
    }
}
