use rrr::{AudioPlayer, SwfParser};

pub(crate) struct Head {
    audio_player: Option<AudioPlayer>,
}

impl Head {
    pub(crate) fn new() -> Self {
        Self { audio_player: None }
    }

    pub(crate) fn play_song(&mut self) -> anyhow::Result<()> {
        const TEST_CHART: usize = 3348;
        if let Some(raw_chart) = rrr::download_chart(TEST_CHART) {
            if let Ok(mut parser) = SwfParser::new(raw_chart) {
                parser.parse();
                if let Some(mp3) = parser.get_mp3().as_ref() {
                    self.audio_player = Some(AudioPlayer::try_new(mp3)?);
                }
            };
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn stop(&mut self) {
        self.audio_player.as_mut().map(AudioPlayer::stop);
        self.audio_player = None;
    }

    pub(crate) fn tick(&mut self) {
        self.audio_player.as_mut().map(AudioPlayer::tick);
    }
}
