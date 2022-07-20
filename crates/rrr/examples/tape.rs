use rrr::{Chart, CompiledChart, Tape, TapeDeck};

fn main() {
    let chart = Chart::default();
    let compiled_chart: CompiledChart = chart.compile();
    let tape = Tape::new(vec![], compiled_chart);

    let tape_deck = TapeDeck::new();
    let loaded_tape_deck = tape_deck.load(tape);
    let playing_tape_deck = loaded_tape_deck.play();
    let _stopped_tape_deck = playing_tape_deck.stop();
}
