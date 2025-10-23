use crate::libs::store::Store;
use dioxus::prelude::*;
use layout::{Bind, BindVariant, Layout, Settings};
use maplit::hashmap;
use serde_json::{Value, json};
use std::default::Default;
use std::ops::Deref;

pub fn merge_css_class<'a>(css: &'a mut Vec<&'a str>, layout: &'a Layout) -> &'a mut Vec<&'a str> {
    let mut v = ["box", "case", "rack", "text", "tab", "menu"].contains(&layout.kind.as_str());
    if let Some(a) = layout.attrs.as_ref() {
        if let Some(h) = a.horizontal {
            if h {
                v = false;
            }
        }
        if let Some(cc) = &a.class {
            css.push(cc);
        }
    }
    if v {
        css.push("v");
    }
    css
}

pub fn use_default<'a>(layout: &'a Layout) -> Option<Value> {
    layout
        .bind
        .as_ref()
        .and_then(|x| x.get("value"))
        .and_then(|x| x.default.clone())
}

pub fn use_source_id<'a>(layout: &'a Layout) -> Option<&'a String> {
    if let Bind {
        variant: BindVariant::Source { source },
        ..
    } = layout.bind.as_ref().and_then(|x| x.get("value"))?
    {
        Some(source)
    } else {
        None
    }
}

pub fn use_source_list<'a>(layout: &'a Layout, key: &'a str) -> Option<Vec<Layout>> {
    let store = use_context::<Store>();
    let s = store.list.read();
    if let Some(x) = layout.bind.as_ref()
        && let Some(Bind {
            variant: BindVariant::Source { source },
            default: _,
            r#type: _kind,
        }) = x.get(key)
        && let list = s.get(source)
        && list.is_some()
    {
        list.cloned()
    } else {
        Some(Vec::new())
    }
}

pub fn use_source<'a>(layout: &'a Layout, key: &'a str) -> Option<Value> {
    let store = use_context::<Store>();
    let s = store.data.read();
    let value = if let Some(x) = layout.bind.as_ref()
        && let Some(Bind {
            variant: BindVariant::Source { source },
            default: _,
            r#type: _kind,
        }) = x.get(key)
        && let data = s.get(source)
        && data.is_some()
    {
        data
    } else {
        Some(layout)
    };
    if let Some(layout) = value
        && let Some(bind) = &layout.bind
        && let Some(value) = bind.get(key)
    {
        value.default.clone()
    } else {
        None
    }
}

pub fn use_source_value(layout: &Layout) -> Option<Value> {
    use_source(layout, "value")
}

pub fn use_target<'a>(layout: &'a Layout, key: &'a str) -> Signal<Value> {
    let signal = use_signal::<Value>(|| Default::default());
    if let Some(x) = layout.bind.as_ref()
        && let Some(Bind {
            // TODO: variable
            variant:
                BindVariant::Target {
                    event,
                    target,
                    silent,
                },
            default: _,
            r#type: _,
        }) = x.get(key)
    {
        let ev = event.clone();
        let store = use_context::<Store>();
        use_resource(move || {
            let target = ev.clone();
            let mut store = store.clone();
            async move { store.send(target, None, signal()).await }
        });
    };
    return signal;
}

pub fn use_target_value(layout: &Layout) -> Signal<Value> {
    use_target(layout, "value")
}
