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

#[component]
pub fn Tab(layout: Layout, children: Element) -> Element {
    rsx! {
        div {
            class: "Card f v box border shadow",
            {children}
        }
    }
}

#[component]
pub fn Menu(layout: Layout, children: Element) -> Element {
    rsx! {
        div {
            class: "Card f v box border shadow",
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

