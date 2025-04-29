use super::super::data::Layout;
use super::super::store::Store;
use super::utils::merge_css_class;
use super::Dynamic;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use dioxus_logger::tracing;
use std::rc::Rc;

#[component]
pub fn List(layout: Layout, children: Element) -> Element {
    let mut tail: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut css = vec!["list", "f"];
    let css = merge_css_class(&mut css, &layout);

    let item0 = &layout.clone().item.context("item")?[0];
    let data_bind = layout.data.clone().context("data")?;

    let s = use_context::<Store>();
    let c = s.list.read();
    let c = c
        .get(&data_bind.event)
        .cloned()
        .unwrap_or_else(|| Vec::new());
    let r = c.iter().enumerate().map(|(idx, child)| {
        let x = if c.len() - 1 == idx {
            // last element
            rsx! {
                Dynamic {
                    layout: child.clone()
                }
            }
        } else {
            rsx! {
                Dynamic {
                    layout: child.clone()
                }
            }
        };
        rsx! {
            Dynamic {
                key: child.id.unwrap_or(idx.to_string()),
                layout: item0.clone(),
                {x}
            }
        }
    });

    rsx! {
        div {
            class: css.join(" "),
            {r}
        }
        button {
            onclick: move |_e| {
                if let Some(x) = tail().as_ref() {
                    let _ = x.scroll_to(ScrollBehavior::Smooth);
                }
            },
            "scroll"
        }
    }
}
