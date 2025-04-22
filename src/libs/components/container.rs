use super::super::data::Layout;
use super::super::store::Store;
use dioxus::prelude::*;
use super::utils::merge_css_class;
use super::Dynamic;

#[component]
pub fn Container(layout: Layout, children: Element) -> Element {
    let mut css = vec!["container", "f"];

    let css = merge_css_class(&mut css, &layout);

    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn List(layout: Layout, children: Element) -> Element {
    let mut css = vec!["list", "f"];
    let css = merge_css_class(&mut css, &layout);

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
            class: "card f v box border shadow",
            {children}
        }
    }
}

#[component]
pub fn Menu(layout: Layout, children: Element) -> Element {
    rsx! {
        div {
            class: "card f v box border shadow",
            {children}
        }
    }
}

#[component]
pub fn Card(layout: Layout, children: Element) -> Element {
    rsx! {
        div {
            class: "card f v box border shadow",
            {children}
        }
    }
}

