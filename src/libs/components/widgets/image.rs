
use dioxus::prelude::*;
use super::super::super::data::{Layout, Settings};

#[component]
pub fn Img(layout: Layout, children: Element) -> Element {
    if let Some(src) = &layout.value {
        if let Some (src) = src.as_str() {
            let alt = layout.attrs.and_then(|x| {
                if let Some(Settings::Image { alt }) = x.settings {
                    Some(alt)
                } else {
                    None
                }

            }).unwrap_or("".to_string());
            return rsx! {
                img {
                    src: src,
                    alt: alt
                }
            }
        }
    };
    rsx!()
}
