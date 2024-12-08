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

use crate::global_vars::{AppState, MidiNote, TimeAxis};

pub struct LoadMidiReturn {
    pub format: midly::Format,
    pub ppm: u16,
    pub time_axis_vec: Vec<TimeAxis>,
    pub midi_notes_vec: Vec<Vec<MidiNote>>,
}

pub fn load_midi(midi_file_path: &str) -> LoadMidiReturn {
    let bytes = fs::read(midi_file_path).unwrap();
    let smf = Smf::parse(&bytes).unwrap();
    let format = smf.header.format;
    let ppm = match smf.header.timing {
        Timing::Metrical(ppm) => ppm.as_int(),
        _ => panic!("unsupported timing"),
    };

    let result_time_axis_vec = get_time_axis(&smf);
    let result_midi_notes_vec = get_midi_notes(&smf, &result_time_axis_vec.time_axis);

    // for (i, track) in smf.tracks.iter().enumerate() {
    //     println!("track {} has {} events", i, track.len());
    //     for event in track.iter() {
    //         println!("    delta: {:>4}, {:?}", event.delta, event.kind);
    //     }
    // }
    return LoadMidiReturn {
        format,
        ppm,
        time_axis_vec: result_time_axis_vec.time_axis,
        midi_notes_vec: result_midi_notes_vec.midi_notes,
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
    let mut current_tick_reset_by_measure = 0;
    let mut current_measure_length_ticks = ppm as u32 * 4;
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

        current_measure_length_ticks = ppm as u32 * current_time_signature_numerator as u32 * 4
            / current_time_signature_denominator as u32;
        current_measure = tick as u32
            / (ppm as u32 * current_time_signature_numerator as u32 * 4
                / current_time_signature_denominator as u32);
        current_tick_reset_by_measure = tick as u32
            % (ppm as u32 * current_time_signature_numerator as u32 * 4
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
            tick_reset_by_measure: current_tick_reset_by_measure,
            measure_length_ticks: current_measure_length_ticks,
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

struct GetMidiNotesReturn {
    midi_notes: Vec<Vec<MidiNote>>, // channel, notes
}

fn get_midi_notes(smf: &Smf, time_axis: &Vec<TimeAxis>) -> GetMidiNotesReturn {
    let mut midi_notes: Vec<Vec<MidiNote>> = vec![Vec::new(); 16]; // 16 channels

    // secondsは後でticksを元に計算する

    for track in smf.tracks.iter() {
        let mut total_ticks = 0;

        for event in track.iter() {
            total_ticks += event.delta.as_int();

            match event.kind {
                TrackEventKind::Midi {
                    channel,
                    message: midly::MidiMessage::NoteOn { key, vel },
                } => {
                    if vel > 0 {
                        let note_on_ticks = total_ticks;

                        let key_cdefgab = match key.as_int() % 12 {
                            0 => "C",
                            1 => "C#",
                            2 => "D",
                            3 => "D#",
                            4 => "E",
                            5 => "F",
                            6 => "F#",
                            7 => "G",
                            8 => "G#",
                            9 => "A",
                            10 => "A#",
                            11 => "B",
                            _ => panic!("key error"),
                        };

                        let key_octave_yamaha = key.as_int() as i32 / 12 - 2;
                        let key_octave_general_midi = key.as_int() as i32 / 12 - 1;

                        let midi_note = MidiNote {
                            note_on_ticks,
                            note_on_seconds: None,
                            note_on_measure: None,
                            note_on_beat: None,
                            note_on_tick: None,
                            note_on_tick_reset_by_measure: None,
                            note_off_ticks: None,
                            note_off_seconds: None,
                            note_off_measure: None,
                            note_off_beat: None,
                            note_off_tick: None,
                            note_off_tick_reset_by_measure: None,

                            note_length_ticks: None,

                            measure_length_ticks: None,

                            key: key.as_int() as u32,
                            key_cdefgab: key_cdefgab.to_string(),
                            key_octave_yamaha,
                            key_octave_general_midi,
                            key_and_octave_yamaha: format!("{}{}", key_cdefgab, key_octave_yamaha),
                            velocity: vel.as_int() as u32,
                            channel: channel.as_int() as u32,
                        };

                        midi_notes[channel.as_int() as usize].push(midi_note);
                    } else {
                        // it is possibly note off event
                        let note_on_event = midi_notes[channel.as_int() as usize]
                            .iter_mut()
                            .rev()
                            .find(|x| x.key == key.as_int() as u32 && x.note_off_ticks.is_none());
                        if let Some(note_on_event) = note_on_event {
                            note_on_event.note_off_ticks = Some(total_ticks);
                        }
                    }
                }
                TrackEventKind::Midi {
                    channel,
                    message: midly::MidiMessage::NoteOff { key, vel: _ },
                } => {
                    // search note_on event which has same key and channel and note_off_ticks is None
                    let note_on_event = midi_notes[channel.as_int() as usize]
                        .iter_mut()
                        .rev()
                        .find(|x| x.key == key.as_int() as u32 && x.note_off_ticks.is_none());
                    if let Some(note_on_event) = note_on_event {
                        note_on_event.note_off_ticks = Some(total_ticks);
                    }
                }
                _ => {}
            }
        }
    }

    let ppm = match smf.header.timing {
        Timing::Metrical(ppm) => ppm.as_int(),
        _ => panic!("unsupported timing"),
    };

    // calculate seconds
    for channel in midi_notes.iter_mut() {
        for note in channel.iter_mut() {
            let note_on_time_axis = time_axis
                .iter()
                .find(|x| x.ticks_index == note.note_on_ticks);
            if let Some(note_on_time_axis) = note_on_time_axis {
                note.note_on_seconds = Some(note_on_time_axis.seconds);
                note.note_on_measure = Some(note_on_time_axis.measure);
                note.note_on_beat = Some(note_on_time_axis.beat);
                note.note_on_tick = Some(note_on_time_axis.tick);
                note.measure_length_ticks = Some(
                    ppm as u32 * note_on_time_axis.time_signature_numerator as u32 * 4
                        / note_on_time_axis.time_signature_denominator as u32,
                );
                note.note_on_tick_reset_by_measure = Some(note_on_time_axis.tick_reset_by_measure);
            }

            let note_off_time_axis = time_axis
                .iter()
                .find(|x| x.ticks_index == note.note_off_ticks.unwrap());
            if let Some(note_off_time_axis) = note_off_time_axis {
                note.note_off_seconds = Some(note_off_time_axis.seconds);
                note.note_off_measure = Some(note_off_time_axis.measure);
                note.note_off_beat = Some(note_off_time_axis.beat);
                note.note_off_tick = Some(note_off_time_axis.tick);
                note.note_length_ticks = Some(note.note_off_ticks.unwrap() - note.note_on_ticks);
                note.note_off_tick_reset_by_measure =
                    Some(note_off_time_axis.tick_reset_by_measure);
            }
        }
    }

    // 小節を跨ぐノートは分割する
    let mut new_midi_notes: Vec<Vec<MidiNote>> = Vec::new();
    for channel in midi_notes.iter_mut() {
        let mut new_notes: Vec<MidiNote> = Vec::new();
        for note in channel.iter_mut() {
            if note.note_on_measure.unwrap() == note.note_off_measure.unwrap() {
                new_notes.push(note.clone());
            } else {
                // まず現状のノートを修正
                let mut current_note = note.clone();

                for current_measure in current_note.note_on_measure.unwrap()
                    ..(current_note.note_off_measure.unwrap() + 1)
                {
                    if current_note.note_off_measure.unwrap() == current_measure {
                        // 小節を跨ぐノートの最後の小節に該当
                        new_notes.push(current_note.clone());
                    } else {
                        // 小節を跨ぐノートの途中の小説に該当
                        // 末尾を小節内に収めたノートを作成し、current_noteはnote_on系を修正
                        let mut new_note = current_note.clone();
                        let last_time_axis = time_axis
                            .iter()
                            .rev()
                            .find(|x| x.measure == current_measure)
                            .unwrap();
                        new_note.note_off_ticks = Some(last_time_axis.ticks_index);
                        new_note.note_off_seconds = Some(last_time_axis.seconds);
                        new_note.note_off_measure = Some(last_time_axis.measure);
                        new_note.note_off_beat = Some(last_time_axis.beat);
                        new_note.note_off_tick = Some(last_time_axis.tick);
                        new_note.note_off_tick_reset_by_measure =
                            Some(last_time_axis.tick_reset_by_measure);
                        new_note.note_length_ticks =
                            Some(last_time_axis.ticks_index - new_note.note_on_ticks);

                        new_notes.push(new_note);

                        // current_noteを修正
                        let first_time_axis = time_axis
                            .iter()
                            .find(|x| {
                                x.measure == current_measure + 1 && x.beat == 1 && x.tick == 0
                            })
                            .unwrap();
                        current_note.note_on_ticks = first_time_axis.ticks_index;
                        current_note.note_on_seconds = Some(first_time_axis.seconds);
                        current_note.note_on_measure = Some(first_time_axis.measure);
                        current_note.note_on_beat = Some(first_time_axis.beat);
                        current_note.note_on_tick = Some(first_time_axis.tick);
                        current_note.note_on_tick_reset_by_measure =
                            Some(first_time_axis.tick_reset_by_measure);
                        current_note.note_length_ticks = Some(
                            current_note.note_off_ticks.unwrap() - first_time_axis.ticks_index,
                        );
                        current_note.measure_length_ticks =
                            Some(first_time_axis.measure_length_ticks);
                    }
                }
            }
        }

        new_midi_notes.push(new_notes);
    }

    // for debug print ch1 notes
    // for note in &new_midi_notes[0] {
    //     println!(
    //         "Note: {}, Velocity: {}, Channel: {}, OnTicks: {}, OffTicks: {}",
    //         note.key_and_octave_yamaha,
    //         note.velocity,
    //         note.channel,
    //         note.note_on_ticks,
    //         note.note_off_ticks.unwrap()
    //     );
    // }

    return GetMidiNotesReturn {
        midi_notes: new_midi_notes,
    };
}
