use std::time::Duration;

use crate::{
    note::{Color, CompiledNote, Direction},
    turntable, Turntable,
};

pub struct PlayStats {
    amazings: u32,
    perfects: u32,
    goods: u32,
    averages: u32,
    misses: u32,
    boos: u32,
}

impl PlayStats {
    #[must_use]
    pub fn default() -> Self {
        Self {
            amazings: 0,
            perfects: 0,
            goods: 0,
            averages: 0,
            misses: 0,
            boos: 0,
        }
    }
}

pub struct NoteAction {
    note: CompiledNote,
    timestamp: Duration,
}

pub struct Play<S: PlayState> {
    stats: PlayStats,
    state: S,
}

pub struct Ready {
    turntable: Turntable<turntable::Loaded>,
}

pub struct Active {
    turntable: Turntable<turntable::Playing>,
    actions: Vec<NoteAction>,
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
                actions: Vec::default(),
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
    #[must_use]
    pub fn view(&self) -> &[CompiledNote] {
        self.state.turntable.view()
    }

    pub fn tick(&mut self, _timestamp: Duration) {
        // gameplay logic
        self.state.turntable.tick();
        let chart_view = self.state.turntable.view();

        // Small state machine that controls whether the music should be started.
        // I can use the tape deck for this _maybe_, but for now could just have a bool.

        // game starts in play mode, notes should begin moving as soon as the player hits the spacebar.
        // notes are not consumable right now, no accuracy no key presses.
        // Remove arrow from the render list when it:
        // -- moves past the end of the screen.

        // TODO: Spawn arrows and begin to move them up the field at delta rate.
        // - Get chart.
        // - Spawn arrows in order based on time-to-target offset. (See how we do this in R^3).
        // - Destroy arrows when they hit the top of the screen.
        // - Destroy arrows when they are on a recepor when the player activates it.
    }

    pub fn do_action(&mut self, direction: Direction, ts: Duration) {
        let view = self.state.turntable.view();

        if let Some(closest_note) = view
            .iter()
            .filter(|note| direction == note.direction)
            .min_by(|x, y| x.timestamp.diff(&ts).cmp(&y.timestamp.diff(&ts)))
        {
            self.state.actions.push(NoteAction {
                note: closest_note.clone(),
                timestamp: ts,
            });
        } else {
            self.state.actions.push(NoteAction {
                note: CompiledNote {
                    beat_position: -1,
                    color: Color::Receptor,
                    direction,
                    timestamp: ts,
                },
                timestamp: ts,
            });
        }

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
    fn diff(self, right: &Duration) -> Self;
}

impl Difference for Duration {
    fn diff(self, right: &Duration) -> Duration {
        if self < *right {
            *right - self
        } else {
            self - *right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getting_the_difference_between_durations() {
        let ts0 = Duration::from_millis(100);
        let ts1 = Duration::from_millis(130);

        assert_eq!(ts0.diff(&ts1), Duration::from_millis(30));
        assert_eq!(ts1.diff(&ts0), Duration::from_millis(30));
    }

    #[test]
    fn find_closest_duration() {
        let ts0 = Duration::from_millis(100);
        let ts1 = Duration::from_millis(130);
        let ts2 = Duration::from_millis(150);
        let ts3 = Duration::from_millis(170);
        let ts_list = vec![ts0, ts1, ts2, ts3];

        let target_ts = Duration::from_millis(140);

        #[allow(clippy::expect_used)]
        let closest_note = ts_list
            .iter()
            .min_by(|x, y| x.diff(&target_ts).cmp(&y.diff(&target_ts)))
            .expect("no closest note found");

        assert_eq!(closest_note, &ts1);
    }
}
