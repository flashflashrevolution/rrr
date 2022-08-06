use super::BytesFetch;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde::Deserialize;
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

pub struct Fetcher {
    rx: Receiver<BytesFetch>,
    handle: thread::JoinHandle<()>,
}

impl Fetcher {
    pub fn new(chart_id: usize) -> Self {
        let (tx, rx): (Sender<BytesFetch>, Receiver<BytesFetch>) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let temp_hash = if let Some(hash) = option_env!("TEST_PREVIEW_HASH") {
                hash.to_string()
            } else {
                "Fill hash here for local testing.".to_string()
            };
            let url = format!(
                "https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&mode=2&type=ChartFFR_music",
                temp_hash
            );
            println!("{}", url);
            let dat = fetch_data(url);
            tx.send(dat).unwrap();
        });

        Self { rx, handle }
    }

    pub fn fetch(&self) -> Option<BytesFetch> {
        if let Ok(fetched_data) = self.rx.try_recv() {
            Some(fetched_data)
        } else {
            None
        }
    }
}

pub(crate) fn fetch<T: for<'de> Deserialize<'de>>(url: String) -> Result<Option<T>> {
    let response = reqwest::blocking::get(url)?;

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<T>() {
            Ok(parsed) => Ok(Some(parsed)),
            Err(err) => Err(anyhow!(err)),
        },
        other => Err(anyhow!("Invalid status code: {}", other.as_str())),
    }
}

pub(crate) fn fetch_data(url: String) -> BytesFetch {
    if let Ok(response) = reqwest::blocking::get(url) {
        match response.status() {
            reqwest::StatusCode::OK => match response.bytes() {
                Ok(parsed) => BytesFetch::Ok(parsed.to_vec()),
                Err(err) => BytesFetch::Err(format!("{:?}", err.status())),
            },
            other => BytesFetch::Err(format!("Invalid status code: {}", other.as_str())),
        }
    } else {
        BytesFetch::Err("Could not make request.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch::PlaylistPayload;

    #[test]
    fn test_fetch() {
        let test_result = fetch::<PlaylistPayload>(
            "https://www.flashflashrevolution.com/game/r3/r3-playlist.v2.php".to_string(),
        );
        assert!(test_result.is_ok());

        if let Ok(Some(payload)) = test_result {
            assert!(!payload.songs.is_empty());
            if let Some(song) = payload.songs.get(0) {
                assert!(!song.name.is_empty());

                let song_result = fetch_data(
                    format!("https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&mode=2&type=ChartFFR_music", song.previewhash),
                );

                match song_result {
                    BytesFetch::Ok(bytes) => {
                        assert!(!bytes.is_empty());
                    }
                    BytesFetch::Err(err) => {
                        assert!(false);
                    }
                    BytesFetch::Wait => {
                        assert!(false);
                    }
                }
            }
        }
    }
}
