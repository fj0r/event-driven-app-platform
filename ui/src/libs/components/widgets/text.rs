use super::super::super::store::Store;
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use layout::{Bind, Layout, Settings};
use markdown::{Options, to_html_with_options};
use std::sync::LazyLock;

#[component]
pub fn Text(layout: ReadOnlySignal<Layout>) -> Element {
    let mut css = vec!["text", "txt"];

    let layout_cloned = layout();
    let css = merge_css_class(&mut css, &layout_cloned);

    let store = use_context::<Store>();

    let mut txt_layout = {
        let value = layout.read().value.clone();
        Layout {
            kind: "Text".to_string(),
            value,
            ..Layout::default()
        }
    };
    if let Some(Bind::Source { source, .. }) = layout.read().bind.get("value") {
        let event_data = store.data.read().get(source).cloned();
        if let Some(event_layout) = event_data {
            txt_layout = event_layout
        }
    };
    let text_content = if let Some(json_data) = txt_layout.value {
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

    if let Some(attrs) = layout.read().clone().attrs
        && let Some(Settings::Text {
            format: text_format,
        }) = attrs.settings
        && (*MDFMT).contains(&text_format)
    {
        let v = text_content.clone();
        if let Ok(md_html) = to_html_with_options(&v, &Options::gfm()) {
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
