use dioxus::prelude::*;
use layout::Layout;

#[component]
pub fn Chart(id: String, layout: Layout) -> Element {
    let eid = id.clone();
    if let Some(val) = layout
        .bind
        .as_ref()
        .and_then(|x| x.get("value"))
        .and_then(|x| x.default.clone())
    {
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
