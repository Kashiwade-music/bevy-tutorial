use crate::cubic_bezier;
use crate::global_vars::{
    AppState, GlobalMonitorValues, GlobalSettings, MainWindowCamera, MidiNote,
};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::time::Stopwatch;

pub struct MidiNoteAnimatePlugin;

impl Plugin for MidiNoteAnimatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(PostStartup, setup)
            .add_systems(Update, (update_visibility_of_midi_measure,));
    }
}

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
    total_animation_time: f32,
    state: AnimateState,
}

#[derive(Component)]
struct MidiMeasure {
    measure: u32,
}

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

    // spawn midi measure first
    let mut measure_entities = Vec::new();
    let max_measure = if let Some(max_measure) = global_settings
        .time_axis_vec
        .iter()
        .max_by_key(|time_axis| time_axis.measure)
        .map(|time_axis| time_axis.measure)
    {
        max_measure
    } else {
        0
    };
    for i in 0..(max_measure + 1) {
        let measure_entity = commands
            .spawn((
                MidiMeasure { measure: i as u32 },
                Transform::from_xyz(
                    -(global_settings.window_width as f32) / 2.0 + 25.0 * 4.0,
                    -(global_settings.window_height as f32) / 2.0 + 25.0 * 9.0,
                    0.0,
                ),
                Visibility::Hidden,
                RenderLayers::layer(0),
            ))
            .id();
        measure_entities.push(measure_entity);
    }

    // spawn midi notes
    for midi_notes in &global_settings.midi_notes_vec {
        for midi_note in midi_notes {
            let cubic_bezier = cubic_bezier::CubicBezier::new(
                cubic_bezier::Vec2 { x: 0.85, y: 0.0 },
                cubic_bezier::Vec2 { x: 0.15, y: 1.0 },
            );
            width_per_tick = width_piano_roll / midi_note.measure_length_ticks.unwrap() as f32;

            commands
                .entity(measure_entities[midi_note.note_on_measure.unwrap() as usize])
                .with_children(|parent| {
                    let note_width = width_per_tick * midi_note.note_length_ticks.unwrap() as f32;

                    parent.spawn((
                        Transform::from_xyz(
                            width_per_tick
                                * midi_note.note_on_tick_reset_by_measure.unwrap() as f32
                                + note_width / 2.0,
                            (midi_note.key - min_key) as f32 * note_height + note_height / 2.0,
                            0.0,
                        ),
                        GlobalTransform::default(),
                        Mesh2d(meshes.add(Rectangle::new(note_width, note_height))),
                        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                        MidiNoteForAnimate {
                            midi_note: midi_note.clone(),
                            cubic_bezier,
                            elapsed_time: Stopwatch::new(),
                            total_animation_time: 0.3,
                            state: AnimateState::Invisible,
                        },
                        TargetCamera(main_window_camera),
                    ));
                });
        }
    }
}

fn update_visibility_of_midi_measure(
    mut query: Query<(&MidiMeasure, &mut Visibility)>,
    global_monitor_values: Res<GlobalMonitorValues>,
) {
    for (midi_measure, mut visibility) in query.iter_mut() {
        if (midi_measure.measure as f32) == global_monitor_values.time_axis.measure as f32 {
            if visibility.clone() == Visibility::Hidden {
                visibility.toggle_visible_hidden();
            }
        } else {
            if visibility.clone() == Visibility::Visible {
                visibility.toggle_visible_hidden();
            }
        }
    }
}
