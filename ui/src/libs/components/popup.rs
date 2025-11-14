use crate::libs::components::Frame;
use crate::libs::hooks::use_common_css;
use brick::Popup;
use dioxus::prelude::*;

#[component]
pub fn popup_(id: Option<String>, brick: Popup, children: Element) -> Element {
    let mut css = vec!["popup", "f"];
    use_common_css(&mut css, &brick);

    let style = brick
        .attrs
        .as_ref()
        .and_then(|x| x.direction.as_ref())
        .map(|x| x.into_flex());

    if let Some(children) = &brick.children
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
                        brick: placeholder.clone()
                    }
                }
                div {
                    class: "f body",
                    Frame {
                        brick: modal.clone()
                    }
                }
            }
        }
    } else {
        rsx!()
    }
}
