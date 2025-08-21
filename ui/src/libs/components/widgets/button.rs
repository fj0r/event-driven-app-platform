use dioxus::prelude::*;
use layout::{Bind, Layout, Settings};
use serde_json::{Value, to_value};

#[component]
pub fn Button(layout: Layout) -> Element {
    let t = layout
        .value
        .unwrap_or(to_value("Ok").unwrap())
        .as_str()
        .unwrap()
        .to_owned();

    let oneshot = layout
        .attrs
        .and_then(|x| {
            if let Some(Settings::Button { oneshot }) = x.settings {
                Some(oneshot)
            } else {
                None
            }
        })
        .unwrap_or(false);

    if let Some(Bind::Submit {
        signal: Some(mut s),
        ..
    }) = layout.bind
    {
        let v = s.read().as_bool().unwrap();
        let mut css = vec!["button", "shadow"];
        css.push(if !v { "accent" } else { "disabled" });
        rsx! {
            button {
                class: css.join(" "),
                onclick: move |_event| {
                    if oneshot {
                        if !v {
                            s.set(Value::Bool(true));
                        }
                    } else {
                        s.set(Value::Bool(!v));
                        spawn(async move {
                            s.set(Value::Bool(v));
                        });
                    }
                },
                {t}
            }
        }
    } else {
        rsx! {}
    }
}
