use std::cell::RefCell;

use crate::fetch::BytesFetch;
use anyhow::Result;
use futures::channel::{oneshot, oneshot::Receiver};
use gloo_net::http::Request;
use gloo_worker::{Spawnable, WorkerBridge};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use gloo_worker::{HandlerId, Worker, WorkerScope};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug)]
pub enum Msg<T> {
    Respond { output: T, id: HandlerId },
}

pub struct FetchWorker {}

impl Worker for FetchWorker {
    // The Markdown Markup to Render.
    type Input = String;

    type Message = Msg<BytesFetch>;

    // The Rendered Html Output.
    type Output = BytesFetch;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        let Msg::Respond { output, id } = msg;
        scope.respond(id, output);
    }

    fn received(&mut self, scope: &WorkerScope<Self>, url: Self::Input, who: HandlerId) {
        let move_scope = scope.clone();
        spawn_local(async move {
            let data = fetch_data(url).await;

            let res = if let Ok(Some(data)) = data {
                BytesFetch::Ok(data)
            } else {
                BytesFetch::Err(data.unwrap_err().as_string().unwrap())
            };

            move_scope.send_message(Msg::Respond {
                output: res,
                id: who,
            });
        });
    }
}

pub struct Fetcher {
    bridge: WorkerBridge<FetchWorker>,
    rx: Receiver<BytesFetch>,
}

impl Fetcher {
    pub fn new(chart_id: usize) -> Self {
        let (tx, rx) = oneshot::channel();
        let tx = RefCell::new(Some(tx));
        let bridge = FetchWorker::spawner()
            .callback(move |bytes| {
                if let Some(tx) = tx.borrow_mut().take() {
                    let _ = tx.send(bytes);
                }
            })
            .spawn("bin/fetch-worker.js");

        let temp_hash = if let Some(hash) = option_env!("TEST_PREVIEW_HASH") {
            hash.to_string()
        } else {
            "".to_string()
        };
        bridge.send(format!("https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&mode=2&type=ChartFFR_music", temp_hash));
        Self { bridge, rx }
    }

    pub fn fetch(&mut self) -> Option<BytesFetch> {
        if let Ok(Some(data)) = self.rx.try_recv() {
            Some(data)
        } else {
            None
        }
    }
}

pub async fn fetch<T: for<'de> Deserialize<'de>>(url: String) -> Result<Option<T>, JsValue> {
    let resp = Request::get(url.as_str()).send().await.unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = resp.json().await;

    // Use serde to parse the JSON into a struct.
    if let Ok(meta) = json {
        Ok(Some(meta))
    } else {
        Ok(None)
    }
}

pub async fn fetch_data(url: String) -> Result<Option<Vec<u8>>, JsValue> {
    log::info!("{}", url);
    let resp = Request::get(url.as_str()).send().await.unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = resp.binary().await;

    // Use serde to parse the JSON into a struct.
    if let Ok(meta) = json {
        Ok(Some(meta))
    } else {
        Ok(None)
    }
}

// rust tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch::PlaylistPayload;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test(async)]
    async fn test_fetch() {
        let test_result = fetch::<PlaylistPayload>(
            "https://www.flashflashrevolution.com/game/r3/r3-playlist.v2.php".to_string(),
        )
        .await;
        assert!(test_result.is_ok());

        if let Ok(result) = test_result {
            if let Some(payload) = result {
                assert!(!payload.songs.is_empty());
                let song_result: Result<Option<Vec<u8>>, JsValue> = fetch_data(format!("https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&mode=2&type=ChartFFR_music", payload.songs[0].previewhash).to_string()).await;
                assert!(song_result.is_ok());
                if let Ok(song) = song_result {
                    assert!(song.is_some());
                    assert!(song.unwrap().len() > 0);
                }
            }
        }
    }
}
