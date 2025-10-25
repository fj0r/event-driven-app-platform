use crate::libs::hooks::use_common_css;
use crate::libs::hooks::{use_source, use_source_value, use_target_value};
use dioxus::prelude::*;
use layout::Layout;
use serde_json::to_value;
use std::rc::Rc;

#[component]
pub fn TextArea(id: String, layout: Layout) -> Element {
    let mut css = vec!["textarea", "shadow"];
    let layout = Rc::new(layout);
    use_common_css(&mut css, &layout);
    let value = use_source_value(&layout);
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
