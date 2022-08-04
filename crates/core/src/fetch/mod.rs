use self::playlist::{Song, Stat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod playlist;

// type BytesFetch = Result<std::option::Option<bytes::Bytes>>;
#[derive(Serialize, Deserialize)]
pub enum BytesFetch {
    Ok(Vec<u8>),
    Wait,
    Err(String),
}

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub mod worker;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ManifestPayload {
    pub artists: Vec<Artist>,
    pub charts: Vec<Chart>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistPayload {
    #[serde(rename = "songs")]
    pub songs: Vec<Song>,

    #[serde(rename = "stats")]
    pub stats: HashMap<String, Stat>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub userid: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chart {
    pub arrows: i32,
    pub artist: i32,
    pub author: i32,
    pub difficulty: i32,
    pub genre: i32,
    pub level: i32,
    pub name: String,
    pub releasedate: String,
    pub style: String,
    pub time: String,
}

pub fn download_chart(chart_id: usize) -> Fetcher {
    Fetcher::new(chart_id)
}
