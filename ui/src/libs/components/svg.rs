use crate::libs::hooks::{use_common_css, use_default};
use dioxus::prelude::*;
use layout::{Layout, Settings};

#[component]
pub fn Svg(layout: Layout, children: Element) -> Element {
    let mut css = vec!["svg"];
    use_common_css(&mut css, &layout);
    let style = if let Some(a) = &layout.attrs
        && let Some(s) = &a.size
    {
        s.into_style()
    } else {
        "".to_owned()
    };
    rsx! {
        svg {
            style: style,
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Group(layout: Layout, children: Element) -> Element {
    let mut css = vec!["group"];
    use_common_css(&mut css, &layout);

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
    use_common_css(&mut css, &layout);
    if let Some(x) = use_default(&layout)
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
