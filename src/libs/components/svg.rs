use super::super::data::{Container as Ct, Layout, Settings};
use super::utils::merge_css_class;
use dioxus::prelude::*;

#[component]
pub fn Svg(layout: Layout, children: Element) -> Element {
    let mut css = vec!["svg"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        svg {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Group(layout: Layout, children: Element) -> Element {
    let mut css = vec!["group"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        g {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Path(layout: Layout, children: Element) -> Element {
    let mut css = vec!["path"];
    let css = merge_css_class(&mut css, &layout);
    if let Some(d) = layout.value.as_ref().and_then(|x| x.as_str()) {
        rsx! {
            path {
                class: css.join(" "),
                d: d.to_string(),
                {children}
            }
        }
    } else {
        rsx! {}
    }
}
