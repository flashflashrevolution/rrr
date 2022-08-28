use rrr_core::{
    chart::{NoteColor, NoteDirection, RuntimeNote},
    play::judge,
};
use std::collections::HashSet;

fn main() {
    let view = vec![
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Up,
            timestamp: 40,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Up,
            timestamp: 44,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Up,
            timestamp: 80,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Up,
            timestamp: 110,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Up,
            timestamp: 160,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Up,
            timestamp: 180,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Left,
            timestamp: 240,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Down,
            timestamp: 240,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Down,
            timestamp: 260,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Down,
            timestamp: 270,
        },
        RuntimeNote {
            beat_position: 0,
            color: NoteColor::Red,
            direction: NoteDirection::Down,
            timestamp: 350,
        },
    ];

    let current_receptor_ms_position = 160u64;

    let key_actions = [
        (NoteDirection::Up, current_receptor_ms_position),
        (NoteDirection::Left, current_receptor_ms_position),
        (NoteDirection::Down, current_receptor_ms_position),
    ];

    let missed_view = view.clone();
    let missed_notes: HashSet<&RuntimeNote> = missed_view
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
                for judge in judge::JUDGE {
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
