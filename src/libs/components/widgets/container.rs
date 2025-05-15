use super::super::super::data::{Layout, Settings};
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn Container(layout: Layout, children: Element) -> Element {
    let mut css = vec!["container"];
    let mut style = String::new();
    if let Some(a) = &layout.attrs {
        let mut f = true;
        if let Some(Settings::Container { table, grid }) = &a.settings {
            if let Some(t) = table {
                f = false;
            };
            if let Some(g) = grid {
                f = false;
                css.push("g");
                style = g
                    .iter()
                    .map(|(k, v)| format!("{}: {};", k, v.as_str().unwrap()))
                    .join("\n");
            };
        };
        if f {
            css.push("f");
        }
    };
    let css = merge_css_class(&mut css, &layout);

    rsx! {
        div {
            class: css.join(" "),
            style: style,
            {children}
        }
    }
}

#[component]
pub fn Tab(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Menu(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Card(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}
