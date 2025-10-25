use crate::libs::components::Frame;
use crate::libs::hooks::{use_common_css, use_source_list, use_source_value, use_target_value};
use dioxus::prelude::*;
use layout::{Layout, classify::Classify};
use serde_json::{Value, to_value};
use std::rc::Rc;

#[component]
pub fn Select(layout: Layout, children: Element) -> Element {
    let mut css = vec!["select"];
    let layout = Rc::new(layout);
    use_common_css(&mut css, &layout);
    let option = use_source_list(&layout, "options");
    let current = use_source_value(&layout);
    let mut current = use_signal(|| {
        current
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or("".to_string())
    });
    let mkclick = |value: Value| {
        let layout = layout.clone();
        move |_: MouseEvent| {
            let emitter = use_target_value(&layout);
            emitter.map(|x| x(value.clone()));
            if let Some(v) = value.as_str() {
                current.set(v.to_string());
            }
        }
    };
    if let Some(option) = option {
        let children = option.iter().enumerate().map(|(idx, child)| {
            let key = child.id.clone().unwrap_or(idx.to_string());
            let current = current();
            let mut child = child.clone();
            child.add_class("f nogrow s ax box");
            if current == key {
                child.add_class("selected");
                rsx! {
                    div {
                        Frame {
                            key: "{key}",
                            layout: child
                        }
                    }
                }
            } else {
                let v = match to_value(&key) {
                    Ok(v) => v,
                    Err(_) => to_value("").unwrap(),
                };
                rsx! {
                    div {
                        onclick: mkclick(v),
                        Frame {
                            key: "{key}",
                            layout: child
                        }
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
