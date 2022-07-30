use std::time::Duration;

use rrr_core::{
    note::{self, CompiledNote},
    play::{self, Difference},
};

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
    let view = [
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: Duration::from_millis(40),
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: Duration::from_millis(80),
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: Duration::from_millis(110),
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: Duration::from_millis(160),
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Up,
            timestamp: Duration::from_millis(180),
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Left,
            timestamp: Duration::from_millis(240),
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Down,
            timestamp: Duration::from_millis(240),
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Down,
            timestamp: Duration::from_millis(260),
        },
        note::CompiledNote {
            beat_position: 0,
            color: note::Color::Red,
            direction: note::Direction::Down,
            timestamp: Duration::from_millis(270),
        },
    ];

    let current_receptor_ms_position = 160u64;

    // "Keyboard input"
    let key_actions = [
        (
            note::Direction::Up,
            Duration::from_millis(current_receptor_ms_position),
        ),
        (
            note::Direction::Up,
            Duration::from_millis(current_receptor_ms_position),
        ),
        (
            note::Direction::Left,
            Duration::from_millis(current_receptor_ms_position),
        ),
        (
            note::Direction::Down,
            Duration::from_millis(current_receptor_ms_position),
        ),
        (
            note::Direction::Down,
            Duration::from_millis(current_receptor_ms_position),
        ),
    ];

    // Need the judgement code to be in here. Probably don't need "Note Actions" just judgements /w associated note. So a NoteActionBuilder produces a judgement.

    // Calculate missed notes, build a miss action for those notes, hash-map for processed notes.
    let missed_notes = view
        .iter()
        .filter(|note| {
            note.timestamp
                < Duration::from_millis(current_receptor_ms_position - i8::abs(JUDGE[0].0) as u64)
        })
        .collect::<Vec<&CompiledNote>>();

    println!("Missed notes: {:?}", missed_notes);

    // For each key action, filter the lane.
    // For each lane, find the note with the lowest timestamp.
    // Build a hit key action for that note.

    // Calculate misses, and flag.
    let mut note_actions = Vec::<Judgement>::new();

    // Problem: Given a view of N notes, and a set of "key actions":
    // -- In the lane declared in the "key action", which note in either direction is closest, and has not been "actioned" yet?

    // Steps:
    // 1. For view, calculate misses and flag.
    // 2. For each key action, find the lane, find the earliest note, and build a NoteAction.
    // 3. If no note within actionable range, build a BooAction.

    for (direction, ts) in key_actions {
        // let diff = note.timestamp.diff(&self.timestamp);
        // // Create definition list for offsets. Hard code for now.
        // let judge = {
        //     let mut last_judge = None;
        //     for judge in JUDGE {
        //         if diff > judge.0 {
        //             last_judge.replace(judge);
        //         }
        //     }
        //     last_judge
        // };

        // Filter out notes that are past the current timestamp.
        let closest_note = view
            .iter()
            .filter(|note| direction == note.direction)
            .filter(|note| !missed_notes.contains(note))
            .collect::<Vec<&CompiledNote>>();

        print!("{:?}", closest_note);

        //note_actions.push(closest_note);
    }

    for action in note_actions {
        println!("{:?}", action);
    }
}
