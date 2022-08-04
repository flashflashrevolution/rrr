use super::{fetch_data, BytesFetch};
use bytes::Bytes;
use futures::executor::block_on;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::{spawn_local, JsFuture};

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
