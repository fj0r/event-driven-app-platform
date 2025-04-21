use super::super::data::Layout;
use super::super::store::Store;
use super::dynamic::Dynamic;
use dioxus::prelude::*;

#[component]
pub fn X(layout: Layout, children: Element) -> Element {
    let c = if let Some(i) = layout.item {
        let x = i[0].clone();

        let s = use_context::<Store>();
        let v = use_memo(move || {
            let t = if let Some(b) = &layout.data {
                if !b.upload {
                    let x = s.list.read().get(&b.event).cloned();
                }
            };
        });
        rsx! {
            Dynamic {
                layout: x,
                {children}
            }
        }
    } else {
        children
    };
    rsx! {
        div {
            class: "List f v",
            {c}
        }
    }
}
