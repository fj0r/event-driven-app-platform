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
    let mut css = vec!["input", "f", "shadow"];
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
                        // TODO: event
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
pub fn Text(layout: ReadOnlySignal<Layout>) -> Element {
    let mut css = vec!["text", "f", "txt"];

    let layout_cloned = layout();
    let css = merge_css_class(&mut css, &layout_cloned);

    let s = use_context::<Store>();

    let v = use_memo(move || {
        let mut t = {
            let value = layout.read().value.clone();
            Layout {
                kind: "Text".to_string(),
                value,
                ..Layout::default()
            }
        };
        if let Some(b) = &layout.read().data {
            if !b.upload {
                let x = s.data.read().get(&b.event).cloned();
                if let Some(t1) = x {
                    t = t1
                }
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
        if let Some(a) = get_attrs(layout.read().clone(), "format") {
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
