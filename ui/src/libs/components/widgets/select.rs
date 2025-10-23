use crate::libs::components::Frame;
use crate::libs::hooks::{
    use_common_css, use_source, use_source_list, use_source_value, use_target_value,
};
use dioxus::prelude::*;
use layout::{Bind, JsType, Layout, classify::Classify};
use maplit::hashmap;
use serde_json::{Value, to_value};

#[component]
pub fn Select(layout: Layout, children: Element) -> Element {
    let mut css = vec!["select", "shadow"];
    use_common_css(&mut css, &layout);
    let option = use_source_list(&layout, "options");
    let value = use_source_value(&layout);
    let mut value = use_signal(|| {
        value
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or("".to_string())
    });
    let mut signal = use_target_value(&layout);
    if let Some(option) = option {
        let children = option.iter().enumerate().map(|(idx, child)| {
            let key = child.id.clone().unwrap_or(idx.to_string());
            let value = value();
            if value == key {
                let mut child = child.clone();
                child.add_class("selected");
                rsx! {
                    Frame {
                        key: "{key}",
                        layout: child
                    }
                }
            } else {
                rsx! {
                    Frame {
                        key: "{key}",
                        layout: child.clone()
                    }
                }
            }
        });
        rsx! {
            div {
                class: css.join(" "),
                {children}
            }
        }
    } else {
        rsx!()
    }
}
