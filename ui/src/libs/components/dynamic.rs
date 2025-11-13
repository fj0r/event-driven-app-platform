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
use ui_macro::gen_dispatch;

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
        gen_dispatch! {
            file = "./component/src/lib.rs",
            entry = "JsonComponent"
        }
    };
    rsx! {
        {c}
    }
}
