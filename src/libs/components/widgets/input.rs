use super::super::super::data::{Bind, Layout};
use super::super::super::store::Store;
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use serde_json::to_value;

#[component]
pub fn Input(layout: Layout) -> Element {
    let mut x = use_signal(|| {
        layout
            .value
            .as_ref()
            .and_then(|x| x.as_str())
            .map(|x| x.to_owned())
            .unwrap_or("".to_string())
    });
    let s = use_context::<Store>();
    let mut css = vec!["input", "f", "shadow"];
    let css = merge_css_class(&mut css, &layout);

    let ev = layout.data.clone();

    rsx! {
        input {
            class: css.join(" "),
            value: x,
            oninput: move |event| {
                x.set(event.value());
                let ev = layout.data.clone();
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

