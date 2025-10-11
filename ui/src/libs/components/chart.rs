use super::utils::use_default;
use dioxus::prelude::*;
use layout::Layout;

#[component]
pub fn Chart(id: String, layout: Layout) -> Element {
    let eid = id.clone();
    if let Some(val) = use_default(&layout) {
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
