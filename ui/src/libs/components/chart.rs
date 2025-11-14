use crate::libs::hooks::use_default;
use brick::Chart;
use dioxus::prelude::*;

#[component]
pub fn chart_(id: Option<String>, brick: Chart) -> Element {
    let eid = id.clone();
    if let Some(val) = use_default(&brick) {
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
