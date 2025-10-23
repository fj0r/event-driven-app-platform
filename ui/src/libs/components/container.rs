use crate::libs::hooks::use_common_css;
use dioxus::prelude::*;
use layout::{Container as Ct, Layout, Settings};

#[component]
pub fn Case(id: String, layout: Layout, children: Element) -> Element {
    let mut css = vec!["case", &id];
    let mut style = String::new();
    if let Some(a) = &layout.attrs {
        let mut f = true;
        if let Some(Settings::Container(c)) = &a.settings {
            match &c {
                Ct::grid(g) => {
                    f = false;
                    css.push("g");
                    style = g
                        .iter()
                        .map(|(k, v)| format!("{}: {};", k, v.as_str().unwrap()))
                        .collect::<Vec<String>>()
                        .join("\n");
                }
            }
        };
        if f {
            css.push("f");
        }
    };
    use_common_css(&mut css, &layout);

    rsx! {
        div {
            class: css.join(" "),
            style: style,
            {children}
        }
    }
}

#[component]
pub fn Switch(layout: Layout, children: Element) -> Element {
    let mut css = vec!["switch", "f"];
    use_common_css(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}
