use songs::SongList;

pub mod songs;

#[derive(Debug, Default)]
pub struct RRRData {
    song_list: SongList,
}

impl RRRData {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_fake() -> Self {
        Self {
            song_list: SongList::from_fake(),
        }
    }

    pub fn songs(&self) -> &SongList {
        &self.song_list
    }
}
