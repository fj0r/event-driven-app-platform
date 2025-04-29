use super::super::store::Store;
use super::utils::merge_css_class;
use super::{super::data::Layout, utils::get_attrs};
use comrak::{markdown_to_html, Options};
use dioxus::prelude::*;
use std::sync::{LazyLock, Mutex};

#[component]
pub fn Text(layout: ReadOnlySignal<Layout>) -> Element {
    /*
    static TEXT_COUNT: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
    let mut tc = TEXT_COUNT.lock().unwrap();
    *tc += 1;
    let id = format!("text-{}", *tc);
    */

    let mut css = vec!["text", "f", "txt"];

    let layout_cloned = layout();
    let css = merge_css_class(&mut css, &layout_cloned);

    let s = use_context::<Store>();

    let mut t = {
        let value = layout.read().value.clone();
        Layout {
            kind: "Text".to_string(),
            value,
            ..Layout::default()
        }
    };
    if let Some(b) = &layout.read().data {
        if !b.upload {
            let x = s.data.read().get(&b.event).cloned();
            if let Some(t1) = x {
                t = t1
            }
        }
    };
    let v = if let Some(j) = t.value {
        if j.is_string() {
            j.as_str().unwrap().to_owned()
        } else {
            j.to_string()
        }
    } else {
        "".to_string()
    };

    static MDFMT: LazyLock<Vec<&str>> = LazyLock::new(|| vec!["markdown", "md"]);

    if let Some(a) = get_attrs(layout.read().clone(), "format") {
        if a.is_string() && (*MDFMT).contains(&a.as_str().unwrap()) {
            let v = v.clone();
            let md = markdown_to_html(&v, &Options::default());
            return rsx! {
                div {
                    class: css.join(" "),
                    dangerous_inner_html: md
                }
            };
        }
    };

    rsx! {
        div {
            class: css.join(" "),
            {v}
        }
    }
}
