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
use component::{ComponentProps, JsonComponent};
use dioxus::prelude::*;

use std::sync::{LazyLock, Mutex};
static COMPONENT_ID: LazyLock<Mutex<u64>> = LazyLock::new(|| Mutex::new(0));
static DIAGRAM_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static CHART_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static RACK_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static PLACEHOLDER_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[component]
pub fn Dynamic(component: JsonComponent, children: Element) -> Element {
    let id = if cfg!(debug_assertions) {
        let mut tc = COMPONENT_ID.lock().unwrap();
        *tc += 1;
        Some(format!("={}=", *tc))
    } else {
        None
    };

    let c = {
        match component.get_type() {
            "case" => {
                rsx!(Case { id: id, layout: component, {children} })
            }
            "fold" => rsx!(Fold { id: id, layout: component, {children} }),
            "placeholder" => {
                let mut tc = PLACEHOLDER_ID.lock().unwrap();
                *tc += 1;
                let id = format!("placeholder-{}", *tc);
                rsx!(Placeholder {id, layout: component, {children} })
            }
            "rack" => {
                let mut tc = RACK_ID.lock().unwrap();
                *tc += 1;
                let id = format!("rack-{}", *tc);
                rsx!(Rack { id: id, layout: component, {children} })
            }
            "form" => rsx!(Form { layout: component }),
            "chart" => {
                let mut tc = CHART_ID.lock().unwrap();
                *tc += 1;
                let id = format!("chart-{}", *tc);
                rsx!(Chart {
                    id: id,
                    layout: component
                })
            }
            "diagram" => {
                let mut tc = DIAGRAM_ID.lock().unwrap();
                *tc += 1;
                let id = format!("diagram-{}", *tc);
                rsx!(Diagram {
                    id: id,
                    layout: component
                })
            }
            "input" => rsx!(Input { layout: component }),
            "select" => rsx!(Select { layout: component, {children} }),
            "popup" => rsx!(Popup { layout: component, {children} }),
            "float" => rsx!(Float { layout: component, {children} }),
            "text" => {
                rsx!(Text {
                    id: id,
                    layout: component
                })
            }
            "textarea" => {
                rsx!(TextArea {
                    id: id,
                    layout: component
                })
            }
            "button" => rsx!(Button { layout: component }),
            "image" => rsx!(Img { layout: component }),
            "svg" => rsx! (Svg { layout: component, {children} }),
            "group" => rsx! (Group { layout: component, {children} }),
            "path" => rsx!(Path { layout: component }),
            "table" => rsx! (TABLE { layout: component, {children} }),
            "thead" => rsx! (Thead { layout: component, {children} }),
            "tbody" => rsx! (Tbody { layout: component, {children} }),
            "tr" => rsx! (Tr { layout: component, {children} }),
            "th" => rsx! (Th { layout: component, {children} }),
            "td" => rsx! (Td { layout: component, {children} }),
            "x" => rsx!(X { layout: component }),
            "empty" => rsx!(),
            _ => {
                let t = format!("{} unimplemented!\n{:?}", component.get_type(), component);
                rsx! { div { "{t}" } }
            }
        }
    };
    rsx! {
        {c}
    }
}
