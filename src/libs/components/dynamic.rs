use super::container::*;
use super::widgets::*;
use dioxus::prelude::*;
use super::super::data::Layout;

#[component]
pub fn Dynamic(layout: Layout, children: Element) -> Element {
    let c = {
        match layout.kind.as_str() {
            "Container" => rsx!( Container { layout: layout, {children} } ),
            "List" => rsx!( List { layout: layout, {children} } ),
            "Input" => rsx! ( Input { layout: layout, {children} } ),
            "Text" => rsx! ( Text { layout: layout, {children} } ),
            "Card" => rsx! ( Card { layout: layout, {children } } ),
            "Button" => rsx! ( Button { layout: layout, {children} } ),
            _ => {
                let t = format!("{} unimplemented!", layout.kind);
                rsx! { div { "{t}" } }
            }
        }
    };
    rsx! {
        {c}
    }
}
