use super::super::data::Layout;
use super::super::store::Store;
use dioxus::prelude::*;
use serde_json::to_value;
use comrak::{markdown_to_html, Options};
use super::utils::get_attrs;
use std::sync::LazyLock;

#[component]
pub fn Input(layout: Layout) -> Element {
    let mut x = use_signal(|| "".to_string());
    let r = use_resource(move || async move {
        let x = x.read();
        let mut s = use_context::<Store>();
        let _ = s.send("x", to_value(x.to_string()).unwrap()).await;
    });
    rsx! {
        input {
            class: "Input",
            oninput: move |event| x.set(event.value())
        }
    }
}

#[component]
pub fn Text(layout: Layout) -> Element {
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
            if a.is_string() && (*MDFMT).contains(&a.as_str().unwrap())  {
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
            class: "Text",
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
            class: "Button",
            {t}
        }
    }
}

#[component]
pub fn Test(layout: Layout) -> Element {
    let mut count = use_signal(|| 1);
    rsx! {
        div {
            "{count}"
        }
        button {
            onclick: move |_| count += 1,
            "Count"
        }
    }
}
