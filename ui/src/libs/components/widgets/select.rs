use crate::libs::components::Frame;
use crate::libs::hooks::{
    merge_css_class, use_source, use_source_list, use_source_value, use_target_value,
};
use dioxus::prelude::*;
use layout::{Bind, JsType, Layout};
use serde_json::{Value, to_value};

#[component]
pub fn Select(layout: Layout, children: Element) -> Element {
    let mut css = vec!["select", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    let value = use_source_value(&layout);
    let option = use_source_list(&layout, "option");
    let mut slot = use_signal(|| value.unwrap_or_else(|| Default::default()));
    let mut signal = use_target_value(&layout);
    if let Some(option) = option {
        let children = option.iter().enumerate().map(|(idx, child)| {
            let key = child.id.clone().unwrap_or(idx.to_string());
            rsx! {
                Frame {
                    key: "{key}",
                    layout: child.clone()
                }
            }
        });
        rsx! {
            div {
                class: css.join(" "),

            }
        }
    } else {
        rsx!()
    }
}
