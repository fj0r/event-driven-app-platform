use super::super::data::Layout;
use super::super::store::Store;
use super::utils::merge_css_class;
use dioxus::prelude::*;
use serde_json::to_value;

#[component]
pub fn Input(layout: Layout) -> Element {
    let mut x = use_signal(|| "".to_string());
    let mut s = use_context::<Store>();
    let mut css = vec!["input", "f", "shadow"];
    let css = merge_css_class(&mut css, &layout);

    let ev = layout.clone().data.map(|x| x.event);

    rsx! {
        input {
            class: css.join(" "),
            value: x,
            oninput: move |event| {
                x.set(event.value())
            },
            onkeydown: move |event| {
                let mut s = s.clone();
                let ev = ev.clone();
                async move {
                    if event.data.key() == Key::Enter {
                        if let Some(e) = ev {
                            s.send(e, None, "text".to_owned(), to_value(x.read().to_string()).unwrap()).await;
                            // x.set("".to_string())
                            *x.write() = "".to_string()
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
