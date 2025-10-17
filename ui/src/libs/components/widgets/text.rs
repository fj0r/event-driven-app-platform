use crate::libs::hooks::merge_css_class;
use crate::libs::hooks::use_source_value;
use dioxus::prelude::*;
use layout::{Bind, BindVariant, Layout, Settings};
use markdown::{Options, to_html_with_options};
use std::sync::LazyLock;

#[component]
pub fn Text(id: String, layout: Layout) -> Element {
    let mut css = vec!["text", "txt", &id];

    let css = merge_css_class(&mut css, &layout);

    let text_content = if let Some(json_data) = use_source_value(&layout) {
        if json_data.is_string() {
            json_data.as_str().unwrap().to_owned()
        } else {
            json_data.to_string()
        }
    } else {
        "".to_string()
    };

    static MDFMT: LazyLock<Vec<String>> = LazyLock::new(|| {
        ["markdown", "md"]
            .iter()
            .map(|fmt| fmt.to_string())
            .collect()
    });

    if let Some(attrs) = layout.clone().attrs
        && let Some(Settings::Text {
            format: text_format,
        }) = attrs.settings
        && (*MDFMT).contains(&text_format)
    {
        let v = &text_content;
        if let Ok(md_html) = to_html_with_options(v, &Options::gfm()) {
            css.push("markdown");
            return rsx! {
                div {
                    class: css.join(" "),
                    dangerous_inner_html: md_html
                }
            };
        }
    };

    rsx! {
        div {
            class: css.join(" "),
            {text_content}
        }
    }
}

