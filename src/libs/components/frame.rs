use super::super::data::Layout;
use super::Dynamic;
use dioxus::prelude::*;

#[component]
pub fn Frame(layout: Layout) -> Element {
    let children = layout.clone().children.unwrap_or_else(Vec::new);
    let children = children.into_iter().map(|c| {
        rsx! {
            Frame { layout: c }
        }
    });

    rsx! {
        Dynamic {
            layout: layout,
            {children}
        }
    }
}
