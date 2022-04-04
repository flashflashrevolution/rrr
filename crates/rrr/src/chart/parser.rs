use std::iter::Map;

use crate::Note;
use swf_parser::{
    swf_types::{Movie, Tag},
    SwfParseError,
};

use super::swf_types;

pub struct SwfParser {
    swf: Movie,
    mp3: Option<Vec<u8>>,
    chart: Option<Vec<Note>>,
}

impl SwfParser {
    pub fn new(swf: Box<Vec<u8>>) -> Result<Self, SwfParseError> {
        let swf = match swf_parser::parse_swf(&swf) {
            Ok(swf) => swf,
            Err(err) => return Err(err),
        };

        Ok(Self {
            swf,
            mp3: None,
            chart: None,
        })
    }

    pub fn is_parsed(&self) -> bool {
        self.mp3.is_some() && self.chart.is_some()
    }

    pub fn get_mp3(&self) -> &Option<Vec<u8>> {
        return &self.mp3;
    }

    pub fn parse(&mut self) {
        let mut mp3_data: Vec<u8> = Vec::new();
        for tag in &self.swf.tags {
            match tag {
                Tag::DefineSound(sound_definition) => {
                    log::trace!("Format: {:?}, Sample Count: {:?}, Sample Rate: {:?}, Sample Size: {:?}, Sample Type: {:?}", sound_definition.format, sound_definition.sample_count, sound_definition.sound_rate, sound_definition.sound_size, sound_definition.sound_type);
                }

                Tag::DoAction(do_action) => SwfParser::parse_beatbox(&do_action.actions),

                Tag::SoundStreamBlock(stream_block) => {
                    mp3_data.extend_from_slice(&stream_block.data);
                }
                Tag::SoundStreamHead(stream_head) => {
                    log::trace!("{:?}", stream_head);
                }
                _ => {}
            }
        }

        self.mp3.replace(mp3_data);
    }

    fn parse_beatbox(actions: &Vec<u8>) {
        if let Some((action_code, action)) = actions.split_first() {
            if action_code == &swf_types::SWF_ACTION_CONSTANTPOOL {
                log::trace!("Action: {:?}", action);
            }
        } else {
        }
    }
}
