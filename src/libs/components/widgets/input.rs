use super::super::super::data::{Bind, Layout, Settings};
use super::super::super::store::Store;
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use serde_json::{to_value, Value};

#[component]
pub fn Input(layout: Layout) -> Element {
    let instant = layout
        .attrs
        .clone()
        .and_then(|x| {
            if let Some(Settings::Input { instant }) = x.settings {
                Some(instant)
            } else {
                None
            }
        })
        .unwrap_or(false);

    let (ty, event, kind, signal) = layout
        .data
        .clone()
        .and_then(|x| match x {
            Bind::Field {
                field,
                kind,
                signal,
            } => Some(("field", field, kind.unwrap_or("text".to_string()), signal)),
            Bind::Event {
                event,
                kind,
                local: _,
            } => Some(("event", event, kind.unwrap_or("text".to_string()), None)),
            _ => None,
        })
        .unwrap_or(("", "".to_string(), "text".to_string(), None));

    let s = use_context::<Store>();
    let mut css = vec!["input", "f", "shadow"];
    let css = merge_css_class(&mut css, &layout);

    let input_type = match kind.as_str() {
        "bool" => "checkbox",
        "number" => "number",
        "password" => "password",
        "button" => "button",
        "submit" => "submit",
        _ => "text",
    };

    let mut v = signal.unwrap_or_else(|| {
        use_signal(|| {
            let v = match kind.as_str() {
                "number" => to_value(0),
                "bool" => to_value(false),
                _ => to_value(""),
            }
            .unwrap();
            v
        })
    });

    let kind_clone = kind.clone();
    let oninput = move |event: Event<FormData>| {
        v.set(to_value(event.value()).unwrap());
        if let Some(mut signal) = signal {
            let vl = event.value();
            let vl = match kind_clone.as_str() {
                "bool" => to_value(vl == "true"),
                "number" => to_value(vl.parse::<f64>().unwrap()),
                _ => to_value(vl),
            }
            .unwrap();
            signal.set(vl);
            // TODO: instant
            if instant {
                todo!()
            }
        };
    };

    let onkeydown = move |ev: Event<KeyboardData>| {
        let mut s = s.clone();
        let event = event.clone();
        async move {
            if ev.data.key() == Key::Enter {
                // TODO: remove convert to string
                let val = to_value(v.read().to_string()).unwrap();
                match ty {
                    "field" => {
                        if let Some(mut sig) = signal {
                            sig.set(val);
                        };
                    }
                    "event" => {
                        s.send(event, None, val).await;
                        *v.write() = to_value("".to_string()).unwrap();
                    }
                    _ => {}
                }
            }
        }
    };

    match kind.as_str() {
        "number" => {
            let v = v.read().as_f64();
            rsx! {
                input {
                    class: css.join(" "),
                    type: input_type,
                    value: v,
                    oninput: oninput,
                    onkeydown: onkeydown
                }
            }
        }
        "bool" => {
            let v = v.read().as_bool();
            rsx! {
                input {
                    class: css.join(" "),
                    type: input_type,
                    value: v,
                    oninput: oninput,
                    onkeydown: onkeydown
                }
            }
        }
        _ => {
            let v = v.read().as_str().unwrap_or("").to_string();
            rsx! {
                input {
                    class: css.join(" "),
                    type: input_type,
                    value: v,
                    oninput: oninput,
                    onkeydown: onkeydown
                }
            }
        }
    }
}
