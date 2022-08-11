// use rrr_core::{download_chart, SwfParser, Turntable};

// fn main() {
//     if let Some(raw_chart) = download_chart(3348) {
//         let parser_compressed = SwfParser::new(*raw_chart);
//         let record = if let Ok(ready_to_parse) = parser_compressed.decompress() {
//             let parsing = ready_to_parse.parse();
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

//         use lerp::Lerp;

//         let field_height = 768.0;
//         let note_height = 64.0;
//         let notes_in_field = field_height / note_height;
//         println!("field_height: {}", field_height);
//         println!("note_height: {}", note_height);
//         println!("notes_in_field: {}", notes_in_field);

//         let lane_offset = 96.0;
//         println!("lane_offset: {}", lane_offset);

//         let start_position = field_height;
//         let end_position = -note_height;

//         let time_on_screen = 2000;
//         let delta = 16;

//         while !playing_turntable.is_finished() {
//             playing_turntable.tick(delta);
//             let view = playing_turntable.view(time_on_screen);
//             let chart_progress = playing_turntable.progress();

//             for (duration, note) in view {
//                 let note_progress = duration.as_millis() as u64 - chart_progress;
//                 let normalized = note_progress as f64 / time_on_screen as f64;
//                 let position = end_position.lerp(start_position, normalized);

//                 if note.beat_position == 1183 {
//                     println!("pos: {:?} {:.0}", note.beat_position, position);
//                 }
//             }
//         }
//     }
// }

fn main() {}
