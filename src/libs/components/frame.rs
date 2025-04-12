use super::super::data::Layout;
use super::Dynamic;
use dioxus::prelude::*;

#[component]
pub fn Frame(layout: Layout) -> Element {
    let children = layout.children.unwrap_or_else(|| vec![]);
    let children = children.iter().map(|c| {
        rsx! {
            Frame{ layout: c.clone() }
        }
    });

    rsx! {
        Dynamic {
            kind: layout.kind,
            {children}
        }
    }
}
