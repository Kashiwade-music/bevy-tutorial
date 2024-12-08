use crate::global_vars::{
    AppState, GlobalMonitorValues, GlobalSettings, MainWindowCamera, MidiNote,
};
use bevy::prelude::*;

pub struct MidiNoteTextPlugin;

impl Plugin for MidiNoteTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(PostStartup, setup)
            .add_systems(Update, (update_midinote_ch1_text,));
    }
}

#[derive(Component)]
struct MidiNoteCh1Text;

fn setup(mut commands: Commands, query: Query<Entity, With<MainWindowCamera>>) {
    let main_window_camera = commands.entity(query.single()).id();

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
                TargetCamera(main_window_camera),
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

fn update_midinote_ch1_text(
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,
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
                .collect::<Vec<&MidiNote>>();
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
