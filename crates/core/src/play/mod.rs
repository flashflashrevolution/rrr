pub mod actions;
pub mod field;
pub mod judge;

use self::{
    actions::NoteAction,
    field::Field,
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
    field: Field,
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
    pub fn new(turntable: Turntable<turntable::Loaded>, field: Field) -> Self {
        Self {
            state: Ready { turntable },
            settings: Settings::default(),
            field,
        }
    }

    #[must_use]
    pub fn with_settings(self, settings: Settings) -> Self {
        Self {
            field: self.field,
            state: self.state,
            settings,
        }
    }

    #[must_use]
    pub fn with_field(self, field: Field) -> Self {
        Self {
            field,
            state: self.state,
            settings: self.settings,
        }
    }

    #[must_use]
    pub fn start(self, judge_zero_point: i128) -> Play<Active> {
        Play {
            field: self.field,
            state: Active {
                turntable: self.state.turntable.play(),
                actions: BTreeMultiMap::default(),
                judge: Judge::new(judge_zero_point.try_into().unwrap()),
            },
            settings: self.settings,
        }
    }
}

impl Play<Active> {
    #[must_use]
    pub fn stop(self) -> Play<Ready> {
        Play {
            field: self.field,
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

    pub fn tick(&mut self, progress: u64) {
        self.state.turntable.tick(progress);
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
    pub fn field(&self) -> &Field {
        &self.field
    }

    #[must_use]
    pub fn judge_zero_point(&self) -> u32 {
        self.state.judge.judge_zero_point
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
        let is_within_judge_range = note
            .timestamp
            .abs_dif(&(ts + i128::from(self.state.judge.judge_zero_point)))
            <= 118;
        log::debug!(
            "note: {:?} || judge: {:?}",
            note.timestamp
                .abs_dif(&(ts + i128::from(self.state.judge.judge_zero_point))),
            118
        );
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
            field: self.field,
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

// tests
#[cfg(test)]
mod tests {
    use crate::math::lerp::Lerp;

    fn screen_pos_to_lerp_time() -> f64 {
        (-64.).inv_lerp(720., 64.)
    }

    fn lerp_time_to_screen_pos() -> f64 {
        (-64.).lerp(720., screen_pos_to_lerp_time()).round()
    }

    fn ms_time_from_screen_height_time_on_screen_and_position() -> f64 {
        let start_position = 720.;
        let end_position = -64.;
        let time_on_screen = 3000.;
        let judge_position = 64.;

        let normalized_note_progress = end_position.inv_lerp(start_position, judge_position);
        let ms: f64 = normalized_note_progress * time_on_screen;

        println!("normalized: {}", ms);
        ms.round()
    }

    #[test]
    fn test_screen_space_to_judgement_zero() {
        assert!(screen_pos_to_lerp_time() - 0.163_265_306_122_448_97 <= f64::EPSILON);
    }

    #[test]
    fn test_screen_lerp_time_to_screen_space() {
        assert!((lerp_time_to_screen_pos() - 64.).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ms_time_from_screen_height_time_on_screen_and_position() {
        assert!(
            (ms_time_from_screen_height_time_on_screen_and_position() - 490.0).abs() < f64::EPSILON
        );
    }
}

// What I should really be doing is determining exactly what ratio is between this zero point and the note.
// So if a note has a ms timestamp of 2000, and the zero point is at 2000,
// how many milliseconds is the is the receptor before that. Ex. (2000 - 120)
