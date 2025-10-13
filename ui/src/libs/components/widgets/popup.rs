use crate::libs::hooks::merge_css_class;
use dioxus::prelude::*;
use layout::{Bind, JsKind, Layout};
use serde_json::{Value, to_value};
use std::ops::Deref;
use std::rc::Rc;

#[component]
pub fn Popup(layout: Layout, children: Element) -> Element {
    rsx!()
}
