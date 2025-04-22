use super::super::data::Layout;
use super::super::store::Store;
use super::utils::{get_attrs, merge_css_class};
use comrak::{markdown_to_html, Options};
use dioxus::prelude::*;
use serde_json::to_value;
use std::sync::LazyLock;

#[component]
pub fn Input(layout: Layout) -> Element {
    let mut x = use_signal(|| "".to_string());
    let mut s = use_context::<Store>();
    let mut css = vec!["input", "f"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        input {
            class: css.join(" "),
            value: x,
            oninput: move |event| {
                x.set(event.value())
            },
            onkeydown: move |event| {
                let mut s = s.clone();
                async move {
                    if event.data.key() == Key::Enter {
                        s.send("x", None, to_value(x.read().to_string()).unwrap()).await;
                        // x.set("".to_string())
                        *x.write() = "".to_string()
                    }
                }
            }
        }
    }
}

#[component]
pub fn Text(layout: Layout) -> Element {
    let mut css = vec!["text", "f"];
    let l = layout.clone();
    let css = merge_css_class(&mut css, &l);

    let s = use_context::<Store>();

    let v = use_memo(move || {
        let t = if let Some(b) = &layout.data {
            if !b.upload {
                let x = s.data.read().get(&b.event).cloned();
                x.unwrap_or_else(|| Layout::new("Text"))
            } else {
                Layout::new("Text")
            }
        } else {
            let value = layout.value.clone();
            Layout {
                kind: "Text".to_string(),
                value,
                ..Layout::default()
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

        static MDFMT: LazyLock<Vec<&str>> = LazyLock::new(|| vec!["markdown", "md"]);
        if let Some(a) = get_attrs(layout.clone(), "format") {
            if a.is_string() && (*MDFMT).contains(&a.as_str().unwrap()) {
                markdown_to_html(&v, &Options::default())
            } else {
                v
            }
        } else {
            v
        }
    });

    rsx! {
        div {
            class: css.join(" "),
            {v}
        }
    }
}

#[component]
pub fn Button(layout: Layout) -> Element {
    let t = layout
        .value
        .unwrap_or(to_value("Ok").unwrap())
        .as_str()
        .unwrap()
        .to_owned();

    rsx! {
        button {
            class: "button",
            {t}
        }
    }
}
