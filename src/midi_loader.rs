use bevy::{
    app::DynEq,
    prelude::*,
    render::camera::RenderTarget,
    state::commands,
    window::{EnabledButtons, WindowRef, WindowResolution},
};
use core::num;
use midly::{MetaMessage, Smf, Timing, TrackEvent, TrackEventKind};
use std::fs;

use crate::global_vars::{AppState, GlobalSettings, TempoChangeEvent, TimeSignatureChangeEvent};

pub struct LoadMidiReturn {
    pub format: midly::Format,
    pub timing: midly::Timing,
    pub tempo_change_events: Vec<TempoChangeEvent>,
    pub time_signature_change_events: Vec<TimeSignatureChangeEvent>,
}

pub fn load_midi(midi_file_path: &str, commands: &mut Commands) -> LoadMidiReturn {
    let bytes = fs::read(midi_file_path).unwrap();
    let smf = Smf::parse(&bytes).unwrap();
    let format = smf.header.format;
    let timing = smf.header.timing;
    let ppm = match timing {
        Timing::Metrical(ppm) => ppm.as_int(),
        _ => panic!("unsupported timing"),
    };

    // グローバルメタ情報から取得する
    let mut tempo_change_events: Vec<TempoChangeEvent> = Vec::new();
    let mut time_signature_change_events: Vec<TimeSignatureChangeEvent> = Vec::new();

    for (i, track) in smf.tracks.iter().enumerate() {
        if i == 0 {
            // meta SysEx track
            let mut current_time_sec = 0.0;
            let mut current_seconds_per_tick = 60.0 / 120.0 / ppm as f32;

            for event in track.iter() {
                current_time_sec += event.delta.as_int() as f32 * current_seconds_per_tick;

                match event.kind {
                    TrackEventKind::Meta(MetaMessage::Tempo(tempo)) => {
                        current_seconds_per_tick = tempo.as_int() as f32 * 1E-6 / ppm as f32;

                        let tempo_change_event = TempoChangeEvent {
                            tempo: 60.0 / ((tempo.as_int() as f32) * 1E-6),
                            time_sec: current_time_sec,
                            seconds_per_tick: current_seconds_per_tick,
                        };
                        tempo_change_events.push(tempo_change_event);
                    }
                    TrackEventKind::Meta(MetaMessage::TimeSignature(
                        numerater,
                        denominator,
                        midi_clocks_per_metronome_click,
                        thirty_seconds_notes_per_quarter_note,
                    )) => {
                        let time_signature_change_event = TimeSignatureChangeEvent {
                            numerator: numerater,
                            denominator,
                            midi_clocks_per_metronome_click,
                            thirty_seconds_notes_per_quarter_note,
                            time_sec: current_time_sec,
                        };
                        time_signature_change_events.push(time_signature_change_event);
                    }
                    _ => {}
                }
            }
        }
    }

    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
        for event in track.iter() {
            println!("    delta: {:>4}, {:?}", event.delta, event.kind);
        }
    }
    return LoadMidiReturn {
        format,
        timing,
        tempo_change_events,
        time_signature_change_events,
    };
}
