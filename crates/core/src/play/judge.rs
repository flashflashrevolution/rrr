use crate::note::CompiledNote;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct JudgeWindow(pub i128);
pub static JUDGE: [JudgeWindow; 8] = [
    JudgeWindow(-118),
    JudgeWindow(-85),
    JudgeWindow(-51),
    JudgeWindow(-18),
    JudgeWindow(17),
    JudgeWindow(50),
    JudgeWindow(84),
    JudgeWindow(117),
];

pub type Judgement = HashMap<CompiledNote, JudgeWindow>;

#[derive(Debug, Clone)]
pub struct Judge {
    pub judgements: Judgement,
    pub misses: HashSet<CompiledNote>,
    pub receptor_position: i128,
}

impl Judge {
    /// Creates a new [`Judge`].
    #[must_use]
    pub fn new(receptor_position: i128) -> Self {
        Self {
            judgements: HashMap::default(),
            misses: HashSet::default(),
            receptor_position,
        }
    }

    pub fn judge(&mut self, current_timestamp: i128, closest_note: &CompiledNote) {
        if !self.misses.contains(&closest_note) && !self.judgements.contains_key(&closest_note) {
            let diff = current_timestamp - closest_note.timestamp;

            let judge = {
                let mut last_judge = None;
                for judge in JUDGE {
                    if diff > judge.0.into() {
                        last_judge.replace(judge);
                    }
                }
                last_judge
            };

            if let Some(some_judge) = judge {
                log::info!("{:?} || {:?}", closest_note.timestamp, some_judge);
                let local_note = closest_note.clone();
                self.judgements.insert(local_note, some_judge);
            } else {
                log::info!("BOOOOO");
            }
        }
    }
}

pub struct JudgementReport {
    pub amazings: u32,
    pub perfects: u32,
    pub goods: u32,
    pub averages: u32,
    pub misses: u32,
    pub boos: u32,
}

impl JudgementReport {
    #[must_use]
    pub fn new(judge: Judge) -> Self {
        todo!();
    }
}

// Create a judge which lives for the curation of a play session.

// Expectations:
// Currently game receives a keyboard event and triggers a note action and generates a judgement.
// The judge, could be a class which holds information regarding how judgements are to be calculated.
// "Note Actions" are derived from finding the note, in the associated lane, nearest to the current receptor position.
// Judgement is calculated based on the difference between the current receptor position and the note's timestamp.
// The judgement is then stored in the judgement vector.
// Each note can only be judged once.

// Alterantive:
// Keyboard event is triggered by the player.
// A timestamp is collected for the event.
// Judgement is immediately calculated based on the current view.
// Judgement can either be a HIT, with an associated note, or a boo, with no associated not.
// Every judgement is stored in the judgement vector, any judgement with an associated note is also added to a hashset.
