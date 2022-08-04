// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "genre")]
    pub genre: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "difficulty")]
    pub difficulty: i64,

    #[serde(rename = "style")]
    pub style: String,

    #[serde(rename = "time")]
    pub time: String,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "arrows")]
    pub arrows: i64,

    #[serde(rename = "playhash")]
    pub playhash: String,

    #[serde(rename = "previewhash")]
    pub previewhash: String,

    #[serde(rename = "prerelease")]
    pub prerelease: bool,

    #[serde(rename = "releasedate")]
    pub releasedate: i64,

    #[serde(rename = "author")]
    pub author: String,

    #[serde(rename = "stepauthor")]
    pub stepauthor: String,

    #[serde(rename = "authorURL")]
    pub author_url: String,

    #[serde(rename = "stepauthorURL")]
    pub stepauthor_url: String,

    #[serde(rename = "min_nps")]
    pub min_nps: i64,

    #[serde(rename = "max_nps")]
    pub max_nps: i64,

    #[serde(rename = "nps_data")]
    pub nps_data: String,

    #[serde(rename = "song_rating")]
    pub song_rating: Option<f64>,

    #[serde(rename = "credits")]
    pub credits: Option<i64>,

    #[serde(rename = "price")]
    pub price: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    #[serde(rename = "total_length")]
    pub total_length: i64,

    #[serde(rename = "eff_length")]
    pub eff_length: i64,

    #[serde(rename = "chord_count")]
    pub chord_count: i64,

    #[serde(rename = "avg_nps")]
    pub avg_nps: f64,

    #[serde(rename = "first_delay")]
    pub first_delay: i64,

    #[serde(rename = "last_delay")]
    pub last_delay: i64,

    #[serde(rename = "note_delays")]
    pub note_delays: Vec<i64>,

    #[serde(rename = "hand_bias")]
    pub hand_bias: i64,

    #[serde(rename = "jumps")]
    pub jumps: Vec<i64>,

    #[serde(rename = "color_jumps")]
    pub color_jumps: Vec<i64>,

    #[serde(rename = "framers")]
    pub framers: Vec<i64>,

    #[serde(rename = "density")]
    pub density: Vec<Vec<Density>>,

    #[serde(rename = "camel_jacks")]
    pub camel_jacks: i64,

    #[serde(rename = "color_total")]
    pub color_total: ColorTotal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorTotal {
    #[serde(rename = "red")]
    pub red: Vec<f64>,

    #[serde(rename = "blue")]
    pub blue: Vec<f64>,

    #[serde(rename = "purple")]
    pub purple: Vec<f64>,

    #[serde(rename = "yellow")]
    pub yellow: Vec<f64>,

    #[serde(rename = "pink")]
    pub pink: Vec<f64>,

    #[serde(rename = "orange")]
    pub orange: Vec<f64>,

    #[serde(rename = "cyan")]
    pub cyan: Vec<f64>,

    #[serde(rename = "green")]
    pub green: Vec<f64>,

    #[serde(rename = "white")]
    pub white: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Density {
    Integer(i64),

    String(String),
}
