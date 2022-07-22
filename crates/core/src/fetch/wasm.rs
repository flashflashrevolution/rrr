use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::fetch::Payload;

pub async fn fetch() -> Result<Option<Payload>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://meta.rrr.flashflashrevolution.com/payloads/staging-manifest.json",
        &opts,
    )?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    if let Ok(meta) = json.into_serde() {
        Ok(Some(meta))
    } else {
        Ok(None)
    }
}

// rust tests
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test(async)]
    async fn test_fetch() {

        let result = fetch().await;
        match result {
            Ok(Some(meta)) => {
                assert_ne!(meta.artists.len(), 0);
                assert_ne!(meta.charts.len(), 0);
            }
            Ok(None) => panic!("No payload found."),
            Err(err) => panic!("{:?}", err),
        }
    }
}
