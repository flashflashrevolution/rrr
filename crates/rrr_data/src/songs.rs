use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Author {
    name: String,
    url: String,
}

impl Default for Author {
    fn default() -> Self {
        Self {
            name: String::default(),
            url: String::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    difficulty: u8,
    time: u16,
    id: u64,
    releasedate: u64,
    author: Author,
    stepauthor: Author,
    rating: f32,
}
impl Default for Song {
    fn default() -> Self {
        Self {
            name: String::default(),
            difficulty: 0,
            time: 0,
            id: 0,
            releasedate: 0,
            author: Author::default(),
            stepauthor: Author::default(),
            rating: 0.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SongList {
    songs: Vec<Song>,
}

impl Default for SongList {
    fn default() -> Self {
        Self { songs: Vec::new() }
    }
}

impl SongList {
    pub fn from_fake() -> Self {
        let mut file = File::open("sample_data/sample_data.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str::<SongList>(&data).unwrap()
    }

    pub fn get_song(&self, index: usize) -> &Song {
        &self.songs[index]
    }
}
