pub mod actions;
pub mod judge;
pub mod stats;

use self::{
    actions::{ActionState, NoteAction},
    judge::JudgeWindow,
    stats::PlayStats,
};
use crate::{
    note::{Color, CompiledNote, Direction},
    turntable, Turntable,
};
use btreemultimap::{BTreeMultiMap, MultiRange};
use std::collections::HashSet;

pub struct Play<S: PlayState> {
    stats: PlayStats,
    state: S,
}

pub struct Ready {
    turntable: Turntable<turntable::Loaded>,
}

#[derive(Clone)]
pub struct Active {
    turntable: Turntable<turntable::Playing>,
    actions: BTreeMultiMap<CompiledNote, NoteAction>,
    missed: HashSet<CompiledNote>,
    judgements: Vec<&'static JudgeWindow>,
}

pub struct Concluded {
    tape_deck: Turntable<turntable::Loaded>,
    actions: Vec<NoteAction>,
}

pub trait PlayState {}
impl PlayState for Ready {}
impl PlayState for Active {}
impl PlayState for Concluded {}

impl Play<Ready> {
    #[must_use]
    pub fn new(turntable: Turntable<turntable::Loaded>) -> Self {
        Self {
            stats: PlayStats::default(),
            state: Ready { turntable },
        }
    }

    #[must_use]
    pub fn start(self) -> Play<Active> {
        Play {
            stats: PlayStats::default(),
            state: Active {
                turntable: self.state.turntable.play(),
                actions: BTreeMultiMap::default(),
                missed: HashSet::default(),
                judgements: Vec::default(),
            },
        }
    }

    #[must_use]
    pub fn stats(&self) -> &PlayStats {
        &self.stats
    }
}

impl Play<Active> {
    #[must_use]
    pub fn stop(self) -> Play<Ready> {
        Play {
            stats: self.stats,
            state: Ready {
                turntable: self.state.turntable.stop(),
            },
        }
    }

    /// Temporary function giving a view directly into the playing turntable.
    ///
    /// Remove this after we create the `ChartDriver`.
    /// # Errors
    /// Turntable could slice into an invalid set of notes.
    #[must_use]
    pub fn view(&self, range_in_milliseconds: u64) -> MultiRange<'_, i128, CompiledNote> {
        self.state.turntable.view(range_in_milliseconds.into())
    }

    #[must_use]
    pub fn progress(&self) -> u64 {
        self.state.turntable.progress()
    }

    #[must_use]
    pub fn missed_notes(&self) -> &HashSet<CompiledNote> {
        &self.state.missed
    }

    #[must_use]
    pub fn actions(&self) -> &BTreeMultiMap<CompiledNote, NoteAction> {
        &self.state.actions
    }

    pub fn tick(&mut self, delta_time: u64) {
        self.state.turntable.tick(delta_time);
        self.check_miss();

        // TODO: Calculate and store a judgement when the player activates a receptor, and a note is near it.
    }

    fn check_miss(&mut self) {
        let current_receptor_position =
            self.progress() as i128 + i128::abs(judge::JUDGE[0].0 as i128);
        let mapped_notes = self
            .view(3600)
            .filter(|(&ts, _)| current_receptor_position > ts)
            .map(|(_, note)| note.clone());

        self.state
            .missed
            .extend(mapped_notes.collect::<HashSet<CompiledNote>>());
    }

    fn judge(&mut self, note_action: &NoteAction) {
        let current_receptor_position =
            self.progress() as i128 + i128::abs(judge::JUDGE[0].0 as i128);

        if !self.state.missed.contains(&note_action.note) {
            let diff = current_receptor_position - note_action.timestamp;

            let judge = {
                let mut last_judge = None;
                for judge in judge::JUDGE {
                    if diff > judge.0.into() {
                        last_judge.replace(judge);
                    }
                }
                last_judge
            };

            if let Some(some_judge) = judge {
                log::info!("{:?} || {:?}", note_action.timestamp, some_judge);
                self.state.judgements.push(some_judge);
            }
        }
    }

    #[must_use]
    pub fn judgements(&self) -> &Vec<&'static JudgeWindow> {
        &self.state.judgements
    }

    pub fn do_action(&mut self, direction: Direction, ts: i128) {
        let play = self.state.clone();
        let view_result = play.turntable.view(2000);
        if let Some((_, closest_note)) = view_result
            .filter(|(_, note)| direction == note.direction)
            .min_by(|(_, x_note), (_, y_note)| {
                x_note
                    .timestamp
                    .abs_dif(&ts)
                    .cmp(&y_note.timestamp.abs_dif(&ts))
            })
        {
            let note_action = NoteAction {
                note: closest_note.clone(),
                timestamp: ts,
                state: ActionState::Hit,
            };

            self.judge(&note_action);

            self.state.actions.insert(
                closest_note.clone(),
                NoteAction {
                    note: closest_note.clone(),
                    timestamp: ts,
                    state: ActionState::Hit,
                },
            );
        } else {
            let note = CompiledNote {
                beat_position: -1,
                color: Color::Receptor,
                direction,
                timestamp: ts,
            };
            self.state.actions.insert(
                note.clone(),
                NoteAction {
                    note,
                    timestamp: ts,
                    state: ActionState::Boo,
                },
            );
        }

        // TODO: Result and Optional need to be managed better here.
        // Possibility of invalid chart during gameplay is not good.

        // self.state.actions.push(NoteAction {
        //     note,
        //     timestamp: ts,
        // });
    }

    // fn build_note_action(&self, note: Direction, ts: Duration) -> NoteAction {
    //     let ayo = &self.state.turntable;
    //     NoteAction {
    //         note,
    //         timestamp: ts,
    //     }
    // }
}

impl Play<Concluded> {
    #[must_use]
    pub fn actions(&self) -> &Vec<NoteAction> {
        &self.state.actions
    }

    #[must_use]
    pub fn stats(&self) -> &PlayStats {
        &self.stats
    }

    #[must_use]
    pub fn finalize(self) -> Play<Ready> {
        Play {
            stats: PlayStats::default(),
            state: Ready {
                turntable: self.state.tape_deck,
            },
        }
    }
}

pub trait Difference {
    #[must_use]
    fn abs_dif(self, right: &i128) -> Self;

    #[must_use]
    fn diff(self, right: &i128) -> i128;
}

impl Difference for i128 {
    fn abs_dif(self, right: &i128) -> i128 {
        if self < *right {
            *right - self
        } else {
            self - *right
        }
    }

    fn diff(self, right: &i128) -> i128 {
        self - right
    }
}
