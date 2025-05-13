use super::super::data::{Bind, Layout, Settings};
use super::super::store::Store;
use super::utils::merge_css_class;
use super::{Dynamic, Frame};
use dioxus::{prelude::*, CapturedError};
use std::collections::hash_map::HashMap;
use std::str::FromStr;

struct ItemContainer {
    index: HashMap<String, Layout>,
    default: Option<Layout>,
}

impl From<Vec<Layout>> for ItemContainer {
    fn from(value: Vec<Layout>) -> Self {
        let mut default = None;
        let mut index = HashMap::new();
        for l in &value {
            if let Some(x) = l.attrs.as_ref().and_then(|x| {
                if let Some(Settings::Item { selector }) = &x.settings {
                    Some(selector)
                } else {
                    None
                }
            }) {
                index.insert(x.to_owned(), l.clone());
            } else {
                default = Some(l.clone());
            };
        }
        ItemContainer { index, default }
    }
}

impl ItemContainer {
    fn select(&self, child: &Layout) -> Layout {
        let default = self.default.clone().unwrap();
        if let Some(s) = child.attrs.as_ref().and_then(|x| x.kind.as_ref()) {
            self.index.get(s).unwrap_or_else(|| &default).clone()
        } else {
            default
        }
    }
}

#[component]
pub fn List(id: String, layout: Layout, children: Element) -> Element {
    let mut css = vec!["list", "f"];
    let css = merge_css_class(&mut css, &layout);

    let item: ItemContainer = layout.item.clone().context("item")?.into();
    let Bind::Event { event, .. } = layout.data.as_ref().context("data")? else {
        return Err(RenderError::Aborted(CapturedError::from_str("no event")?));
    };
    let attrs = layout.attrs.as_ref().context("attrs")?;

    let s = use_context::<Store>();
    let c = s.list.read();
    let c = c.get(event).cloned().unwrap_or_else(|| Vec::new());
    let r = c.iter().enumerate().map(|(idx, child)| {
        let x = rsx! {
            Frame {
                layout: child.clone()
            }
        };
        let key = child.id.clone().unwrap_or(idx.to_string());
        let layout = item.select(child);
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
    });

    if let Some(Settings::List { scroll: x, .. }) = attrs.settings {
        if x {
            let sl = s.list.clone();
            let eid = id.clone();
            use_effect(move || {
                let _ = sl.read();
                document::eval(&format!(
                    r#"
                var e = document.getElementById("{eid}");
                e.scrollTop = e.scrollHeight;
            "#
                ));
            });
        }
    };

    rsx! {
        div {
            id: id,
            class: css.join(" "),
            {r}
        }
    }
}
