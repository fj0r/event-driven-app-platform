use super::ws::{WebSocketHandle, use_web_socket};
use anyhow::Result;
use component::{
    ComponentProps, JsonComponent,
    merge::{ComponentOp, Concat, Delete, Replace},
};
use content::{Content, Message, Method, Outflow};
#[allow(unused_imports)]
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use js_sys::wasm_bindgen::JsError;
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
pub struct Status {
    pub ws: WebSocketHandle,
    pub layout: Signal<JsonComponent>,
    pub data: Signal<HashMap<String, JsonComponent>>,
    pub list: Signal<HashMap<String, Vec<JsonComponent>>>,
}

impl Status {
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

    pub fn set(&mut self, name: impl AsRef<str>, component: JsonComponent) {
        self.data
            .write()
            .insert(name.as_ref().to_string(), component);
    }
}

fn dispatch(
    act: Message<JsonComponent>,
    layout: &mut Signal<JsonComponent>,
    data: &mut Signal<HashMap<String, JsonComponent>>,
    list: &mut Signal<HashMap<String, Vec<JsonComponent>>>,
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
                let d = &mut x.data;
                let vs: &dyn ComponentOp = match x.method {
                    Method::Replace => &Replace,
                    Method::Concat => &Concat,
                    Method::Delete => &Delete,
                };
                if let Some(_id) = &d.get_id() {
                    let mut l = list.write();
                    let list = l.entry(e).or_default();
                    let mut is_merge = false;
                    for i in list.iter_mut() {
                        if i.cmp_id(d) {
                            is_merge = true;
                            i.merge(vs, d);
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

pub fn use_status(url: &str) -> Result<Status, JsError> {
    let ws = use_web_socket(url)?;
    let x = ws.message_texts();

    let mut layout = use_signal::<JsonComponent>(|| {
        JsonComponent::text(component::Text {
            ..Default::default()
        })
    });
    let mut data = use_signal::<HashMap<String, JsonComponent>>(HashMap::new);
    let mut list = use_signal::<HashMap<String, Vec<JsonComponent>>>(HashMap::new);

    use_memo(move || {
        let act = &x();
        if !act.is_empty() {
            match serde_json::from_str::<Message<JsonComponent>>(act) {
                Ok(act) => dispatch(act, &mut layout, &mut data, &mut list),
                Err(err) => dioxus::logger::tracing::info!(
                    "deserialize from_str error {:?}\n\n{:?}",
                    err,
                    act
                ),
            }
        }
    });

    Ok(Status {
        ws,
        layout,
        data,
        list,
    })
}
