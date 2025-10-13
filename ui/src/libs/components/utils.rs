use dioxus::prelude::*;
use layout::{Bind, BindVariant, Layout, Settings};
use serde_json::{Value, json};

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
