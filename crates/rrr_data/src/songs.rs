use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Author {
    name: String,
    url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Song {
    name: String,
    difficulty: u8,
    time: u16,
    id: u64,
    releasedate: u64,
    author: Author,
    stepauthor: Author,
    rating: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Songs {
    songs: Vec<Song>,
}
