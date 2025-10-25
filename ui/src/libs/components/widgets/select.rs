use crate::libs::components::Frame;
use crate::libs::hooks::{use_common_css, use_source_list, use_source_value, use_target_value};
use dioxus::prelude::*;
use layout::{Layout, classify::Classify};
use serde_json::{Value, to_value};

#[component]
pub fn Select(layout: Layout, children: Element) -> Element {
    let mut css = vec!["select"];
    use_common_css(&mut css, &layout);
    let option = use_source_list(&layout, "options");
    let value = use_source_value(&layout);
    let value = use_signal(|| {
        value
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or("".to_string())
    });
    let mut emitter = use_target_value(&layout);
    let mkclick = |value: Value| move |_: MouseEvent| emitter.map(|x| x(value.clone()));
    let x = async || ();
    let y = || async {};
    if let Some(option) = option {
        let children = option.iter().enumerate().map(|(idx, child)| {
            let key = child.id.clone().unwrap_or(idx.to_string());
            let value = value();
            let mut child = child.clone();
            child.add_class("f nogrow s ax box");
            if value == key {
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
