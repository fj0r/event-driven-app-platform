use super::Dynamic;
use brick::Brick;
use dioxus::prelude::*;

#[component]
pub fn Frame(brick: Brick) -> Element {
    let children = brick.get_children();
    if let Some(children) = children {
        let children = children.iter().map(|c| {
            rsx! {
                Frame { brick: c.clone() }
            }
        });

        rsx! {
            Dynamic {
                brick: brick,
                {children}
            }
        }
    } else {
        rsx! {
            Dynamic { brick }
        }
    }
}
