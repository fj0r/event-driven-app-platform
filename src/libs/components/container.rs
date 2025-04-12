use dioxus::prelude::*;

#[component]
pub fn Container(children: Element) -> Element {
    rsx!{
        {children}
    }
}
