use super::Dynamic;
use brick::{Brick, BrickProps};
use dioxus::prelude::*;

#[component]
pub fn Frame(brick: Brick) -> Element {
    let children = brick.borrow_children();
    if let Some(children) = children {
        let children = children.iter().map(|c| {
            rsx! {
                Frame { brick: c.clone() }
            }
        });

        rsx! {
            Dynamic {
                brick: brick.clone(),
                {children}
            }
        }
    } else {
        rsx! {
            Dynamic { brick }
        }
    }
}
