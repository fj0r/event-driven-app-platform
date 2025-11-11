use crate::libs::components::Frame;
use crate::libs::hooks::use_common_css;
use crate::libs::store::Status;
use component::{Bind, BindVariant, Case, CaseAttr, JsonComponent, Placeholder, ClassAttr};
use dioxus::prelude::*;

#[component]
pub fn case(id: Option<String>, component: Case, children: Element) -> Element {
    let mut css = vec!["case", "f"];
    if let Some(id) = &id {
        css.push(id);
    }
    let mut style = String::new();
    if let Case {
        id,
        attrs,
        bind,
        render,
        children,
    } = &component
    {
        let mut f = true;
        if let Some(CaseAttr {
            class,
            horizontal,
            grid,
        }) = attrs
        {
            if let Some(g) = grid {
                f = false;
                css.push("g");
                style = g
                    .iter()
                    .map(|(k, v)| format!("{}: {};", k, v.as_str().unwrap()))
                    .collect::<Vec<String>>()
                    .join("\n");
            };
            if f {
                css.push("f");
            };
        }
    };
    use_common_css(&mut css, &component);

    rsx! {
        div {
            class: css.join(" "),
            style: style,
            {children}
        }
    }
}

#[component]
pub fn placeholder(id: String, component: Placeholder, children: Element) -> Element {
    let mut css = vec!["placeholder", "f"];
    use_common_css(&mut css, &component);
    let store = use_context::<Status>();
    let s = store.data.read();

    if let Some(x) = component.bind.as_ref()
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
