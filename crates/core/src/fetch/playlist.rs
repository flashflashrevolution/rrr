use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub genre: i64,
    pub name: String,
    pub difficulty: i64,
    pub style: String,
    pub time: String,
    pub level: i64,
    pub order: i64,
    pub arrows: i64,
    pub playhash: String,
    pub previewhash: String,
    pub prerelease: bool,
    pub releasedate: i64,
    pub author: String,
    pub stepauthor: String,

    #[serde(rename = "authorURL")]
    pub author_url: String,

    #[serde(rename = "stepauthorURL")]
    pub stepauthor_url: String,
    pub min_nps: i64,
    pub max_nps: i64,
    pub nps_data: String,
    pub song_rating: Option<f64>,
    pub credits: Option<i64>,
    pub price: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub total_length: i64,
    pub eff_length: i64,
    pub chord_count: i64,
    pub avg_nps: f64,
    pub first_delay: i64,
    pub last_delay: i64,
    pub note_delays: Vec<i64>,
    pub hand_bias: i64,
    pub jumps: Vec<i64>,
    pub color_jumps: Vec<i64>,
    pub framers: Vec<i64>,
    pub density: Vec<Vec<Density>>,
    pub camel_jacks: i64,
    pub color_total: ColorTotal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorTotal {
    pub red: Vec<f64>,
    pub blue: Vec<f64>,
    pub purple: Vec<f64>,
    pub yellow: Vec<f64>,
    pub pink: Vec<f64>,
    pub orange: Vec<f64>,
    pub cyan: Vec<f64>,
    pub green: Vec<f64>,
    pub white: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Density {
    Integer(i64),
    String(String),
}
