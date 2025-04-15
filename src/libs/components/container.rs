use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn Container(children: Element) -> Element {
    rsx!{
        div {
            class: "container",
            {children}
        }
    }
}


#[component]
pub fn List(children: Element) -> Element {
    rsx!{
        div {
            class: "list",
            {children}
        }
    }
}
