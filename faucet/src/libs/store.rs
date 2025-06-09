use super::data::*;
use super::data::{Content, Created, LayoutOp, Message};
use super::ws::{use_web_socket, WebSocketHandle};
use anyhow::Result;
use dioxus::prelude::*;
use js_sys::wasm_bindgen::JsError;
use minijinja::{AutoEscape, Environment};
use serde_json::{to_string, Value};
use std::collections::HashMap;
use std::str;
use std::sync::{LazyLock, RwLock};

static TMPL: LazyLock<RwLock<Environment>> = LazyLock::new(|| {
    let mut env = Environment::new();
    //env.set_auto_escape_callback(|_| AutoEscape::Json);
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
            content: Content::Tmpl(x),
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
            content: Content::Create(mut x),
            ..
        } => {
            let env = TMPL.read().expect("read TMPL failed");
            x.data.render(&env);
            layout.set(x.data)
        }
        Message {
            content: Content::Set(x),
            ..
        } => {
            let e = x.event;
            let mut d = x.data;
            let env = TMPL.read().expect("read TMPL failed");
            d.render(&env);
            data.write().insert(e, d);
        }
        Message {
            content: Content::Join(mut x),
            ..
        } => {
            let env = TMPL.read().expect("read TMPL failed");
            x.data.render(&env);
            let e = x.event;
            let d = &x.data;
            let vs: &dyn LayoutOp = match x.method {
                Method::Replace => &Replace as &dyn LayoutOp,
                Method::Concat => &Concat as &dyn LayoutOp,
                Method::Delete => &Delete as &dyn LayoutOp,
            };
            if let Some(_id) = &d.id {
                let mut l = list.write();
                let list = l.entry(e).or_default();
                let mut mg = false;
                for i in list.iter_mut() {
                    if i.cmp_id(d) {
                        mg = true;
                        i.merge(vs, d.clone());
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
            content: Content::Empty,
            created: _,
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
