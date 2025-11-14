use crate::libs::hooks::use_common_css;
use crate::libs::hooks::{use_source, use_source_value, use_target_value};
use brick::TextArea;
use dioxus::prelude::*;
use serde_json::to_value;
use std::rc::Rc;

#[component]
pub fn textarea_(id: Option<String>, brick: TextArea) -> Element {
    let mut css = vec!["textarea", "shadow"];
    let layout = Rc::new(brick);
    use_common_css(&mut css, &brick);
    let value = use_source_value(&brick);
    let mut slot = use_signal(|| value.unwrap_or_else(|| Default::default()));

    let placeholder = if let Some(d) = use_source(&layout, "placeholder") {
        d.as_str().unwrap().to_owned()
    } else {
        "".to_string()
    };

    let oninput = move |event: Event<FormData>| {
        slot.set(to_value(event.value()).unwrap());
    };
    let layout_rc = layout.clone();
    let onkeydown = move |ev: Event<KeyboardData>| {
        let layout = layout_rc.clone();
        async move {
            let emitter = use_target_value(&layout);
            if ev.data.key() == Key::Enter {
                emitter.map(|x| x(slot()));
                slot.set(Default::default());
            }
        }
    };

    let value = slot.read().as_str().unwrap_or("").to_string();
    rsx! {
        input {
            class: css.join(" "),
            value: value,
            placeholder: placeholder,
            oninput: oninput,
            onkeydown: onkeydown
        }
    }
}
