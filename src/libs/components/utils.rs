use super::super::data::Layout;
use serde_json::{from_str, Value};
use std::sync::LazyLock;

pub fn get_attrs(layout: Layout, key: &str) -> Option<Value> {
    let a = layout.attrs.unwrap_or(from_str("{}").unwrap());
    let Some(h) = a.as_object() else {
        return None;
    };
    h.get(key).cloned()
}

pub fn unwrap_or_object(value: Option<Value>) -> Value {
    value.unwrap_or(from_str("{}").unwrap())
}

static DEFAULT_ATTRS: LazyLock<Value> =
    LazyLock::new(|| from_str("{}").expect("Failed to parse default empty JSON object"));

pub fn merge_css_class<'a>(css: &'a mut Vec<&'a str>, layout: &'a Layout) -> &'a mut Vec<&'a str> {
    let attrs = layout.attrs.as_ref().unwrap_or(&DEFAULT_ATTRS);
    if let Some(a) = attrs.as_object() {
        let h = a
            .get("horizontal")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);
        if !h {
            css.push("v");
        }
        if let Some(cc) = a.get("class").and_then(|x| x.as_str()) {
            css.push(cc);
        }
    }
    css
}
