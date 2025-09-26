use serde_json::{Value, to_value};
use std::collections::HashMap;

use super::super::store::Store;
use super::{Dynamic, Frame};
use dioxus::prelude::*;
use layout::{Bind, JsKind, Layout, Settings};
use maplit::hashmap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    pub data: Value,
    pub payload: Option<Value>,
}

type FormScope = HashMap<String, (Signal<Value>, Option<Value>)>;

fn walk(layout: &mut Layout, scope: &mut FormScope, confirm: Signal<Value>) {
    match layout.bind.as_ref().and_then(|x| x.get("value")) {
        Some(Bind::Field {
            field,
            kind,
            payload,
            signal: _,
        }) => {
            let kind = kind.clone();
            let v = match kind {
                Some(JsKind::number) => {
                    let n = layout
                        .value
                        .as_ref()
                        .and_then(|x| x.as_f64())
                        .unwrap_or(0 as f64);
                    to_value(n).unwrap()
                }
                Some(JsKind::bool) => {
                    let b = layout
                        .value
                        .as_ref()
                        .and_then(|x| x.as_bool())
                        .unwrap_or(false);
                    to_value(b).unwrap()
                }
                _ => {
                    let s = layout.value.as_ref().and_then(|x| x.as_str()).unwrap_or("");
                    to_value(s).unwrap()
                }
            };

            let s = use_signal(|| v);
            scope.insert(field.to_string(), (s, payload.clone()));
            layout.bind = Some(hashmap! {
                "value".to_owned() => Bind::Field {
                    field: field.to_string(),
                    kind,
                    payload: None,
                    signal: Some(s),
                },
            });
        }
        Some(Bind::Submit { .. }) => {
            layout.bind = Some(hashmap! {
                "value".to_owned() => Bind::Submit {
                    submit: true,
                    signal: Some(confirm),
                },
            });
        }
        _ => {}
    };
    if let Some(children) = &mut layout.children {
        for c in children.iter_mut() {
            walk(c, scope, confirm);
        }
    };
}

#[component]
pub fn Form(layout: Layout) -> Element {
    // TODO: instant
    let _instant = layout
        .attrs
        .clone()
        .and_then(|x| {
            if let Some(Settings::Form { instant }) = x.settings {
                Some(instant)
            } else {
                None
            }
        })
        .unwrap_or(false);

    let mut data: FormScope = HashMap::new();
    let confirm = use_signal(|| Value::Bool(false));
    walk(&mut layout, &mut data, confirm);
    let children = layout.clone().children.unwrap_or_else(Vec::new);
    let children = children.into_iter().map(|c| {
        rsx! {
            Frame { layout: c }
        }
    });

    let lc = layout.bind.as_ref().and_then(|x| x.get("value")).cloned();
    if let Some(Bind::Target { target, .. }) = lc {
        let s = use_context::<Store>();
        let mut content = HashMap::new();
        for (k, v) in &data {
            let d = Message {
                data: v.0(),
                payload: v.1.clone(),
            };
            content.insert(k.to_owned(), d);
        }
        //dioxus_logger::tracing::info!("{payload:?}");
        let v = to_value(content).unwrap();
        let _ = use_resource(move || {
            let ev = target.clone();
            let mut s = s.clone();
            let v = v.clone();
            async move {
                if let Some(c) = confirm.read().as_bool()
                    && c
                {
                    s.send(ev, None, v).await;
                }
            }
        });
    };

    layout.kind = "case".to_owned();
    rsx! {
        Dynamic {
            layout: layout,
            {children}
        }
    }
}
