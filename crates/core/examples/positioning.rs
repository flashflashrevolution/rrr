// use rrr_core::{download_chart, SwfParser, Turntable};

use std::thread::current;

use rrr_core::{
    chart::{NoteColor, NoteDirection, RuntimeChart, RuntimeNote},
    math::lerp,
    play::{self, record::Record, turntable::Turntable},
};

fn main() {
    let notes = [
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Yellow,
            direction: NoteDirection::Right,
            timestamp: 14000,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Down,
            timestamp: 10000,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Left,
            timestamp: 2000,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Up,
            timestamp: 2000,
        },
    ];

    let chart = RuntimeChart::new(&notes);
    let mut playing_turntable = if let Ok(record) = Record::new(Vec::new(), chart) {
        let loaded_turntable = Turntable::load(record);
        loaded_turntable.play()
    } else {
        panic!("Failed to start playing turntable.")
    };

    let progress_start = 1.0;
    let progress_end = -0.1;

    let look_ahead = 2000;
    let look_behind = 200;

    let delta = 1000;
    let mut delta_accumulator = 0;
    while !playing_turntable.is_finished() {
        playing_turntable.tick(delta_accumulator);
        let current_progress = playing_turntable.progress();
        let view = playing_turntable.view(look_behind, look_ahead);
        println!("Progress: {:?} || View: {:?}", current_progress, view);

        delta_accumulator += delta;
    }

    //         while !playing_turntable.is_finished() {
    //             playing_turntable.tick(delta);
    //             let view = playing_turntable.view(time_on_screen);
    //             let chart_progress = playing_turntable.progress();

    //             for (duration, note) in view {
    //                 let note_progress = duration.as_millis() as u64 - chart_progress;
    //                 let normalized = note_progress as f64 / time_on_screen as f64;
    //                 let position = end_position.lerp(start_position, normalized);

    //                 if note.beat_position == 1183 {
    //                     println!("pos: {:?} {:.0}", note.beat_position, position);
    //                 }
    //             }
    //         }
}
