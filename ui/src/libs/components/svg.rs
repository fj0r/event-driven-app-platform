use super::utils::merge_css_class;
use dioxus::prelude::*;
use layout::{Container as Ct, Layout, Settings};

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

    let mut style = String::new();
    if let Some(x) = &layout.attrs
        && let Some(Settings::Svg { svg }) = &x.settings
    {
        style = svg
            .iter()
            .map(|(k, v)| format!("{}: {};", k, v.as_str().unwrap()))
            .collect::<Vec<String>>()
            .join("\n");
    };
    rsx! {
        g {
            class: css.join(" "),
            style: style,
            {children}
        }
    }
}

#[component]
pub fn Path(layout: Layout, children: Element) -> Element {
    let mut css = vec!["path"];
    let css = merge_css_class(&mut css, &layout);
    if let Some(x) = layout
        .bind
        .as_ref()
        .and_then(|x| x.get("value"))
        .and_then(|x| x.default.clone())
        && let Some(d) = x.as_str()
    {
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
