use super::super::data::Layout;
use super::super::store::Store;
use super::utils::merge_css_class;
use super::{Dynamic, Frame};
use dioxus::prelude::*;

#[component]
pub fn Form(layout: Layout) -> Element {
    dioxus_logger::tracing::info!("{layout:?}");

    let children = layout.clone().children.unwrap_or_else(|| vec![]);
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
