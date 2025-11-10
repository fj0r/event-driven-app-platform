use crate::libs::store::Status;
use component::{Bind, BindVariant, ComponentProps, JsonComponent};
#[allow(unused_imports)]
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use serde_json::Value;

pub fn use_common_css<'a, 'b: 'a>(css: &'a mut Vec<&'b str>, component: &'b JsonComponent) {
    let mut v = ["box", "case", "rack", "text", "tab", "select"].contains(&component.get_type());
    if let Some(a) = component.get_attrs() {
        if a.is_horizontal() {
            v = false;
        }
        if let Some(cc) = a.get_class() {
            let c = cc.iter().map(|x| &**x).collect::<Vec<_>>();
            css.extend(c);
        }
    }
    if v {
        css.push("col");
    }
}

pub fn use_default<'a>(component: &'a JsonComponent) -> Option<Value> {
    component
        .get_bind()
        .and_then(|x| x.get("value"))
        .and_then(|x| x.default.clone())
}

pub fn use_source_id<'a>(component: &'a JsonComponent) -> Option<&'a String> {
    if let Bind {
        variant: BindVariant::Source { source },
        ..
    } = component.get_bind().and_then(|x| x.get("value"))?
    {
        Some(source)
    } else {
        None
    }
}

pub fn use_source_list<'a>(
    component: &'a JsonComponent,
    key: &'a str,
) -> Option<Vec<JsonComponent>> {
    let store = use_context::<Status>();
    let s = store.list.read();
    if let Some(x) = component.get_bind()
        && let Some(Bind {
            variant: BindVariant::Source { source },
            default: _,
            r#type: _kind,
        }) = x.get(key)
        && let list = s.get(source)
        && list.is_some()
    {
        list.cloned()
    } else {
        Some(Vec::new())
    }
}

pub fn use_source<'a>(component: &'a JsonComponent, key: &'a str) -> Option<Value> {
    let store = use_context::<Status>();
    let s = store.data.read();
    let value = if let Some(x) = component.get_bind()
        && let Some(Bind {
            variant: BindVariant::Source { source },
            default: _,
            r#type: _kind,
        }) = x.get(key)
        && let data = s.get(source)
        && data.is_some()
    {
        data
    } else {
        Some(component)
    };
    if let Some(comp) = value
        && let Some(bind) = &comp.get_bind()
        && let Some(value) = bind.get(key)
    {
        value.default.clone()
    } else {
        None
    }
}

pub fn use_source_value(component: &JsonComponent) -> Option<Value> {
    use_source(component, "value")
}

pub fn use_target<'a>(component: &'a JsonComponent, key: &'a str) -> Option<impl Fn(Value)> {
    if let Some(x) = component.get_bind()
        && let Some(Bind {
            // TODO: variable
            variant: BindVariant::Event { event },
            default: _,
            r#type: _,
        }) = x.get(key)
    {
        let fun = move |val| {
            let ev = event.clone();
            let mut store = use_context::<Status>();
            spawn(async move {
                store.send(ev, None, val).await;
            });
        };
        Some(fun)
    } else {
        None
    }
}

pub fn use_target_value(component: &JsonComponent) -> Option<impl Fn(Value)> {
    use_target(component, "value")
}
