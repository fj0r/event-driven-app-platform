use crate::libs::hooks::use_default;
use component::{Chart, ClassAttr};
use dioxus::prelude::*;

#[component]
pub fn chart_(id: String, component: Chart) -> Element {
    let eid = id.clone();
    if let Some(val) = use_default(&component) {
        use_effect(move || {
            let js = format!(
                r#"
                var chart = new ApexCharts(document.getElementById("{eid}"), {val});
                chart.render();
            "#
            );
            document::eval(&js);
        });
        rsx! {
            div {
                id: id,
            }
        }
    } else {
        rsx! {
            div {}
        }
    }
}
