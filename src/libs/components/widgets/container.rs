use super::super::super::data::{Container as Ct, Table, Layout, Settings};
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn Case(layout: Layout, children: Element) -> Element {
    let mut css = vec!["case"];
    let mut style = String::new();
    if let Some(a) = &layout.attrs {
        let mut f = true;
        if let Some(Settings::Container(c)) = &a.settings {
            match &c {
                Ct::grid(g) => {
                    f = false;
                    css.push("g");
                    style = g
                        .iter()
                        .map(|(k, v)| format!("{}: {};", k, v.as_str().unwrap()))
                        .join("\n");
                }
            }
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
pub fn Fold(layout: Layout, children: Element) -> Element {
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
pub fn Pop(layout: Layout, children: Element) -> Element {
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
pub fn Switch(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

