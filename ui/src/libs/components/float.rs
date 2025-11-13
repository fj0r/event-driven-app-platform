#![allow(unused_imports)]
use crate::libs::components::Frame;
use crate::libs::hooks::{use_common_css, use_source_value};
use dioxus::prelude::*;
use layout::{Bind, JsType, Layout};
use serde_json::{Value, to_value};
use std::ops::Deref;
use std::rc::Rc;

#[component]
pub fn float_(layout: Layout, children: Element) -> Element {
    let mut css = vec!["float", "f"];
    use_common_css(&mut css, &layout);
    let style = if let Some(attr) = &layout.attrs
        && let Some(pos) = &attr.position
    {
        pos.into_position()
    } else {
        "".to_owned()
    };
    rsx! {
        div {
            style: style,
            class: css.join(" "),
            {children}
        }
    }
}
