use crate::libs::hooks::merge_css_class;
use crate::libs::hooks::{use_source_value, use_target_value};
use dioxus::prelude::*;
use layout::{Bind, BindVariant, Layout, Settings};
use serde_json::to_value;

#[component]
pub fn TextArea(id: String, layout: Layout) -> Element {
    let mut css = vec!["textarea", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    let value = use_source_value(&layout);
    let mut slot = use_signal(|| value.unwrap_or_else(|| Default::default()));
    let mut signal = use_target_value(&layout);

    let oninput = move |event: Event<FormData>| {
        slot.set(to_value(event.value()).unwrap());
    };
    let onkeydown = move |ev: Event<KeyboardData>| {
        //
        async move {
            if ev.data.key() == Key::Enter {
                signal.set(slot());
                slot.set(Default::default());
            }
        }
    };

    let value = slot.read().as_str().unwrap_or("").to_string();
    rsx! {
        input {
            class: css.join(" "),
            value: value,
            oninput: oninput,
            onkeydown: onkeydown
        }
    }
}
