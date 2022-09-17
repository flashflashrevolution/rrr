use super::record::Record;
use crate::{audio::AudioPlayer, chart::RuntimeNote};
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
    pub progress: u32,
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

    pub fn tick(&mut self, progress: u32) {
        self.state.progress = progress;
        if let Some(player) = self.state.audio_player.borrow_mut() {
            player.tick();
        }
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.state.progress >= self.record.duration.into()
    }

    #[must_use]
    pub fn progress(&self) -> u32 {
        self.state.progress
    }

    pub fn view(&self, look_behind: u32, look_ahead: u32) -> MultiRange<'_, u32, RuntimeNote> {
        let chart = &self.record.optimized_chart;

        let first =
            if let Some(first_value) = self.state.progress.checked_sub(u32::from(look_behind)) {
                first_value
            } else {
                self.state.progress
            };

        chart.range((Included(first), Included(self.state.progress + look_ahead)))
    }
}
