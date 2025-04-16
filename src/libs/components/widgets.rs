use super::super::data::Layout;
use super::super::store::Store;
use super::Dynamic;
use dioxus::prelude::*;
use serde_json::to_value;

#[component]
pub fn Input(layout: Layout, children: Element) -> Element {
    rsx! {
        input {
            {children}
        }
    }
}

#[component]
pub fn Text(layout: Layout, children: Element) -> Element {
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
            j.to_string()
        } else {
            "".to_string()
        }
    });

    rsx! {
        div {
            {v}
        }
    }
}


#[component]
pub fn Button(layout: Layout, children: Element) -> Element {
    rsx! {
        button {
            {children}
        }
    }
}
