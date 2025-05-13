use super::super::data::{Layout, Bind};
use super::super::store::Store;
use super::{Dynamic, Frame};
use dioxus::prelude::*;

fn walk(layout: &Layout) {

}

#[component]
pub fn Form(layout: Layout) -> Element {
    dioxus_logger::tracing::info!("{layout:?}");

    let children = layout.clone().children.unwrap_or_else(|| vec![]);
    walk(&layout);
    let children = children.into_iter().map(|c| {
        rsx! {
            Frame { layout: c }
        }
    });
    layout.kind = "container".to_owned();
    rsx! {
        Dynamic {
            layout: layout,
            {children}
        }
    }
}
