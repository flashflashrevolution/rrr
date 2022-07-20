#![allow(dead_code)]

use std::marker::PhantomData;

use rrr::CompiledChart;

struct Tape {
    mp3: Vec<u8>,
    chart: CompiledChart,
}

impl Tape {
    fn new(mp3: Vec<u8>, chart: CompiledChart) -> Self {
        Self { mp3, chart }
    }
}

struct TapeDeckData {
    tape: Option<Tape>,
}

struct TapeDeck<S: TapeDeckState> {
    state: TapeDeckData,
    marker: std::marker::PhantomData<S>,
}

struct Empty;
struct Loaded;
struct Playing;

trait TapeDeckState {}
impl TapeDeckState for Empty {}
impl TapeDeckState for Loaded {}
impl TapeDeckState for Playing {}

impl TapeDeck<Empty> {
    fn new() -> Self {
        Self {
            state: TapeDeckData { tape: None },
            marker: PhantomData::<Empty>,
        }
    }

    fn load(self, tape: Tape) -> TapeDeck<Loaded> {
        TapeDeck {
            state: TapeDeckData { tape: Some(tape) },
            marker: PhantomData::<Loaded>,
        }
    }
}

impl TapeDeck<Loaded> {
    fn play(self) -> TapeDeck<Playing> {
        TapeDeck {
            state: self.state,
            marker: PhantomData::<Playing>,
        }
    }

    fn remove(self) -> TapeDeck<Empty> {
        TapeDeck {
            state: TapeDeckData { tape: None },
            marker: PhantomData::<Empty>,
        }
    }
}

impl TapeDeck<Playing> {
    fn stop(self) -> TapeDeck<Loaded> {
        TapeDeck {
            state: self.state,
            marker: PhantomData::<Loaded>,
        }
    }
}

fn main() {
    let chart = rrr::Chart::default();
    let compiled_chart: CompiledChart = chart.compile();
    let tape = Tape::new(vec![], compiled_chart);

    let tape_deck = TapeDeck::new();
    let loaded_tape_deck = tape_deck.load(tape);
    let playing_tape_deck = loaded_tape_deck.play();
    let _stopped_tape_deck = playing_tape_deck.stop();
}
