use dioxus::prelude::*;
use layout::{Layout, Settings};

#[component]
pub fn Img(layout: Layout, children: Element) -> Element {
    if let Some(src) = &layout.data
        && let Some(src) = src.as_str()
        && let Some(x) = layout.attrs
    {
        if let Some(Settings::Image {
            desc,
            width,
            height,
            thumb: _,
        }) = x.settings
        {
            let mut style = Vec::new();
            style.push(format!("width: {};", width.unwrap_or("auto".to_string())));
            style.push(format!("height: {};", height.unwrap_or("auto".to_string())));
            let style = style.join("\n");
            return rsx! {
                img {
                    src: src,
                    alt: desc,
                    style: style
                }
            };
        } else {
            return rsx! {
                img {
                    src: src,
                }
            };
        }
    };
    rsx!()
}
