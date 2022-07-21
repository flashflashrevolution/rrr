use rrr::{Chart, CompiledChart, Record, Turntable};

fn main() {
    let chart = Chart::default();
    let compiled_chart: CompiledChart = chart.compile();
    let tape = Record::new(vec![], compiled_chart);

    let loaded_turntable = Turntable::load(tape);
    let playing_turntable = loaded_turntable.play();
    let _stopped_turntable = playing_turntable.stop();
}
