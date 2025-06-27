use super::utils::merge_css_class;
use super::Frame;
use dioxus::prelude::*;
use layout::{Layout, Settings};

#[component]
pub fn Fold(layout: Layout, children: Element) -> Element {
    let mut css = vec!["g"];
    let css = merge_css_class(&mut css, &layout);

    let Some((replace_header, _float_body)) = layout.attrs.as_ref().map(|x| {
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
            .data
            .clone()
            .and_then(|x| x.as_bool())
            .unwrap_or_default()
    });

    let onclick = move |_event| {
        let mut s = show;
        s.set(!show());
    };

    let h = if replace_header && show() {
        rsx! { div {} }
    } else {
        rsx! {
            div {
                Frame { layout: item }
            }
        }
    };
    let b = if show() {
        rsx! {
            div {
                {children}
            }
        }
    } else {
        rsx! { div {} }
    };

    let icon_style = r#"
        height: 100%;
        aspect-ratio: 1 / 1;
    "#;
    let grid_style = r#"
        grid-template-columns: auto 1fr;
        "#;
    let icon_class = if show() { "icon open" } else { "icon close " };
    rsx! {
        div {
            class: css.join(" "),
            style: "{grid_style}",
            onclick: onclick,
            div {
                class: icon_class,
                style: "{icon_style}",
            },
            {h},
            div {},
            {b}
        }
    }
}
