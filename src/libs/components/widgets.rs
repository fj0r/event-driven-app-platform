use super::super::data::Layout;
use super::super::store::Store;
use super::Dynamic;
use dioxus::prelude::*;
use serde_json::to_value;

#[component]
pub fn Input(layout: Layout) -> Element {
    rsx! {
        input {
            class: "Input",
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
        if let Some(j) = t.value {
            j.as_str().unwrap().to_owned()
        } else {
            "".to_string()
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
