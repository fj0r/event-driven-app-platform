use crate::libs::hooks::use_default;
use brick::{Image, ImageAttr};
use dioxus::prelude::*;

#[component]
pub fn image_(id: Option<String>, brick: Image, children: Element) -> Element {
    if let Some(src) = use_default(&brick)
        && let Some(src) = src.as_str()
        && let Some(x) = brick.attrs
    {
        let ImageAttr {
            class,
            desc,
            thumb,
            width,
            height,
        } = &x;
        let style = x.size_style();
        let desc = desc.as_ref().unwrap_or(&"".to_string()).clone();
        return rsx! {
            img {
                src: src,
                alt: desc,
                style: style
            }
        };
    };
    rsx!()
}
