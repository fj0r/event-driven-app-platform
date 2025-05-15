use super::super::super::data::{Container as Ct, Table, Layout, Settings};
use super::super::utils::merge_css_class;
use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn Container(layout: Layout, children: Element) -> Element {
    let mut css = vec!["container"];
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
                        .join("\n");
                }
                Ct::table(Table{column, header}) => {
                    // TODO: table
                    // let mut headers = vec![];
                    // let mut rows = vec![];
                    let mut idx = 0;
                    let step = children.iter().len() / column;
                    while idx < step {
                        #[allow(unused_variables)]
                        let s = idx * column;
                        if *header {
                            // headers = children[s..s+column]
                        } else {

                        }
                        idx += 1;
                    }
                    f = false;
                }
            }
        };
        if f {
            css.push("f");
        }
    };
    let css = merge_css_class(&mut css, &layout);

    rsx! {
        div {
            class: css.join(" "),
            style: style,
            {children}
        }
    }
}

#[component]
pub fn Tab(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Menu(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Card(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}
