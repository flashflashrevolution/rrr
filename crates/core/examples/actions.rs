use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use rrr_core::{
    note::{self, CompiledNote, Direction},
    play::{self, judge, Difference},
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
    timestamp: Option<i128>,
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

    let key_actions = [
        (note::Direction::Up, current_receptor_ms_position),
        (note::Direction::Left, current_receptor_ms_position),
        (note::Direction::Down, current_receptor_ms_position),
    ];

    let missed_view = view.clone();
    let missed_notes: HashSet<&CompiledNote> = missed_view
        .iter()
        .filter(|&note| {
            current_receptor_ms_position as i128 + i128::abs(judge::JUDGE[0].0 as i128)
                > note.timestamp
        })
        .collect();

    println!("Missed notes: {:?}", missed_notes);

    for note in view {
        if !missed_notes.contains(&note) {
            let diff = current_receptor_ms_position as i128 - note.timestamp;

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
