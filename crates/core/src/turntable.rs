use std::marker::PhantomData;

use crate::{note::CompiledNote, record::Record};

pub struct Turntable<S: TurntableState> {
    record: Record,
    marker: std::marker::PhantomData<S>,
}

pub struct Empty {}

pub struct Loaded {}

pub struct Playing {}

pub trait TurntableState {}
impl TurntableState for Empty {}
impl TurntableState for Loaded {}
impl TurntableState for Playing {}

impl Turntable<Empty> {
    #[must_use]
    pub fn load(record: Record) -> Turntable<Loaded> {
        Turntable {
            record,
            marker: PhantomData::<Loaded>,
        }
    }
}

impl Turntable<Loaded> {
    #[must_use]
    pub fn play(self) -> Turntable<Playing> {
        Turntable {
            record: self.record,
            marker: PhantomData::<Playing>,
        }
    }
}

impl Turntable<Playing> {
    #[must_use]
    pub fn stop(self) -> Turntable<Loaded> {
        Turntable {
            record: self.record,
            marker: PhantomData::<Loaded>,
        }
    }

    pub fn tick(&mut self) {
        // Here would be positional information about the current position in the record.
    }

    // TODO: Implement a method of tracking how far the slice should be.
    #[must_use]
    pub fn view(&self) -> &[CompiledNote] {
        let chart = self.record.chart();
        let first: usize = 0;
        let last = usize::min(50, chart.notes.len());
        &chart.notes[first..last]
    }
}
