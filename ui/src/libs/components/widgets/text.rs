use super::super::super::data::{Bind, Layout, Settings};
use super::super::super::store::Store;
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use markdown::{to_html_with_options, Options};
use std::sync::LazyLock;

#[component]
pub fn Text(layout: ReadOnlySignal<Layout>) -> Element {
    let mut css = vec!["text", "f", "txt"];

    let layout_cloned = layout();
    let css = merge_css_class(&mut css, &layout_cloned);

    let s = use_context::<Store>();

    let mut t = {
        let value = layout.read().value.clone();
        Layout {
            kind: "Text".to_string(),
            value,
            ..Layout::default()
        }
    };
    if let Some(Bind::Event { event, .. }) = &layout.read().bind {
        let x = s.data.read().get(event).cloned();
        if let Some(t1) = x {
            t = t1
        }
    };
    let v = if let Some(j) = t.value {
        if j.is_string() {
            j.as_str().unwrap().to_owned()
        } else {
            j.to_string()
        }
    } else {
        "".to_string()
    };

    static MDFMT: LazyLock<Vec<String>> = LazyLock::new(|| {
        ["markdown", "md"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    });

    if let Some(Settings::Text { format: a }) =
        layout.read().clone().attrs.and_then(|x| x.settings)
    {
        if (*MDFMT).contains(&a) {
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
        }
    };

    rsx! {
        div {
            class: css.join(" "),
            {v}
        }
    }
}
