use super::super::data::{Container as Ct, Layout, Settings};
use super::utils::merge_css_class;
use super::Frame;
use dioxus::prelude::*;

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
                        .collect::<Vec<String>>()
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
    let mut css = vec!["f", "v"];
    let css = merge_css_class(&mut css, &layout);

    let Some((replace_header, float_body)) = layout.attrs.as_ref().map(|x| {
        let x = if let Some(Settings::Fold {
            replace_header,
            float_body,
        }) = x.settings
        {
            (replace_header, float_body)
        } else {
            (false, false)
        };
        x
    }) else {
        unreachable!()
    };


    let item = layout.item.as_ref().context("item")?[0].clone();
    let show = use_signal(|| {
        layout
            .value
            .clone()
            .and_then(|x| x.as_bool())
            .unwrap_or_default()
    });

    let b = if show() {
        if replace_header {
            rsx! {
                div {
                    class: "f v",
                    {children}
                }
            }
        } else {
            rsx! {
                Frame { layout: item }
                div {
                    class: "f v",
                    {children}
                }
            }
        }
    } else {
        rsx! {
            Frame { layout: item }
        }
    };

    let icon_style = r#"
        width: 1em;
    "#;
    let icon_class = if show() { "icon open" } else { "icon close " };
    rsx! {
        div {
            class: "f",
            div {
                class: icon_class,
                style: "{icon_style}",
            },
            div {
                onclick: move |_event| {
                    let mut s = show.clone();
                    s.set(!show());
                },
                class: css.join(" "),
                {b}
            }
        }
    }
}

#[component]
pub fn Switch(layout: Layout, children: Element) -> Element {
    let mut css = vec!["switch", "f"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}
