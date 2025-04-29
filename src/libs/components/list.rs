use super::super::data::Layout;
use super::super::store::Store;
use super::utils::merge_css_class;
use super::Dynamic;
use dioxus::prelude::*;

#[component]
pub fn List(layout: Layout, children: Element) -> Element {
    let mut css = vec!["list", "f"];
    let css = merge_css_class(&mut css, &layout);

    let i = &layout.clone().item.context("item")?[0];
    let b = layout.data.clone().context("data")?;

    let s = use_context::<Store>();
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
                key: child.id.unwrap_or(idx.to_string()),
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
