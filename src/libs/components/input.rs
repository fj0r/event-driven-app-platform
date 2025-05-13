use super::super::data::{Bind, Layout};
use super::super::store::Store;
use super::utils::merge_css_class;
use dioxus::prelude::*;
use serde_json::{to_value, Value};

#[component]
pub fn Input(layout: Layout) -> Element {
    let mut x = use_signal(|| "".to_string());
    let s = use_context::<Store>();
    let mut css = vec!["input", "f", "shadow"];
    let css = merge_css_class(&mut css, &layout);

    let ev = layout.data.clone();

    rsx! {
        input {
            class: css.join(" "),
            value: x,
            oninput: move |event| {
                let ev = layout.data.clone();
                x.set(event.value());
                if let Some(Bind::Signal { mut signal }) = ev {
                    signal.set(to_value(event.value()).unwrap());
                };
            },
            onkeydown: move |event| {
                let mut s = s.clone();
                let ev = ev.clone();
                async move {
                    if event.data.key() == Key::Enter {
                        let v = to_value(x.read().to_string()).unwrap();
                        match ev {
                            Some(Bind::Event { event: e, .. }) => {
                                s.send(e, None, v).await;
                                *x.write() = "".to_string()
                            },
                            Some(Bind::Signal { mut signal }) => {
                                signal.set(v);
                                x.set("".to_string())
                            },
                            _ => unreachable!()
                        }
                    }
                }
            }
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
