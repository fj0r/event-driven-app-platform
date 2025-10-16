use crate::libs::hooks::{merge_css_class, use_default};
use dioxus::prelude::*;
use layout::{Bind, JsType, Layout};
use serde_json::{Value, to_value};
use std::ops::Deref;
use std::rc::Rc;

#[component]
pub fn Select(layout: Layout, children: Element) -> Element {
    rsx!()
}
