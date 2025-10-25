#![allow(unused_imports)]
use crate::libs::hooks::use_common_css;
use dioxus::prelude::*;
use layout::{Bind, JsType, Layout};
use serde_json::{Value, to_value};
use std::ops::Deref;
use std::rc::Rc;

#[component]
pub fn Popup(layout: Layout, children: Element) -> Element {
    rsx!()
}
