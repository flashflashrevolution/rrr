// use std::thread::sleep;

// use rrr_core::{download_chart, SwfParser, Turntable};

// fn main() {
//     if let Some(raw_chart) = download_chart(3348) {
//         let parser_compressed = SwfParser::new(*raw_chart);
//         let record = if let Ok(ready_to_parse) = parser_compressed.decompress() {
//             let parsing = ready_to_parse.parse();
//             // TODO: Make this async, remove intermediate state and just await it.
//             let parsed = parsing.tick();
//             Some(parsed.produce_tape())
//         } else {
//             None
//         };

//         let mut playing_turntable = if let Some(record) = record {
//             let loaded_turntable = Turntable::load(record);
//             loaded_turntable.play()
//         } else {
//             panic!("Failed to create record.");
//         };

//         let delta = 1000;
//         while !playing_turntable.is_finished() {
//             playing_turntable.tick(delta);
//             sleep(1.0);
//             println!(
//                 "Progress: {:?} || View: {:?}",
//                 playing_turntable.progress(),
//                 playing_turntable.view(delta * 2)
//             );
//         }
//     }
// }

fn main() {}
