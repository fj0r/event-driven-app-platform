use super::chart::Chart;
use super::container::*;
use super::diagram::Diagram;
use super::float::Float;
use super::fold::Fold;
use super::form::Form;
use super::popup::Popup;
use super::rack::Rack;
use super::svg::*;
use super::widgets::*;
use dioxus::prelude::*;
use layout::Layout;

use std::sync::{LazyLock, Mutex};
static COMPONENT_ID: LazyLock<Mutex<u64>> = LazyLock::new(|| Mutex::new(0));
static DIAGRAM_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static CHART_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static RACK_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[component]
pub fn Dynamic(layout: Layout, children: Element) -> Element {
    let mut tc = COMPONENT_ID.lock().unwrap();
    *tc += 1;
    let id = format!("={}=", *tc);

    let c = {
        match layout.kind.as_str() {
            "case" => {
                rsx!(Case { id: id, layout: layout, {children} })
            }
            "fold" => rsx!(Fold { id: id, layout: layout, {children} }),
            "placeholder" => rsx!(Placeholder { layout: layout, {children} }),
            "rack" => {
                let mut tc = RACK_ID.lock().unwrap();
                *tc += 1;
                let id = format!("rack-{}", *tc);
                rsx!(Rack { id: id, layout: layout, {children} })
            }
            "form" => rsx!(Form { layout: layout }),
            "chart" => {
                let mut tc = CHART_ID.lock().unwrap();
                *tc += 1;
                let id = format!("chart-{}", *tc);
                rsx!(Chart {
                    id: id,
                    layout: layout
                })
            }
            "diagram" => {
                let mut tc = DIAGRAM_ID.lock().unwrap();
                *tc += 1;
                let id = format!("diagram-{}", *tc);
                rsx!(Diagram {
                    id: id,
                    layout: layout
                })
            }
            "input" => rsx!(Input { layout: layout }),
            "select" => rsx!(Select { layout: layout, {children} }),
            "popup" => rsx!(Popup { layout: layout, {children} }),
            "float" => rsx!(Float { layout: layout, {children} }),
            "text" => {
                rsx!(Text {
                    id: id,
                    layout: layout
                })
            }
            "textarea" => {
                rsx!(TextArea {
                    id: id,
                    layout: layout
                })
            }
            "button" => rsx!(Button { layout: layout }),
            "image" => rsx!(Img { layout: layout }),
            "svg" => rsx! (Svg { layout: layout, {children} }),
            "group" => rsx! (Group { layout: layout, {children} }),
            "path" => rsx!(Path { layout: layout }),
            "table" => rsx! (TABLE { layout: layout, {children} }),
            "thead" => rsx! (Thead { layout: layout, {children} }),
            "tbody" => rsx! (Tbody { layout: layout, {children} }),
            "tr" => rsx! (Tr { layout: layout, {children} }),
            "th" => rsx! (Th { layout: layout, {children} }),
            "td" => rsx! (Td { layout: layout, {children} }),
            "x" => rsx!(X { layout: layout }),
            "empty" => rsx!(),
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
