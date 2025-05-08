use crate::libs::components::utils::get_attrs;

use super::super::data::Layout;
use super::super::store::Store;
use super::types::Ele;
use super::utils::merge_css_class;
use super::Dynamic;
use dioxus::web::WebEventExt;
use dioxus::{prelude::*, CapturedError};
use std::sync::{LazyLock, Mutex};

#[component]
pub fn List(layout: Layout, children: Element) -> Element {
    static LIST_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
    let mut tc = LIST_ID.lock().unwrap();
    *tc += 1;
    let id = format!("list-{}", *tc);

    let mut css = vec!["list", "f"];
    let css = merge_css_class(&mut css, &layout);

    let item = layout.item.as_ref().context("item")?;
    let data_bind = layout.data.as_ref().context("data")?;
    let attrs = layout.attrs.as_ref().context("attrs")?;

    let s = use_context::<Store>();
    let c = s.list.read();
    let c = c
        .get(&data_bind.event)
        .cloned()
        .unwrap_or_else(|| Vec::new());
    let r = c.iter().enumerate().map(|(idx, child)| {
        let x = rsx! {
            Dynamic {
                layout: child.clone()
            }
        };
        let key = child.id.clone().unwrap_or(idx.to_string());
        let layout = item[0].clone();
        if c.len() - 1 == idx {
            // last element
            rsx! {
                Dynamic {
                    key: "{key}",
                    layout: layout,
                    {x}
                }
            }
        } else {
            rsx! {
                Dynamic {
                    key: "{key}",
                    layout: layout,
                    {x}
                }
            }
        }
    });

    if let Some(f) = attrs.as_object() {
        let x = f.get("scroll").and_then(|x| x.as_bool()).unwrap_or(false);
        if x {
            let sl = s.list.clone();
            let eid = id.clone();
            use_effect(move || {
                let _ = sl.read();
                document::eval(&format!(
                    r#"
                var e = document.getElementById("{eid}");
                e.scrollTop = e.scrollHeight;
            "#
                ));
            });
        }
    };

    rsx! {
        div {
            id: id,
            class: css.join(" "),
            {r}
        }
    }
}
