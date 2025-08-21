use super::super::super::store::Store;
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use layout::{Bind, JsKind, Layout};
use serde_json::{Value, to_value};
use std::ops::Deref;
use std::rc::Rc;

fn default_option_jskind(v: &Option<JsKind>) -> Value {
    v.as_ref()
        .map(|x| x.default_value())
        .unwrap_or_else(|| to_value("").unwrap())
}

#[component]
pub fn Input(layout: Layout) -> Element {
    let store = use_context::<Store>();
    let mut css = vec!["input", "f", "shadow"];
    let css = merge_css_class(&mut css, &layout);

    return match &layout.bind {
        Some(Bind::Field { field, kind, payload, signal }) => { rsx!() }
        Some(Bind::Event { event, kind }) => { rsx!() }
        Some(Bind::Variable { variable, kind }) => { rsx!() }
        _ => { rsx!() }
    };
}
