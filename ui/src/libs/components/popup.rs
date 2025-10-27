#![allow(unused_imports)]
use crate::libs::components::Frame;
use crate::libs::hooks::{use_common_css, use_source_value};
use dioxus::prelude::*;
use layout::{Bind, JsType, Layout};
use serde_json::{Value, to_value};
use std::ops::Deref;
use std::rc::Rc;

#[component]
pub fn Popup(layout: Layout, children: Element) -> Element {
    let mut css = vec!["popup", "f"];
    use_common_css(&mut css, &layout);
    let current = use_source_value(&layout);
    if let Some(children) = layout.children
        && let Some(placeholder) = children.get(0)
        && let Some(modal) = children.get(1)
    {
        rsx! {
            div {
                Frame {
                    layout: placeholder.clone()
                }
                Frame {
                    layout: modal.clone()
                }
            }
        }
    } else {
        rsx!()
    }
}
