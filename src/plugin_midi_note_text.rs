use crate::global_vars::{GlobalMonitorValues, GlobalSettings, MainWindowCamera, MidiNote};
use crate::util_color;
use bevy::prelude::*;

pub struct MidiNoteTextPlugin;

impl Plugin for MidiNoteTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            .add_systems(Update, (update_midinote_ch1_text,));
    }
}

#[derive(Component)]
struct MidiNoteCh1Text;

fn setup(
    mut commands: Commands,
    query: Query<Entity, With<MainWindowCamera>>,
    global_settings: Res<GlobalSettings>,
) {
    let main_window_camera = commands.entity(query.single()).id();
    let color = util_color::adjust_color(
        &global_settings.config.theme[0].main_base_hex,
        &global_settings.config.theme[0].background_hex,
        1,
        4,
    )
    .unwrap();

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
                TextColor(Color::srgb(color[0], color[1], color[2])),
            ));
            parent
                .spawn(Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Notes: "),
                        TextColor(Color::srgb(color[0], color[1], color[2])),
                    ));
                    parent.spawn((
                        Text::new(""),
                        MidiNoteCh1Text,
                        TextColor(Color::srgb(color[0], color[1], color[2])),
                    ));
                });
        });
}

fn update_midinote_ch1_text(
    global_monitor_values: Res<GlobalMonitorValues>,
    global_settings: Res<GlobalSettings>,
    mut query: Query<&mut Text, With<MidiNoteCh1Text>>,
) {
    let time_axis = global_monitor_values.current_time_axis;

    for mut text in &mut query {
        text.clear();
        for (i, midi_notes) in global_settings.midi_notes_vec.iter().enumerate() {
            let current_note_on_notes_vec = midi_notes
                .iter()
                .filter(|x| {
                    x.note_on_time_axis.ticks_total <= time_axis.ticks_total
                        && x.note_off_time_axis.unwrap().ticks_total >= time_axis.ticks_total
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
