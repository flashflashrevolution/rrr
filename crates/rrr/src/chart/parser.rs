use swf_parser::{
    swf_types::{Movie, *},
    SwfParseError,
};

pub struct SwfParser {
    swf: Movie,
}

impl SwfParser {
    pub fn new(swf: Box<Vec<u8>>) -> Result<Self, SwfParseError> {
        let swf = match swf_parser::parse_swf(&swf) {
            Ok(swf) => swf,
            Err(err) => return Err(err),
        };

        Ok(Self { swf })
    }

    pub fn get_mp3(&self) -> Vec<u8> {
        SwfParser::extract_mp3_from_movie(&self.swf)
    }

    fn extract_mp3_from_movie(chart: &Movie) -> Vec<u8> {
        let mut mp3_data: Vec<u8> = Vec::new();
        for tag in &chart.tags {
            match tag {
                Tag::CsmTextSettings(_) => {}
                Tag::DefineBinaryData(_) => {}
                Tag::DefineBitmap(_) => {}
                Tag::DefineButton(_) => {}
                Tag::DefineButtonColorTransform(_) => {}
                Tag::DefineButtonSound(_) => {}
                Tag::DefineCffFont(_) => {}
                Tag::DefineDynamicText(_) => {}
                Tag::DefineFont(_) => {}
                Tag::DefineFontAlignZones(_) => {}
                Tag::DefineFontInfo(_) => {}
                Tag::DefineFontName(_) => {}
                Tag::DefineGlyphFont(_) => {}
                Tag::DefineJpegTables(_) => {}
                Tag::DefineMorphShape(_) => {}
                Tag::DefineScalingGrid(_) => {}
                Tag::DefineSceneAndFrameLabelData(_) => {}
                Tag::DefineShape(_) => {}
                Tag::DefineSound(sound) => {
                    println!(
                    "Format: {:?}, Sample Count: {:?}, Sample Rate: {:?}, Sample Size: {:?}, Sample Type: {:?}",
                    sound.format, sound.sample_count, sound.sound_rate, sound.sound_size, sound.sound_type
                )
                }
                Tag::DefineSprite(_) => {}
                Tag::DefineText(_) => {}
                Tag::DefineVideoStream(_) => {}
                Tag::EnablePostscript => {}
                Tag::DoAbc(_) => {}
                Tag::DoAction(_) => {}
                Tag::DoInitAction(_) => {}
                Tag::EnableDebugger(_) => {}
                Tag::ExportAssets(_) => {}
                Tag::FileAttributes(_) => {}
                Tag::FrameLabel(_) => {}
                Tag::ImportAssets(_) => {}
                Tag::Metadata(_) => {}
                Tag::PlaceObject(_) => {}
                Tag::Protect(_) => {}
                Tag::Raw(_) => {}
                Tag::RawBody(_) => {}
                Tag::RemoveObject(_) => {}
                Tag::ScriptLimits(_) => {}
                Tag::SetBackgroundColor(_) => {}
                Tag::SetTabIndex(_) => {}
                Tag::ShowFrame => {}
                Tag::SoundStreamBlock(sound) => {
                    mp3_data.extend_from_slice(&sound.data);
                }
                Tag::SoundStreamHead(sound) => {
                    println!("{:?}", sound);
                }
                Tag::StartSound(_) => {}
                Tag::StartSound2(_) => {}
                Tag::SymbolClass(_) => {}
                Tag::Telemetry(_) => {}
                Tag::VideoFrame(_) => {}
            }
        }

        mp3_data
    }
}
