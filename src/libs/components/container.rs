use super::super::data::Layout;
use super::super::store::Store;
use dioxus::prelude::*;
use super::utils::unwrap_or_object;
use super::Dynamic;

#[component]
pub fn Container(layout: Layout, children: Element) -> Element {
    let mut css = vec!["Container", "f"];

    let a = unwrap_or_object(layout.attrs);
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


    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn List(layout: Layout, children: Element) -> Element {
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
            {children}
        }
    }
}

#[component]
pub fn Card(layout: Layout, children: Element) -> Element {
    rsx! {
        div {
            class: "Card f v box border shadow",
            {children}
        }
    }
}

