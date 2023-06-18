use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use crate::{INDEX_PREFIX};

#[wasm_bindgen]
pub async fn get_inverted_index() -> Result<String, JsValue> {

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let mut url = String::new();
    url.push_str(INDEX_PREFIX);
    url.push_str("main.json");
    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let inverted_index = JsFuture::from(resp.text()?).await?;

    Ok(inverted_index.as_string().unwrap())

}