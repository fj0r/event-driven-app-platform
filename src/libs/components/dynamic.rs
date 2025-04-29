use super::super::data::Layout;
use super::container::*;
use super::list::List;
use super::text::Text;
use super::widgets::*;
use super::x::*;
use dioxus::prelude::*;

#[component]
pub fn Dynamic(layout: Layout, children: Element) -> Element {
    let c = {
        match layout.kind.as_str() {
            "container" => rsx!( Container { layout: layout, {children} } ),
            "list" => rsx!( List { layout: layout, {children} } ),
            "card" => rsx! ( Card { layout: layout, {children } } ),
            "input" => rsx!(Input { layout: layout }),
            "text" => rsx!(Text { layout: layout }),
            "button" => rsx!(Button { layout: layout }),
            "x" => rsx!(X { layout: layout }),
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
