use dioxus::prelude::*;
use super::super::data::Layout;
use serde_json::to_value;

#[component]
pub fn Input(layout: Layout, children: Element) -> Element {
    rsx!{
        div {
            class: "input",
            {children}
        }
    }
}

#[component]
pub fn Text(layout: Layout, children: Element) -> Element {
    let t = layout
        .value
        .unwrap_or_else(|| to_value("").unwrap())
        .to_string();
    rsx!{
        div {
            class: "text",
            "{t}"
        }
    }
}


#[component]
pub fn Card(layout: Layout, children: Element) -> Element {
    rsx!{
        div {
            class: "card",
            {children}
        }
    }
}


#[component]
pub fn Button(layout: Layout, children: Element) -> Element {
    rsx!{
        div {
            class: "button",
            {children}
        }
    }
}
