use rrr::{AudioPlayer, CompiledChart, SwfParser};

pub(crate) struct Head {
    audio_player: Option<AudioPlayer>,
    mp3: Option<Vec<u8>>,
    chart: Option<CompiledChart>,
    parser: Option<SwfParser>,
}

impl Head {
    pub(crate) fn new() -> Self {
        Self {
            audio_player: None,
            mp3: None,
            chart: None,
            parser: None,
        }
    }

    pub(crate) fn play_song(&mut self) -> anyhow::Result<()> {
        if let Some(mp3) = &self.mp3 {
            self.audio_player = Some(AudioPlayer::try_new(mp3.as_slice())?);
        }
        Ok(())
    }

    pub(crate) fn load_chart(&mut self, chart_id: usize) {
        if let Some(raw_chart) = rrr::download_chart(chart_id) {
            self.parser = if let Ok(parser) = SwfParser::new(&raw_chart[..]) {
                Some(parser)
            } else {
                None
            };

            if let Some(parser) = &mut self.parser {
                parser.parse();
                self.mp3 = parser.get_mp3();
                self.chart = parser
                    .get_chart()
                    .as_ref()
                    .map(|notes| CompiledChart::new(notes));
            };
        }
    }

    #[allow(dead_code)]
    pub(crate) fn stop(&mut self) {
        self.audio_player.as_mut().map(AudioPlayer::stop);
        self.audio_player = None;
    }

    pub(crate) fn tick(&mut self) {
        self.audio_player.as_mut().map(AudioPlayer::tick);
    }

    pub(crate) fn chart(&self) -> Option<CompiledChart> {
        self.chart.clone()
    }
}
