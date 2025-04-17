use super::super::data::Layout;
use super::container::*;
use super::widgets::*;
use dioxus::prelude::*;

#[component]
pub fn Dynamic(layout: Layout, children: Element) -> Element {
    let c = {
        match layout.kind.as_str() {
            "Container" => rsx!( Container { layout: layout, {children} } ),
            "List" => rsx!( List { layout: layout, {children} } ),
            "Card" => rsx! ( Card { layout: layout, {children } } ),
            "Input" => rsx!(Input { layout: layout }),
            "Text" => rsx!(Text { layout: layout }),
            "Button" => rsx!(Button { layout: layout }),
            "Test" => rsx!(Test { layout: layout }),
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
