use super::super::data::Layout;
use serde_json::{from_str, Value};

pub fn get_attrs(layout: Layout, key: &str) -> Option<Value> {
    let a = layout.attrs.unwrap_or(from_str("{}").unwrap());
    if let Some(h) = a.as_object() {
        h.get(key).cloned()
    } else {
        None
    }
}

pub fn unwrap_or_object(value: Option<Value>) -> Value {
    value.unwrap_or(from_str("{}").unwrap())
}
