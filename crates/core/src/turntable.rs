use crate::{note::CompiledNote, record::Record};
use std::ops::Bound::Included;
use std::{collections::btree_map::Range, time::Duration};

pub struct Turntable<S: TurntableState> {
    record: Record,
    state: S,
}

pub struct Empty {}

pub struct Loaded {}

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

    pub fn tick(&mut self, delta_time: u64) {
        self.state.progress += delta_time;
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.state.progress >= self.record.duration.as_millis() as u64
    }

    #[must_use]
    pub fn progress(&self) -> u64 {
        self.state.progress
    }

    pub fn view(&self, range_in_milliseconds: u64) -> Range<'_, Duration, CompiledNote> {
        let chart = &self.record.optimized_chart;

        let extent = Duration::from_millis(range_in_milliseconds);
        chart.range((
            Included(Duration::from_millis(self.state.progress)),
            Included(Duration::from_millis(self.state.progress) + extent),
        ))
    }
}
