use crate::{note::CompiledNote, record::Record, AudioPlayer};
use btreemultimap::MultiRange;
use std::{borrow::BorrowMut, ops::Bound::Included};

#[derive(Clone)]
pub struct Turntable<S: TurntableState> {
    record: Record,
    state: S,
}

pub struct Empty {}

pub struct Loaded {}

pub struct Playing {
    pub progress: u64,
    audio_player: Option<AudioPlayer>,
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
    /// Start playing the record on the turntable.
    #[must_use]
    pub fn play(self) -> Turntable<Playing> {
        Turntable {
            record: self.record,
            state: Playing {
                progress: 0,
                audio_player: None,
            },
        }
    }

    /// Start playing the record on the turntable.
    ///
    /// # Panics
    ///
    /// If the mp3 data is malformed.
    #[must_use]
    pub fn play_with_audio(self) -> Turntable<Playing> {
        let mut turntable = Turntable {
            record: self.record,
            state: Playing {
                progress: 0,
                audio_player: None,
            },
        };

        turntable.state.audio_player =
            Some(AudioPlayer::try_new(turntable.record.mp3.as_slice()).unwrap());

        turntable
    }
}

impl Turntable<Playing> {
    #[must_use]
    pub fn stop(self) -> Turntable<Loaded> {
        if let Some(mut player) = self.state.audio_player {
            player.stop();
        }

        Turntable {
            record: self.record,
            state: Loaded {},
        }
    }

    pub fn tick(&mut self, progress: u64) {
        self.state.progress = progress;
        if let Some(player) = self.state.audio_player.borrow_mut() {
            player.tick();
        }
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
