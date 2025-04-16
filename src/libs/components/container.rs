use super::super::data::Layout;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use serde_json::{from_str, to_value};

#[component]
pub fn Container(layout: Layout, children: Element) -> Element {
    let a = layout.attrs.unwrap_or(from_str("{}").unwrap());
    let a = a.as_object().unwrap();
    let mut css = vec!["Container", "f"];

    let h = a
        .get("horizontal")
        .and_then(|x| x.as_bool())
        .unwrap_or(false);
    if !h {
        css.push("v");
    }
    let css = css.join(" ");

    rsx! {
        div {
            class: css,
            {children}
        }
    }
}

#[component]
pub fn List(layout: Layout, children: Element) -> Element {
    rsx! {
        div {
            class: "List f v",
            {children}
        }
    }
}

#[component]
pub fn Card(layout: Layout, children: Element) -> Element {
    rsx! {
        div {
            class: "Card f v box border shadow",
            {children}
        }
    }
}
