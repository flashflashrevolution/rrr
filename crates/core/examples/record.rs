use rrr_core::{
    chart::{BinChart, RuntimeChart},
    play::{record::Record, turntable::Turntable},
};

fn main() {
    let chart = BinChart::default();
    let compiled_chart: RuntimeChart = chart.compile();
    if let Ok(record) = Record::new(vec![], compiled_chart) {
        let loaded_turntable = Turntable::load(record);
        let playing_turntable = loaded_turntable.play();
        let _stopped_turntable = playing_turntable.stop();
    }
}
