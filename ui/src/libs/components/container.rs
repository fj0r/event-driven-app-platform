use crate::libs::components::Frame;
use crate::libs::hooks::use_common_css;
use crate::libs::store::Status;
use dioxus::prelude::*;
use layout::{Bind, BindVariant, Container, Layout, Settings};

#[component]
pub fn Case(id: Option<String>, layout: Layout, children: Element) -> Element {
    let mut css = vec!["case", "f"];
    if let Some(id) = &id {
        css.push(id);
    }
    let mut style = String::new();
    if let Some(a) = &layout.attrs {
        let mut f = true;
        if let Some(Settings::Container(c)) = &a.settings {
            match &c {
                Container::grid(g) => {
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
pub fn Placeholder(id: String, layout: Layout, children: Element) -> Element {
    let mut css = vec!["placeholder", "f"];
    use_common_css(&mut css, &layout);
    let store = use_context::<Status>();
    let s = store.data.read();
    if let Some(x) = layout.bind.as_ref()
        && let Some(Bind {
            variant: BindVariant::Source { source },
            default: _,
            r#type: _kind,
        }) = x.get("value")
        && let Some(data) = s.get(source)
        && data.kind != "empty"
    {
        let eid = id.clone();
        dioxus::logger::tracing::info!("{:?}", data);
        use_effect(move || {
            let js = format!(
                r#"
                let x = document.getElementById('{eid}');
                x.classList.add('fade-in-and-out');
                setTimeout(() => x.classList.remove('fade-in-and-out'), 1000);
                "#
            );
            document::eval(&js);
        });
        rsx! {
            div {
                id: id,
                class: css.join(" "),
                Frame { component: data.clone() }
            }
        }
    } else {
        rsx! {
            div {
                id: id,
                class: css.join(" "),
                {children}
            }
        }
    }
}
