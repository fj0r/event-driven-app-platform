use super::super::store::Store;
use super::utils::merge_css_class;
use super::{Dynamic, Frame};
use dioxus::prelude::*;
use layout::Layout;

#[component]
pub fn Diagram(id: String, layout: Layout) -> Element {
    let eid = id.clone();
    if let Some(x) = layout
        .bind
        .and_then(|x| x.get("value").cloned())
        .and_then(|x| x.default)
        && let Some(y) = x.as_str()
    {
        let val = y.to_string();
        use_effect(move || {
            let js = format!(
                r#"
                mermaid.init({{}}, '#{eid}')
            "#
            );
            document::eval(&js);
        });
        rsx! {
            div {
                id: id,
                {val}
            }
        }
    } else {
        rsx! {
            div {}
        }
    }
}
