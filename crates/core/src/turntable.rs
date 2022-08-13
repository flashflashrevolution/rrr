use crate::{note::CompiledNote, record::Record};
use btreemultimap::MultiRange;
use std::ops::Bound::Included;

#[derive(Clone)]
pub struct Turntable<S: TurntableState> {
    record: Record,
    state: S,
}

pub struct Empty {}

pub struct Loaded {}

#[derive(Clone)]
pub struct Playing {
    pub progress: u64,
}

pub trait TurntableState {}
impl TurntableState for Empty {}
impl TurntableState for Loaded {}
impl TurntableState for Playing {}

impl Turntable<Empty> {
    #[must_use]
    pub fn load(record: Record) -> Turntable<Loaded> {
        Turntable {
            record,
            state: Loaded {},
        }
    }
}

impl Turntable<Loaded> {
    #[must_use]
    pub fn play(self) -> Turntable<Playing> {
        Turntable {
            record: self.record,
            state: Playing { progress: 0 },
        }
    }
}

impl Turntable<Playing> {
    #[must_use]
    pub fn stop(self) -> Turntable<Loaded> {
        Turntable {
            record: self.record,
            state: Loaded {},
        }
    }

    pub fn tick(&mut self, progress: u64) {
        self.state.progress = progress;
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.state.progress as i128 >= self.record.duration
    }

    #[must_use]
    pub fn progress(&self) -> u64 {
        self.state.progress
    }

    pub fn view(&self, range_in_milliseconds: i128) -> MultiRange<'_, i128, CompiledNote> {
        let chart = &self.record.optimized_chart;
        chart.range((
            Included(self.state.progress as i128),
            Included(self.state.progress as i128 + range_in_milliseconds),
        ))
    }
}
