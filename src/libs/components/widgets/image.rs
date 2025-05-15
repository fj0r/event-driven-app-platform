
use dioxus::prelude::*;
use super::super::super::data::{Layout, Settings};

#[component]
pub fn Img(layout: Layout, children: Element) -> Element {
    if let Some(src) = &layout.value {
        if let Some (src) = src.as_str() {
            let s = layout.attrs.and_then(|x| x.settings);
            if let Some(Settings::Image { desc, width, height, thumb: _ }) = s {
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
                }
            } else {
                return rsx! {
                    img {
                        src: src,
                    }
                }
            }
        }
    };
    rsx!()
}
