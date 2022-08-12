pub mod actions;
pub mod judge;

use self::{
    actions::NoteAction,
    judge::{Judge, Judgement},
};
use crate::{
    note::{CompiledNote, Direction},
    settings::Settings,
    turntable, Turntable,
};
use btreemultimap::{BTreeMultiMap, MultiRange};
use std::collections::HashSet;

pub struct Play<S: PlayState> {
    state: S,
    settings: Settings,
}

impl<S: PlayState> Play<S> {
    pub fn settings(&self) -> &Settings {
        &self.settings
    }
}

pub struct Ready {
    turntable: Turntable<turntable::Loaded>,
}

#[derive(Clone)]
pub struct Active {
    turntable: Turntable<turntable::Playing>,
    actions: BTreeMultiMap<CompiledNote, NoteAction>,
    judge: Judge,
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
            state: Ready { turntable },
            settings: Settings::default(),
        }
    }

    #[must_use]
    pub fn with_settings(self, settings: Settings) -> Self {
        Self {
            state: self.state,
            settings,
        }
    }

    #[must_use]
    pub fn start(self) -> Play<Active> {
        Play {
            state: Active {
                turntable: self.state.turntable.play(),
                actions: BTreeMultiMap::default(),
                judge: Judge::new(200),
            },
            settings: self.settings,
        }
    }
}

impl Play<Active> {
    #[must_use]
    pub fn stop(self) -> Play<Ready> {
        Play {
            state: Ready {
                turntable: self.state.turntable.stop(),
            },
            settings: self.settings,
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
        &self.state.judge.misses
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
        let song_progress = self.progress() as i128;

        let state = self.state.clone();
        let mapped_notes = state
            .turntable
            .view(120)
            .filter(|(&ts, note)| song_progress >= ts + 118 && !state.judge.misses.contains(note))
            .map(|(_, note)| note.clone());

        self.state
            .judge
            .misses
            .extend(mapped_notes.collect::<HashSet<CompiledNote>>());
    }

    #[must_use]
    pub fn judgements(&self) -> &Judgement {
        &self.state.judge.judgements
    }

    pub fn do_action(&mut self, direction: Direction, ts: i128) {
        let play = self.state.clone();
        let view_result = play.turntable.view(500);
        if let Some((_, closest_note)) = view_result
            .filter(|(_, note)| self.determine_judgable(note, &direction, ts))
            .next()
        {
            self.state.judge.judge(ts, closest_note);
        }
    }

    fn determine_judgable(&self, note: &CompiledNote, direction: &Direction, ts: i128) -> bool {
        let is_judged = self.state.actions.contains_key(note);
        let is_same_direction = *direction == note.direction;
        let is_within_judge_range = note.timestamp.abs_dif(&ts) <= 118;
        !is_judged && is_same_direction && is_within_judge_range
    }
}

impl Play<Concluded> {
    #[must_use]
    pub fn actions(&self) -> &Vec<NoteAction> {
        &self.state.actions
    }

    #[must_use]
    pub fn finalize(self) -> Play<Ready> {
        Play {
            state: Ready {
                turntable: self.state.tape_deck,
            },
            settings: self.settings,
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
