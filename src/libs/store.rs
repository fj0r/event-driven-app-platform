use std::str;

use super::data::*;
use super::data::{Content, Message, Created};
use super::ws::{use_web_socket, WebSocketHandle};
use anyhow::Result;
use dioxus::prelude::*;
use js_sys::wasm_bindgen::JsError;
use minijinja::{AutoEscape, Environment};
use serde_json::{to_string, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

static TMPL: LazyLock<RwLock<Environment>> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.set_auto_escape_callback(|_| AutoEscape::Json);
    RwLock::new(env)
});

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

fn dispatch(
    act: Message,
    layout: &mut Signal<Layout>,
    data: &mut Signal<HashMap<String, Layout>>,
    list: &mut Signal<HashMap<String, Vec<Layout>>>,
) {
    match act {
        Message {
            content: Content::create(x),
            ..
        } => {
            layout.set(x.data)
        },
        Message {
            content: Content::tmpl(x),
            ..
        } => {
            let n = x.name;
            let d = x.data;
            let _ = TMPL
                .write()
                .expect("write TMPL failed")
                .add_template_owned(n, d);
        }
        Message {
            sender,
            content: Content::fill(x),
            created
        } => {
            let n = x.name;
            let cx = x.data;
            let t = TMPL.read().expect("read TMPL failed");
            let t = t.get_template(&n).expect("not found TMPL");
            let d = t.render(cx).unwrap();
            match serde_json::from_str::<Content>(&d) {
                Ok(content) => {
                    let m = Message { sender, content, created };
                    dispatch(m, layout, data, list);
                }
                Err(x) => {
                    dioxus_logger::tracing::info!("{x:?}");
                    dioxus_logger::tracing::info!("{d:?}");
                }
            }
        }
        Message {
            content: Content::merge(x),
            ..
        } => {
            let e = x.event;
            let d = x.data;
            data.write().insert(e, d);
        }
        Message {
            content: Content::join(x),
            ..
        } => {
            let e = x.event;
            let d = &x.data;
            if let Some(_id) = &d.id {
                let mut l = list.write();
                let list = l.entry(e).or_default();
                let mut mg = false;
                for i in list.iter_mut() {
                    if i.cmp_id(d) {
                        mg = true;
                        i.join(d.clone());
                    }
                }
                if !mg {
                    list.push(d.clone());
                }
            } else {
                list.write().entry(e).or_default().push(d.clone());
            }
        }
        Message {
            sender: _,
            content: Content::empty,
            created:_
        } => {}
    }
}

pub fn use_store(url: &str) -> Result<Store, JsError> {
    let ws = use_web_socket(url)?;
    let x = ws.message_texts();

    let mut layout = use_signal::<Layout>(Layout::default);
    let mut data = use_signal::<HashMap<String, Layout>>(HashMap::new);
    let mut list = use_signal::<HashMap<String, Vec<Layout>>>(HashMap::new);

    use_memo(move || {
        let act = serde_json::from_str::<Message>(&x()).unwrap_or_else(|_| Message::default());
        dispatch(act, &mut layout, &mut data, &mut list);
    });

    Ok(Store {
        ws,
        layout,
        data,
        list,
    })
}
