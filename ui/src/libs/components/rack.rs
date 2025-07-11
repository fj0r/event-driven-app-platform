use super::super::store::Store;
use super::utils::merge_css_class;
use super::{Dynamic, Frame};
use dioxus::{CapturedError, prelude::*};
use layout::{Bind, Layout, Settings};
use std::collections::hash_map::HashMap;
use std::str::FromStr;

struct ItemContainer {
    default: Option<Layout>,
    index: HashMap<String, Layout>,
}

impl From<Vec<Layout>> for ItemContainer {
    fn from(data: Vec<Layout>) -> Self {
        let mut default = None;
        let mut index = HashMap::new();
        for l in &data {
            if let Some(x) = &l.attrs
                && let Some(Settings::Item { selector }) = &x.settings
            {
                index.insert(selector.to_owned(), l.clone());
            } else {
                default = Some(l.clone());
            };
        }
        ItemContainer { index, default }
    }
}

impl ItemContainer {
    fn select(&self, child: &Layout) -> Option<Layout> {
        if let Some(x) = &child.attrs
            && let Some(kind) = &x.kind
            && let Some(i) = self.index.get(kind)
        {
            return Some(i).cloned();
        }
        self.default.clone()
    }
}

#[component]
pub fn Rack(id: String, layout: Layout, children: Element) -> Element {
    let mut css = vec!["rack", "f"];
    let css = merge_css_class(&mut css, &layout);

    let item: ItemContainer = layout.item.clone().context("item")?.into();
    let Bind::Event { event, .. } = layout.bind.as_ref().context("data")? else {
        return Err(RenderError::Aborted(CapturedError::from_str("no event")?));
    };
    let attrs = layout.attrs.as_ref().context("attrs")?;

    let store = use_context::<Store>();
    let c = store.list.read();
    let c = c.get(event).cloned().unwrap_or_else(Vec::new);
    let r = c.iter().enumerate().map(|(idx, child)| {
        let key = child.id.clone().unwrap_or(idx.to_string());
        let layout = item.select(child);
        if let Some(layout) = layout {
            let x = rsx! {
                Frame {
                    layout: child.clone()
                }
            };
            if c.len() - 1 == idx {
                // last element
                rsx! {
                    Dynamic {
                        key: "{key}",
                        layout: layout,
                        {x}
                    }
                }
            } else {
                rsx! {
                    Dynamic {
                        key: "{key}",
                        layout: layout,
                        {x}
                    }
                }
            }
        } else {
            rsx! {
                Frame {
                    key: "{key}",
                    layout: child.clone()
                }
            }
        }
    });

    if let Some(Settings::Rack { scroll: x, .. }) = attrs.settings
        && x
    {
        let sl = store.list;
        let eid = id.clone();
        use_effect(move || {
            // TODO: fine-grained
            let _ = sl.read();
            document::eval(&format!(
                r#"
                var e = document.getElementById("{eid}");
                if (Math.abs(e.scrollHeight - e.offsetHeight - e.scrollTop) < e.offsetHeight) {{
                    e.scrollTop = e.scrollHeight;
                }}
            "#
            ));
        });
    };

    rsx! {
        div {
            id: id,
            class: css.join(" "),
            {r}
        }
    }
}
