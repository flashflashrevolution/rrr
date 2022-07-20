use std::marker::PhantomData;

use crate::tape::Tape;

pub struct TapeDeckData {
    tape: Option<Tape>,
}

pub struct TapeDeck<S: TapeDeckState> {
    state: TapeDeckData,
    marker: std::marker::PhantomData<S>,
}

pub struct Empty;
pub struct Loaded;
pub struct Playing;

pub trait TapeDeckState {}
impl TapeDeckState for Empty {}
impl TapeDeckState for Loaded {}
impl TapeDeckState for Playing {}

impl Default for TapeDeck<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl TapeDeck<Empty> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: TapeDeckData { tape: None },
            marker: PhantomData::<Empty>,
        }
    }

    #[must_use]
    pub fn load(self, tape: Tape) -> TapeDeck<Loaded> {
        let mut deck = TapeDeck {
            state: self.state,
            marker: PhantomData::<Loaded>,
        };
        deck.state.tape = Some(tape);
        deck
    }
}

impl TapeDeck<Loaded> {
    #[must_use]
    pub fn play(self) -> TapeDeck<Playing> {
        TapeDeck {
            state: self.state,
            marker: PhantomData::<Playing>,
        }
    }

    #[must_use]
    pub fn eject(self) -> TapeDeck<Empty> {
        let mut deck = TapeDeck {
            state: self.state,
            marker: PhantomData::<Empty>,
        };
        deck.state.tape = None;
        deck
    }
}

impl TapeDeck<Playing> {
    #[must_use]
    pub fn stop(self) -> TapeDeck<Loaded> {
        TapeDeck {
            state: self.state,
            marker: PhantomData::<Loaded>,
        }
    }
}
