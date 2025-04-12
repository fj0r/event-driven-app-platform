use std::str;

use super::ws::use_web_socket;
use js_sys::wasm_bindgen::JsError;
use dioxus::prelude::*;
use anyhow::Result;
use super::data::*;


#[derive(Clone, Copy)]
pub struct Store {
    pub layout: Signal<Layout>,
}

pub fn use_store(url: &str) -> Result<Store, JsError> {
    let ws = use_web_socket(url)?;
    let x = ws.message_texts();

    let mut layout = use_signal::<Layout>(|| Layout::default());

    use_memo(move|| {
        let act = serde_json::from_str::<Message>(&x())
            .unwrap_or_else(|_| Message::default());
        match act {
            Message{content: Content::layout(x), ..} => layout.set(x),
            Message{user: _, content: Content::empty} => (),
        };
    });

    Ok(Store{
        layout
    })
}

