use super::super::data::Layout;
use super::chart::Chart;
use super::form::Form;
use super::rack::Rack;
use super::widgets::*;
use dioxus::prelude::*;

use std::sync::{LazyLock, Mutex};
static LIST_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[component]
pub fn Dynamic(layout: Layout, children: Element) -> Element {
    let c = {
        match layout.kind.as_str() {
            "case" => rsx!(Case { layout: layout, {children} }),
            "rack" => {
                let mut tc = LIST_ID.lock().unwrap();
                *tc += 1;
                let id = format!("list-{}", *tc);
                rsx!(Rack { id: id, layout: layout, {children} })
            }
            "form" => rsx!(Form { layout: layout }),
            "chart" => rsx!(Chart { layout: layout }),
            "input" => rsx!(Input { layout: layout }),
            "text" => rsx!(Text { layout: layout }),
            "button" => rsx!(Button { layout: layout }),
            "image" => rsx!(Img { layout: layout }),
            "table" => rsx! (TABLE { layout: layout, {children} }),
            "thead" => rsx! (Thead { layout: layout, {children} }),
            "tbody" => rsx! (Tbody { layout: layout, {children} }),
            "tr" => rsx! (Tr { layout: layout, {children} }),
            "th" => rsx! (Th { layout: layout, {children} }),
            "td" => rsx! (Td { layout: layout, {children} }),
            "x" => rsx!(X { layout: layout }),
            _ => {
                let t = format!("{} unimplemented!\n{:?}", layout.kind, layout);
                rsx! { div { "{t}" } }
            }
        }
    };
    rsx! {
        {c}
    }
}
