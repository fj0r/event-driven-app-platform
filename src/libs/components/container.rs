use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn Container(children: Element) -> Element {
    rsx!{
        {children}
    }
}


#[component]
pub fn List(children: Element) -> Element {
    tracing::info!("{children:?}");
    rsx!{
        {children}
    }
}
