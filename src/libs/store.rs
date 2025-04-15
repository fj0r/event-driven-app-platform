use std::str;

use super::data::*;
use super::ws::use_web_socket;
use anyhow::Result;
use dioxus::prelude::*;
use js_sys::wasm_bindgen::JsError;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Store {
    pub layout: Signal<Layout>,
    pub data: Signal<HashMap<String, Layout>>,
    pub list: Signal<HashMap<String, Vec<Layout>>>,
}

pub fn use_store(url: &str) -> Result<Store, JsError> {
    let ws = use_web_socket(url)?;
    let x = ws.message_texts();

    let mut layout = use_signal::<Layout>(|| Layout::default());
    let mut data = use_signal::<HashMap<String, Layout>>(|| HashMap::new());
    let mut list = use_signal::<HashMap<String, Vec<Layout>>>(|| HashMap::new());

    use_memo(move || {
        let act = serde_json::from_str::<Message>(&x()).unwrap_or_else(|_| Message::default());
        match act {
            Message {
                content: Content::layout(x),
                ..
            } => layout.set(x),
            Message {
                content: Content::data(x),
                ..
            } => {
                let e = x.event;
                let d = x.data;
                data.write().insert(e, d);
            }
            Message {
                content: Content::append(x),
                ..
            } => {
                let e = x.event;
                let d = x.data;
                list.write()
                    .entry(e)
                    .or_insert(vec![d.clone()])
                    .push(d);
            }
            Message {
                sender: _,
                content: Content::empty,
            } => (),
        };
    });

    Ok(Store { layout, data, list })
}
