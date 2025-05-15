use super::super::data::Layout;
use super::chart::Chart;
use super::widgets::*;
use super::form::Form;
use super::list::List;
use dioxus::prelude::*;

use std::sync::{LazyLock, Mutex};
static LIST_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[component]
pub fn Dynamic(layout: Layout, children: Element) -> Element {
    let c = {
        match layout.kind.as_str() {
            "container" => rsx!(Container { layout: layout, {children} }),
            "list" => {
                let mut tc = LIST_ID.lock().unwrap();
                *tc += 1;
                let id = format!("list-{}", *tc);
                rsx!(List { id: id, layout: layout, {children} })
            }
            "form" => rsx!(Form { layout: layout }),
            "chart" => rsx!(Chart { layout: layout }),
            "card" => rsx! (Card { layout: layout, {children} }),
            "input" => rsx!(Input { layout: layout }),
            "text" => rsx!(Text { layout: layout }),
            "button" => rsx!(Button { layout: layout }),
            "table" => rsx! (TABLE { layout: layout, {children} }),
            "thead" => rsx! (Thead { layout: layout, {children} }),
            "tbody" => rsx! (Tbody { layout: layout, {children} }),
            "tr" => rsx! (Tr { layout: layout, {children} }),
            "th" => rsx! (Th { layout: layout, {children} }),
            "td" => rsx! (Td { layout: layout, {children} }),
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
