use dioxus::prelude::*;
use dioxus_logger::tracing;
use super::super::data::Layout;

#[component]
pub fn Container(layout:Layout, children: Element) -> Element {
    rsx!{
        div {
            class: "container",
            {children}
        }
    }
}


#[component]
pub fn List(layout: Layout, children: Element) -> Element {
    rsx!{
        div {
            class: "list",
            {children}
        }
    }
}
