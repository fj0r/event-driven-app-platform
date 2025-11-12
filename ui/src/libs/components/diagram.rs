use crate::libs::hooks::{use_common_css, use_default};
use component::Diagram;
use dioxus::prelude::*;

#[component]
pub fn diagram_(id: String, component: Diagram) -> Element {
    let eid = id.clone();
    let mut css = vec!["diagram"];
    use_common_css(&mut css, &component);
    if let Some(x) = use_default(&component)
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
                class: css.join(" "),
                {val}
            }
        }
    } else {
        rsx! {
            div {}
        }
    }
}
