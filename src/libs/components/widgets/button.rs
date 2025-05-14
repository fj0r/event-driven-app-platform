use super::super::super::data::{Layout, Bind};
use dioxus::prelude::*;
use serde_json::{to_value, Value};

#[component]
pub fn Button(layout: Layout) -> Element {
    let t = layout
        .value
        .unwrap_or(to_value("Ok").unwrap())
        .as_str()
        .unwrap()
        .to_owned();

    rsx! {
        button {
            class: "button",
            onclick: move |_event| {
                if let Some(Bind::Signal { signal: mut s }) = layout.data {
                    let v = s.read().as_bool().unwrap();
                    s.set(Value::Bool(!v));
                }
            },
            {t}
        }
    }
}
