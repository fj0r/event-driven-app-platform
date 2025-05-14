use serde_json::Value;
use std::collections::HashMap;

use super::super::data::{Bind, Layout};
use super::super::store::Store;
use super::{Dynamic, Frame};
use dioxus::prelude::*;

type FormScope = HashMap<String, Signal<Value>>;

fn walk(layout: &mut Layout, scope: &mut FormScope) {
    match layout.data {
        Some(Bind::Field { ref field }) => {
            let s = use_signal(|| Value::default());
            scope.insert(field.to_string(), s);
            layout.data = Some(Bind::Signal { signal: s });
        },
        Some(Bind::Confirm { ref confirm }) => {
            ();
        },
        _ => {}
    };
    if let Some(children) = &mut layout.children {
        for c in children.iter_mut() {
            walk(c, scope);
        }
    };
}

#[component]
pub fn Form(layout: Layout) -> Element {
    let mut data: FormScope = HashMap::new();
    walk(&mut layout, &mut data);
    let children = layout.clone().children.unwrap_or_else(|| vec![]);
    let children = children.into_iter().map(|c| {
        rsx! {
            Frame { layout: c }
        }
    });

    use_effect(move || {
        dioxus_logger::tracing::info!("{data:?}");
    });
    layout.kind = "container".to_owned();
    rsx! {
        Dynamic {
            layout: layout,
            {children}
        }
    }
}
