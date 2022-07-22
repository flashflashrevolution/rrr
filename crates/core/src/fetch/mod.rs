use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Payload {
    pub artists: Vec<Artist>,
    pub charts: Vec<Chart>,
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

pub async fn fetch() -> Result<Option<Payload>, Box<dyn std::error::Error>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::fetch().await
    }
    #[cfg(target_arch = "wasm32")]
    {
        let result = wasm::fetch().await;
        match result {
            Ok(Some(payload)) => Ok(Some(payload)),
            Ok(None) => Ok(None),
            Err(err) => Err(err
                .as_string()
                .unwrap_or_else(|| "Invalid error.".to_string())
                .into()),
        }
    }
}
