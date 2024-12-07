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

use crate::global_vars::{AppState, GlobalSettings, TimeAxis};

pub struct LoadMidiReturn {
    pub format: midly::Format,
    pub ppm: u16,
    pub time_axis_vec: Vec<TimeAxis>,
}

pub fn load_midi(midi_file_path: &str, commands: &mut Commands) -> LoadMidiReturn {
    let bytes = fs::read(midi_file_path).unwrap();
    let smf = Smf::parse(&bytes).unwrap();
    let format = smf.header.format;
    let ppm = match smf.header.timing {
        Timing::Metrical(ppm) => ppm.as_int(),
        _ => panic!("unsupported timing"),
    };

    let result_time_axis = get_time_axis(&smf);

    // for (i, track) in smf.tracks.iter().enumerate() {
    //     println!("track {} has {} events", i, track.len());
    //     for event in track.iter() {
    //         println!("    delta: {:>4}, {:?}", event.delta, event.kind);
    //     }
    // }
    return LoadMidiReturn {
        format,
        ppm,
        time_axis_vec: result_time_axis.time_axis,
    };
}

struct TempoChangeEvent {
    pub tempo: f32, // how many beats per minute
    pub time_sec: f32,
    pub total_ticks: u32,      // how many ticks before the tempo change
    pub seconds_per_tick: f32, // how many seconds per tick after the tempo change
}

struct TimeSignatureChangeEvent {
    pub numerator: u8,
    pub denominator: u8,
    pub midi_clocks_per_metronome_click: u8,
    pub thirty_seconds_notes_per_quarter_note: u8,
    pub time_sec: f32,
    pub total_ticks: u32, // how many ticks before the tempo change
}

struct GetTimeAxisReturn {
    time_axis: Vec<TimeAxis>,
}

fn get_time_axis(smf: &Smf) -> GetTimeAxisReturn {
    let ppm = match smf.header.timing {
        Timing::Metrical(ppm) => ppm.as_int(),
        _ => panic!("unsupported timing"),
    };

    let mut tempo_change_events: Vec<TempoChangeEvent> = Vec::new();
    let mut time_signature_change_events: Vec<TimeSignatureChangeEvent> = Vec::new();
    let mut end_of_track_ticks = 0;

    // テンポデータや拍子データは他トラックにまぎれていることがあるので、
    // トラックごとに解析する必要がある
    for track in smf.tracks.iter() {
        let mut current_time_sec = 0.0;
        let mut current_seconds_per_tick = 60.0 / 120.0 / ppm as f32;
        let mut total_ticks = 0;

        for event in track.iter() {
            current_time_sec += event.delta.as_int() as f32 * current_seconds_per_tick;
            total_ticks += event.delta.as_int();

            match event.kind {
                TrackEventKind::Meta(MetaMessage::Tempo(tempo)) => {
                    current_seconds_per_tick = tempo.as_int() as f32 * 1E-6 / ppm as f32;

                    let tempo_change_event = TempoChangeEvent {
                        tempo: 60.0 / ((tempo.as_int() as f32) * 1E-6),
                        time_sec: current_time_sec,
                        total_ticks: total_ticks,
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
                        denominator: 2u8.pow(denominator as u32),
                        midi_clocks_per_metronome_click,
                        thirty_seconds_notes_per_quarter_note,
                        time_sec: current_time_sec,
                        total_ticks: total_ticks,
                    };
                    time_signature_change_events.push(time_signature_change_event);
                }

                TrackEventKind::Meta(MetaMessage::EndOfTrack) => {
                    end_of_track_ticks = if total_ticks > end_of_track_ticks {
                        total_ticks
                    } else {
                        end_of_track_ticks
                    };
                }
                _ => {}
            }
        }
    }

    // sort by total_ticks
    tempo_change_events.sort_by(|a, b| a.total_ticks.cmp(&b.total_ticks));
    time_signature_change_events.sort_by(|a, b| a.total_ticks.cmp(&b.total_ticks));

    // make time_axis
    let mut time_axis: Vec<TimeAxis> = Vec::new();
    let mut current_seconds = 0.0;
    let mut current_measure = 0;
    let mut current_beat = 1;
    let mut current_tick = 0; // when the beat is 0, the tick is 0
    let mut current_tempo = 120.0; // beat per minute
    let mut current_seconds_per_tick = 60.0 / current_tempo / ppm as f32;
    let mut current_time_signature_numerator = 4;
    let mut current_time_signature_denominator = 4;
    let mut current_time_signature_midi_clocks_per_metronome_click = 24;
    let mut current_time_signature_thirty_seconds_notes_per_quarter_note = 8;

    for tick in 0..(end_of_track_ticks + 1) {
        // search tempo_change_event which has same ticks. if none, no change
        let tempo_change_event = tempo_change_events
            .iter()
            .rev()
            .find(|x| x.total_ticks == tick);
        if let Some(tempo_change_event) = tempo_change_event {
            current_tempo = tempo_change_event.tempo;
            current_seconds_per_tick = tempo_change_event.seconds_per_tick;
        }

        // search time_signature_change_event which has same ticks. if none, no change
        let time_signature_change_event = time_signature_change_events
            .iter()
            .rev()
            .find(|x| x.total_ticks == tick);
        if let Some(time_signature_change_event) = time_signature_change_event {
            current_time_signature_numerator = time_signature_change_event.numerator;
            current_time_signature_denominator = time_signature_change_event.denominator;
            current_time_signature_midi_clocks_per_metronome_click =
                time_signature_change_event.midi_clocks_per_metronome_click;
            current_time_signature_thirty_seconds_notes_per_quarter_note =
                time_signature_change_event.thirty_seconds_notes_per_quarter_note;
        }

        current_measure = tick as u32
            / (ppm as u32 * current_time_signature_numerator as u32 * 4
                / current_time_signature_denominator as u32);
        current_beat = tick as u32 / (ppm as u32 * 4 / current_time_signature_denominator as u32)
            % current_time_signature_numerator as u32
            + 1;
        current_tick = tick as u32 % (ppm as u32 * 4 / current_time_signature_denominator as u32);

        let time_axis_data = TimeAxis {
            ticks_index: tick,
            seconds: current_seconds, // ticks == 0のときは0
            measure: current_measure, // ticks == 0のときは0
            beat: current_beat,       // ticks == 0のときは1
            tick: current_tick,       // ticks == 0のときは0
            tempo: current_tempo,
            time_signature_numerator: current_time_signature_numerator,
            time_signature_denominator: current_time_signature_denominator,
            time_signature_midi_clocks_per_metronome_click:
                current_time_signature_midi_clocks_per_metronome_click,
            time_signature_thirty_seconds_notes_per_quarter_note:
                current_time_signature_thirty_seconds_notes_per_quarter_note,
        };
        time_axis.push(time_axis_data);

        current_seconds += current_seconds_per_tick;
    }

    return GetTimeAxisReturn { time_axis };
}
