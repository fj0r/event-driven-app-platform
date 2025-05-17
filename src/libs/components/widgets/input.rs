use super::super::super::data::{Bind, Layout, Settings};
use super::super::super::store::Store;
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use serde_json::to_value;

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

    let mut v = signal.unwrap_or_else(|| {
        use_signal(|| {
            let x = layout
                .value
                .as_ref()
                .and_then(|x| x.as_str())
                .map(|x| x.to_owned())
                .unwrap_or("".to_string());
            to_value(x).unwrap_or_else(|_| to_value("").unwrap())
        })
    });

    let input_type = match kind.as_str() {
        "bool" => "checkbox",
        "number" => "number",
        "password" => "password",
        "button" => "button",
        "submit" => "submit",
        _ => "text",
    };

    let s = use_context::<Store>();
    let mut css = vec!["input"];
    let css = merge_css_class(&mut css, &layout);

    let oninput = move |event: Event<FormData>| {
        v.set(to_value(event.value()).unwrap());
        if let Some(mut signal) = signal {
            signal.set(to_value(event.value()).unwrap());
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

    let v1 = v.clone();
    let tv = use_memo(move || {
        let x = v1.read();
        x.as_str().unwrap_or_else(|| "").to_string()
    });

    rsx! {
        input {
            class: css.join(" "),
            type: input_type,
            value: tv,
            oninput: oninput,
            onkeydown: onkeydown
        }
    }
}
