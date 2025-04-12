use dioxus::prelude::*;
use super::super::data::Layout;
use super::Dynamic;

#[component]
pub fn Frame(layout: Layout) -> Element {
    rsx! {
        Dynamic {
            kind: layout.kind,
            "fasd"
        }
    }
}
