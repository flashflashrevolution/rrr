use rrr_core::{Chart, CompiledChart, Record, Turntable};

fn main() {
    let chart = Chart::default();
    let compiled_chart: CompiledChart = chart.compile();
    let record = Record::new(vec![], compiled_chart);

    let loaded_turntable = Turntable::load(record);
    let playing_turntable = loaded_turntable.play();
    let _stopped_turntable = playing_turntable.stop();
}
