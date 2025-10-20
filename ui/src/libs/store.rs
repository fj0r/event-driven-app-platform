use super::ws::{WebSocketHandle, use_web_socket};
use anyhow::Result;
use content::{Content, Message, Method, Outflow};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use js_sys::wasm_bindgen::JsError;
use layout::{Bind, Concat, Delete, Layout, LayoutOp, Replace};
use minijinja::Environment;
use serde_json::{Value, to_string};
use std::collections::HashMap;
use std::str;
use std::sync::{LazyLock, RwLock};

static TMPL: LazyLock<RwLock<Environment>> = LazyLock::new(|| {
    let env = Environment::new();
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

    pub fn set(&mut self, name: impl AsRef<str>, layout: Layout) {
        self.data.write().insert(name.as_ref().to_string(), layout);
    }
}

fn dispatch(
    act: Message<Layout>,
    layout: &mut Signal<Layout>,
    data: &mut Signal<HashMap<String, Layout>>,
    list: &mut Signal<HashMap<String, Vec<Layout>>>,
) {
    let Message {
        sender: _,
        created: _,
        content,
    } = act;
    for c in content {
        match c {
            Content::Tmpl(x) => {
                let n = x.name;
                let d = x.data;
                let _ = TMPL
                    .write()
                    .expect("write TMPL failed")
                    .add_template_owned(n, d);
            }
            Content::Create(mut x) => {
                let env = TMPL.read().expect("read TMPL failed");
                x.data.render(&env);
                layout.set(x.data)
            }
            Content::Set(x) => {
                let e = x.event;
                let mut d = x.data;
                let env = TMPL.read().expect("read TMPL failed");
                d.render(&env);
                data.write().insert(e, d);
            }
            Content::Join(mut x) => {
                let env = TMPL.read().expect("read TMPL failed");
                x.data.render(&env);
                let e = x.event;
                let d = &x.data;
                let vs: &dyn LayoutOp = match x.method {
                    Method::Replace => &Replace,
                    Method::Concat => &Concat,
                    Method::Delete => &Delete,
                };
                if let Some(_id) = &d.id {
                    let mut l = list.write();
                    let list = l.entry(e).or_default();
                    let mut is_merge = false;
                    for i in list.iter_mut() {
                        if i.cmp_id(d) {
                            is_merge = true;
                            i.merge(vs, d.clone());
                        }
                    }
                    if !is_merge {
                        list.push(d.clone());
                    }
                } else {
                    list.write().entry(e).or_default().push(d.clone());
                }
            }
            Content::Empty => {}
        }
    }
}

pub fn use_store(url: &str) -> Result<Store, JsError> {
    let ws = use_web_socket(url)?;
    let x = ws.message_texts();

    let mut layout = use_signal::<Layout>(Layout::default);
    let mut data = use_signal::<HashMap<String, Layout>>(HashMap::new);
    let mut list = use_signal::<HashMap<String, Vec<Layout>>>(HashMap::new);

    use_memo(
        move || match serde_json::from_str::<Message<Layout>>(&x()) {
            Ok(act) => dispatch(act, &mut layout, &mut data, &mut list),
            Err(err) => dioxus::logger::tracing::info!("deserialize from_str error {:?}", err),
        },
    );

    Ok(Store {
        ws,
        layout,
        data,
        list,
    })
}
