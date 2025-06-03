use super::super::data::Layout;
use super::super::store::Store;
use super::utils::merge_css_class;
use super::{Dynamic, Frame};
use dioxus::prelude::*;

#[component]
pub fn Diagram(id: String, layout: Layout) -> Element {
    let eid = id.clone();
    if let Some(val) = layout.value {
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
