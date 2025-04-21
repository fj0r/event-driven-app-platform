#![allow(unused_imports)]
use super::super::data::Layout;
use super::super::store::Store;
use super::dynamic::Dynamic;
use super::utils::*;
use anyhow::Ok;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn X(layout: Layout, children: Element) -> Element {
    let mut css = vec!["List", "f"];
    let l = layout.clone();
    let a = unwrap_or_object(l.attrs);
    if let Some(a) = a.as_object() {
        let h = a
            .get("horizontal")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);
        if !h {
            css.push("v");
        }
        let cc = a.get("class").and_then(|x| x.as_str()).unwrap_or("");
        css.push(cc);
    };

    let s = use_context::<Store>();
    let i = &layout.clone().item.context("item")?[0];
    let b = layout.data.clone().context("data")?;
    let c = s.list.read();
    let c = c.get(&b.event).cloned().unwrap_or_else(|| Vec::new());
    let r = c.iter().enumerate().map(|(idx, child)| {
        let x = rsx! {
            Dynamic {
                layout: child.clone()
            }
        };
        rsx! {
            Dynamic {
                key: idx,
                layout: i.clone(),
                {x}
            }
        }
    });
    rsx! {
        div {
            class: css.join(" "),
            {r}
        }
    }
}
