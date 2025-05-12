use std::str;

use super::data::*;
use super::data::{Content, Message};
use super::ws::{use_web_socket, WebSocketHandle};
use anyhow::Result;
use dioxus::prelude::*;
use js_sys::wasm_bindgen::JsError;
use serde_json::{to_string, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use minijinja::Environment;
use std::sync::{LazyLock, RwLock};

static TMPL: LazyLock<RwLock<Environment>> = LazyLock::new(|| RwLock::new(Environment::new()));

#[derive(Clone)]
pub struct Store {
    pub ws: WebSocketHandle,
    pub layout: Signal<Layout>,
    pub data: Signal<HashMap<String, Layout>>,
    pub list: Signal<HashMap<String, Vec<Layout>>>,
}

impl Store {
    pub async fn send(&mut self, event: impl AsRef<str>, id: Option<String>, content: Value) {
        let x = Outflow {
            event: event.as_ref().to_string(),
            id,
            data: content,
        };

        if let Ok(msg) = to_string::<Outflow>(&x) {
            let msg = gloo_net::websocket::Message::Text(msg);
            let _ = self.ws.send(msg).await;
        }
    }
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
                content: Content::create(x),
                ..
            } => layout.set(x.data),
            Message {
                content: Content::merge(x),
                ..
            } => {
                let e = x.event;
                let d = x.data;
                data.write().insert(e, d);
            }
            Message {
                content: Content::tmpl(x),
                ..
            } => {
                let e = x.name;
                let d = x.data;
                let _ = TMPL.write().expect("write TMPL failed").add_template_owned(e, d);
                dioxus_logger::tracing::info!("{TMPL:?}");
            }
            Message {
                content: Content::join(x),
                ..
            } => {
                let e = x.event;
                let d = &x.data;
                if let Some(_id) = &d.id {
                    let mut l = list.write();
                    let list = l.entry(e).or_insert(vec![]);
                    let mut m = false;
                    for i in list.iter_mut() {
                        if i.cmp_id(d) {
                            m = true;
                            i.join(d.clone());
                        }
                    }
                    if !m {
                        list.push(d.clone());
                    }
                } else {
                    list.write().entry(e).or_insert(vec![]).push(d.clone());
                }
            }
            Message {
                sender: _,
                content: Content::empty,
            } => (),
        };
    });

    Ok(Store {
        ws,
        layout,
        data,
        list,
    })
}
