use super::super::super::store::Store;
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use layout::{Bind, Layout, Settings};
use markdown::{Options, to_html_with_options};
use std::sync::LazyLock;

#[component]
pub fn Text(layout: ReadOnlySignal<Layout>) -> Element {
    let mut css = vec!["text", "f", "txt"];

    let layout_cloned = layout();
    let css = merge_css_class(&mut css, &layout_cloned);

    let s = use_context::<Store>();

    let mut t = {
        let data = layout.read().data.clone();
        Layout {
            kind: "Text".to_string(),
            data,
            ..Layout::default()
        }
    };
    if let Some(Bind::Event { event, .. }) = &layout.read().bind {
        let x = s.data.read().get(event).cloned();
        if let Some(t1) = x {
            t = t1
        }
    };
    let v = if let Some(j) = t.data {
        if j.is_string() {
            j.as_str().unwrap().to_owned()
        } else {
            j.to_string()
        }
    } else {
        "".to_string()
    };

    static MDFMT: LazyLock<Vec<String>> =
        LazyLock::new(|| ["markdown", "md"].iter().map(|x| x.to_string()).collect());

    if let Some(x) = layout.read().clone().attrs
        && let Some(Settings::Text { format: a }) = x.settings
        && (*MDFMT).contains(&a)
    {
        let v = v.clone();
        if let Ok(md) = to_html_with_options(&v, &Options::gfm()) {
            css.push("markdown");
            return rsx! {
                div {
                    class: css.join(" "),
                    dangerous_inner_html: md
                }
            };
        }
    };

    rsx! {
        div {
            class: css.join(" "),
            {v}
        }
    }
}
