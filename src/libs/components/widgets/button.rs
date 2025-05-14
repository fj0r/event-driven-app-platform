use super::super::super::data::Layout;
use dioxus::prelude::*;
use serde_json::to_value;

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
            {t}
        }
    }
}
