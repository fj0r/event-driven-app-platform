use serde_json::{to_value, Map, Value};
use std::collections::HashMap;

use super::super::data::{Bind, Layout};
use super::super::store::Store;
use super::{Dynamic, Frame};
use dioxus::prelude::*;

type FormScope = HashMap<String, Signal<Value>>;

fn walk(layout: &mut Layout, scope: &mut FormScope, confirm: Signal<Value>) {
    match &layout.data {
        Some(Bind::Field {
            field,
            kind,
            signal: _,
        }) => {
            let kind = kind.clone();
            let v = match kind.as_ref().map(|x| x.as_str()) {
                Some("number") => {
                    let n = layout
                        .value
                        .as_ref()
                        .and_then(|x| x.as_f64())
                        .unwrap_or(0 as f64);
                    to_value(n).unwrap()
                }
                Some("bool") => {
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
            scope.insert(field.to_string(), s);
            layout.data = Some(Bind::Field {
                field: field.to_string(),
                kind,
                signal: Some(s),
            });
        }
        Some(Bind::Submit { .. }) => {
            layout.data = Some(Bind::Submit {
                submit: true,
                signal: Some(confirm),
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
    let mut data: FormScope = HashMap::new();
    let confirm = use_signal(|| Value::Bool(false));
    walk(&mut layout, &mut data, confirm);
    let children = layout.clone().children.unwrap_or_else(Vec::new);
    let children = children.into_iter().map(|c| {
        rsx! {
            Frame { layout: c }
        }
    });

    let lc = layout.data.clone();
    if let Some(Bind::Event { event, .. }) = lc {
        let s = use_context::<Store>();
        let mut payload = Map::new();
        for (k, v) in &data {
            payload.insert(k.to_owned(), v());
        }
        //dioxus_logger::tracing::info!("{payload:?}");
        let v = Value::Object(payload);
        let _ = use_resource(move || {
            let ev = event.clone();
            let mut s = s.clone();
            let v = v.clone();
            async move {
                if let Some(c) = confirm.read().as_bool() {
                    if c {
                        s.send(ev, None, v).await;
                    }
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
