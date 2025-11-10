use super::Dynamic;
use component::{ComponentProps, JsonComponent};
use dioxus::prelude::*;

#[component]
pub fn Frame(component: JsonComponent) -> Element {
    let children = component.get_children();
    if let Some(children) = children {
        let children = children.iter().map(|c| {
            rsx! {
                Frame { component: c.clone() }
            }
        });

        rsx! {
            Dynamic {
                component: component,
                {children}
            }
        }
    } else {
        rsx! {
            Dynamic { component }
        }
    }
}
