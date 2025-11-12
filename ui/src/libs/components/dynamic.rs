use super::chart::chart_;
use super::container::*;
use super::diagram::diagram_;
use super::float::float_;
use super::fold::fold_;
use super::form::form_;
use super::popup::popup_;
use super::rack::rack_;
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
        match component {
            JsonComponent::case(c) => {
                rsx!(case_ { id: id, component: c, {children} })
            }
            JsonComponent::fold(c) => rsx!(fold_ { id: id, layout: c, {children} }),
            JsonComponent::placeholder(c) => {
                let mut tc = PLACEHOLDER_ID.lock().unwrap();
                *tc += 1;
                let id = format!("placeholder-{}", *tc);
                rsx!(placeholder_ {id, component: c, {children} })
            }
            JsonComponent::rack(c) => {
                let mut tc = RACK_ID.lock().unwrap();
                *tc += 1;
                let id = format!("rack-{}", *tc);
                rsx!(rack_ { id: id, layout: c, {children} })
            }
            JsonComponent::form(c) => rsx!(form_ { layout: c }),
            JsonComponent::chart(c) => {
                let mut tc = CHART_ID.lock().unwrap();
                *tc += 1;
                let id = format!("chart-{}", *tc);
                rsx!(chart_ {
                    id: id,
                    component: c
                })
            }
            JsonComponent::diagram(c) => {
                let mut tc = DIAGRAM_ID.lock().unwrap();
                *tc += 1;
                let id = format!("diagram-{}", *tc);
                rsx!(diagram_ {
                    id: id,
                    component: c
                })
            }
            JsonComponent::input(c) => rsx!(Input { layout: c }),
            JsonComponent::select(c) => rsx!(Select { layout: c, {children} }),
            JsonComponent::popup(c) => rsx!(popup_ { layout: c, {children} }),
            JsonComponent::float(c) => rsx!(float_ { layout: c, {children} }),
            JsonComponent::text(c) => {
                rsx!(Text { id: id, layout: c })
            }
            JsonComponent::textarea(c) => {
                rsx!(TextArea { id: id, layout: c })
            }
            JsonComponent::button(c) => rsx!(Button { layout: c }),
            JsonComponent::image(c) => rsx!(Img { layout: c }),
            JsonComponent::svg(c) => rsx! (Svg { layout: c, {children} }),
            JsonComponent::group(c) => rsx! (Group { layout: c, {children} }),
            JsonComponent::path(c) => rsx!(Path { layout: c }),
            JsonComponent::table(c) => rsx! (TABLE { layout: c, {children} }),
            JsonComponent::thead(c) => rsx! (Thead { layout: c, {children} }),
            JsonComponent::tbody(c) => rsx! (Tbody { layout: c, {children} }),
            JsonComponent::tr(c) => rsx! (Tr { layout: c, {children} }),
            JsonComponent::th(c) => rsx! (Th { layout: c, {children} }),
            JsonComponent::td(c) => rsx! (Td { layout: c, {children} }),
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
