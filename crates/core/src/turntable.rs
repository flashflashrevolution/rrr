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
    pub progress: f64,
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
            state: Playing { progress: 0.0 },
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

    pub fn tick(&mut self, delta_time: f64) {
        self.state.progress += delta_time;
        if self.state.progress >= self.record.duration.as_secs_f64() {
            self.state.progress = 0.0;
        }
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.state.progress >= self.record.duration.as_secs_f64()
    }

    #[must_use]
    pub fn progress(&self) -> f64 {
        self.state.progress
    }

    // TODO: Implement a method of tracking how far the slice should be.
    pub fn view(&self) -> Range<'_, Duration, CompiledNote> {
        let chart = &self.record.optimized_chart;

        let extent = Duration::new(2, 0);
        chart.range((
            Included(Duration::from_secs_f64(self.state.progress)),
            Included(Duration::from_secs_f64(self.state.progress) + extent),
        ))
    }
}
