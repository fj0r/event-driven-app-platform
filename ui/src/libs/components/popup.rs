use crate::libs::components::Frame;
use crate::libs::hooks::use_common_css;
use dioxus::prelude::*;
use layout::Layout;

#[component]
pub fn popup_(layout: Layout, children: Element) -> Element {
    let mut css = vec!["popup", "f"];
    use_common_css(&mut css, &layout);

    let style = layout
        .attrs
        .as_ref()
        .and_then(|x| x.direction.as_ref())
        .map(|x| x.into_flex());

    if let Some(children) = &layout.children
        && let Some(placeholder) = children.get(0)
        && let Some(modal) = children.get(1)
    {
        rsx! {
            div {
                class: css.join(" "),
                style: style,
                div {
                    class: "f",
                    Frame {
                        component: placeholder.clone()
                    }
                }
                div {
                    class: "f body",
                    Frame {
                        component: modal.clone()
                    }
                }
            }
        }
    } else {
        rsx!()
    }
}
