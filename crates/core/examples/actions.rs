use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use rrr_core::{
    note::{self, CompiledNote, Direction},
    play::{self, Difference},
};

#[derive(Debug)]
struct JudgeWindow(i8);
static JUDGE: &[JudgeWindow] = &[
    JudgeWindow(-118),
    JudgeWindow(-85),
    JudgeWindow(-51),
    JudgeWindow(-18),
    JudgeWindow(17),
    JudgeWindow(50),
    JudgeWindow(84),
    JudgeWindow(117),
];

#[derive(Debug)]
enum Accuracy {
    Amazing,
    Perfect,
    Good,
    Average,
    Miss,
    Boo,
}

#[derive(Debug)]
struct Judgement<'a> {
    note: Option<&'a CompiledNote>,
    accuracy: Accuracy,
    timestamp: Option<Duration>,
}

fn main() {
    let view = vec![
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: 40,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: 44,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: 80,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: 110,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: 160,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: 180,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Left,
            timestamp: 240,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Down,
            timestamp: 240,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Down,
            timestamp: 260,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Down,
            timestamp: 270,
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Down,
            timestamp: 350,
        },
    ];

    let current_receptor_ms_position = 160u64;

    // "Keyboard input"
    let key_actions = [
        (note::Direction::Up, current_receptor_ms_position),
        (note::Direction::Left, current_receptor_ms_position),
        (note::Direction::Down, current_receptor_ms_position),
    ];

    // Need the judgement code to be in here. Probably don't need "Note Actions" just judgements /w associated note. So a NoteActionBuilder produces a judgement.

    // Calculate missed notes, build a miss action for those notes, hash-map for processed notes.
    let missed_notes: HashSet<CompiledNote> = view
        .iter()
        .filter(|note| note.timestamp < current_receptor_ms_position - i8::abs(JUDGE[0].0 as u64))
        .cloned()
        .collect();

    println!("Missed notes: {:?}", missed_notes);

    // For each key action, filter the lane.
    // For each lane, find the note with the lowest timestamp.
    // Build a hit key action for that note.

    // Calculate misses, and flag.
    // Problem: Given a view of N notes, and a set of "key actions":
    // -- In the lane declared in the "key action", which note in either direction is closest, and has not been "actioned" yet?

    // Steps:
    // 1. For view, calculate misses and flag.
    // 2. For each key action, find the lane, find the earliest note, and build a NoteAction.
    // 3. If no note within actionable range, build a BooAction.

    for note in view {
        if !missed_notes.contains(&note) {
            let diff = note.timestamp.diff(&current_receptor_ms_position);

            let judge = {
                let mut last_judge = None;
                for judge in JUDGE {
                    if diff > judge.0.into() {
                        last_judge.replace(judge);
                    }
                }
                last_judge
            };

            if let Some(judge) = judge {
                println!("{:?}", judge);
            }
        }
    }
}
