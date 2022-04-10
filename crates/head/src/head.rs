use rrr::{AudioPlayer, SwfParser};
use std::time::Duration;

pub struct Head {
    audio_player: Option<AudioPlayer>,
}

impl Head {
    pub fn new() -> Self {
        Self { audio_player: None }
    }

    pub fn play_song(&mut self) {
        const TEST_CHART: usize = 3348;
        if let Some(raw_chart) = rrr::download_chart(TEST_CHART) {
            if let Ok(mut parser) = SwfParser::new(raw_chart) {
                parser.parse();
                if let Some(mp3) = parser.get_mp3() {
                    self.audio_player = Some(AudioPlayer::new(mp3));
                }
            };
        }
    }

    pub fn stop(&mut self) {
        if let Some(player) = &mut self.audio_player {
            player.stop();
        }
        self.audio_player = None;
    }

    pub fn tick(&mut self) {
        if let Some(audio_player) = &mut self.audio_player {
            audio_player.tick();
        }
    }
}
