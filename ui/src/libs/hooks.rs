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

#[derive(PartialEq, Eq)]
pub struct BindKey(String);

impl Default for BindKey {
    fn default() -> Self {
        BindKey("value".to_string())
    }
}

impl Deref for BindKey {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn use_value<'a>(layout: &'a Layout, key: BindKey) -> Option<Value> {
    let store = use_context::<Store>();
    let s = store.data.read();
    let value = if let Some(x) = layout.bind.as_ref()
        && let Some(Bind {
            variant: BindVariant::Source { source },
            default: _,
            r#type: _kind,
        }) = x.get(key.deref())
        && let data = s.get(source)
        && data.is_some()
    {
        data
    } else {
        Some(layout)
    };
    if let Some(layout) = value
        && let Some(bind) = &layout.bind
        && let Some(value) = bind.get(key.deref())
    {
        value.default.clone()
    } else {
        None
    }
}
