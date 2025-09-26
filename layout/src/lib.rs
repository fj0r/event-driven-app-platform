use dioxus::prelude::*;
use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};
use minijinja::Environment;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json, to_value};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct Attrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    // for selector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal: Option<bool>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Settings {
    Container(Container),
    Rack {
        scroll: bool,
    },
    Fold {
        replace_header: bool,
        float_body: bool,
    },
    Svg {
        svg: Map<String, Value>,
    },
    Text {
        format: String,
    },
    Item {
        selector: String,
    },
    Button {
        oneshot: bool,
    },
    Form {
        instant: bool,
    },
    Image {
        desc: String,
        #[serde(default)]
        thumb: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum Container {
    #[allow(non_camel_case_types)]
    grid(Map<String, Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct Render {
    pub name: String,
    pub data: Value,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum JsKind {
    #[allow(non_camel_case_types)]
    bool,
    #[allow(non_camel_case_types)]
    number,
    #[default]
    #[allow(non_camel_case_types)]
    text,
    #[allow(non_camel_case_types)]
    password,
    #[allow(non_camel_case_types)]
    button,
    #[allow(non_camel_case_types)]
    submit,
}

impl JsKind {
    pub fn input_type(&self) -> &'static str {
        match self {
            Self::bool => "checkbox",
            Self::number => "number",
            Self::text => "text",
            Self::password => "password",
            Self::button => "button",
            Self::submit => "submit",
        }
    }

    pub fn default_value(&self) -> Value {
        match self {
            Self::number => to_value(0),
            Self::bool => to_value(false),
            _ => to_value(""),
        }
        .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum BindClass {
    Source {
        source: String,
    },
    Target {
        target: String,
    },
    Variable {
        vairable: String,
    },
    Field {
        field: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<Value>,
    },
    Submit {
        submit: bool,
        #[allow(dead_code)]
        #[serde(skip)]
        signal: Option<Signal<Value>>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Bind {
    #[serde(flatten)]
    class: BindClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    kind: Option<JsKind>,
}

fn kind_empty() -> String {
    "empty".to_string()
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize, Default, JsonSchema)]
pub struct Layout {
    #[serde(rename = "type", default = "kind_empty")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Attrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<Render>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<Layout>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Layout>>,
}

impl From<Layout> for Value {
    fn from(value: Layout) -> Self {
        serde_json::to_value(value).expect("Layout n not a Value")
    }
}

impl Layout {
    #[allow(dead_code)]
    pub fn new(kind: impl AsRef<str>) -> Self {
        Layout {
            kind: kind.as_ref().to_string(),
            ..Default::default()
        }
    }
    pub fn cmp_id(&self, other: &Self) -> bool {
        let Some(id) = &self.id else {
            return false;
        };
        let Some(oid) = &other.id else {
            return false;
        };
        id == oid
    }

    pub fn render(&mut self, env: &Environment) {
        if let Some(r) = &self.render {
            let n = &r.name;
            let cx = &r.data;
            let n = env
                .get_template(n)
                .map_err(|e| e.to_string())
                .and_then(|t| {
                    t.render(cx)
                        .map_err(|e| format!("render failed: {} => {}", e, &cx))
                })
                .and_then(|t| {
                    serde_json::from_str::<Layout>(&t)
                        .map_err(|e| format!("deserialize failed: {} => {}", e, &t))
                });
            match n {
                Ok(x) => {
                    *self = x;
                }
                Err(x) => {
                    dioxus_logger::tracing::info!("{x:?}");
                }
            }
        }
        if let Some(cs) = &mut self.children {
            for c in cs {
                c.render(env);
            }
        }
    }

    pub fn merge(&mut self, op: &(impl LayoutOp + ?Sized), mut rhs: Self) {
        op.merge(self, &mut rhs);
        if let Some(rchildren) = rhs.children {
            if let Some(children) = &mut self.children {
                let children = children
                    .iter_mut()
                    .zip_longest(rchildren)
                    .map(|x| match x {
                        Both(l, r) => {
                            l.merge(op, r);
                            l.clone()
                        }
                        Left(l) => l.clone(),
                        Right(r) => r,
                    })
                    .collect();
                self.children = Some(children);
            } else {
                self.children = Some(rchildren);
            }
        }
    }
}

pub trait LayoutOp {
    fn merge_value(l: &mut Value, r: &Value) -> Option<Value>;
    fn merge(&self, lhs: &mut Layout, rhs: &mut Layout) {
        match (&mut lhs.bind, &mut rhs.bind) {
            (Some(l), Some(r)) => {
                let nv = l
                    .into_iter()
                    .chain(r)
                    .fold(HashMap::new(), |mut m, (k, v)| {
                        m.entry(k.to_owned())
                            .and_modify(|old: &mut Bind| {
                                let nd = match (&mut old.default, &v.default) {
                                    (Some(x), Some(y)) => Self::merge_value(x, y),
                                    (Some(x), None) => Some(x.clone()),
                                    (None, Some(y)) => Some(y.clone()),
                                    (None, None) => None,
                                };
                                old.default = nd;
                            })
                            .or_insert(v.to_owned());
                        m
                    });
                Some(nv)
            }
            (Some(l), None) => Some(l.to_owned()),
            (None, Some(y)) => Some(y.to_owned()),
            (None, None) => None,
        };
    }
}

pub struct Concat;
impl LayoutOp for Concat {
    fn merge_value(x: &mut Value, y: &Value) -> Option<Value> {
        let n = match (x, y) {
            (Value::Number(x), Value::Number(r)) => {
                json!(x.as_f64().unwrap() + r.as_f64().unwrap())
            }
            (Value::Bool(x), Value::Bool(r)) => {
                json!(*x || *r)
            }
            (Value::String(x), Value::String(r)) => {
                x.push_str(&r);
                json!(x)
            }
            (Value::Object(x), Value::Object(r)) => {
                for (k, v) in r {
                    x.entry(k)
                        .and_modify(|x| *x = v.clone())
                        .or_insert_with(|| v.clone());
                }
                json!(x)
            }
            (Value::Array(x), Value::Array(r)) => {
                json!([x.clone(), r.clone()].concat())
            }
            _ => y.clone(),
        };
        Some(n)
    }
}

pub struct Delete;
impl LayoutOp for Delete {
    fn merge_value(x: &mut Value, y: &Value) -> Option<Value> {
        let n = match (x, y) {
            (Value::Number(x), Value::Number(r)) => {
                json!(x.as_f64().unwrap() - r.as_f64().unwrap())
            }
            (Value::Bool(x), Value::Bool(r)) => {
                json!(*x && *r)
            }
            (Value::String(x), Value::String(r)) => {
                json!(x.replace(r, ""))
            }
            (Value::String(x), Value::Number(r)) => {
                let l = x.len();
                let s = r.as_u64().unwrap() as usize;
                let e = if s >= l { 0 } else { l.saturating_sub(s) };
                json!(x[..e])
            }
            (Value::Object(x), Value::Object(r)) => {
                for (k, _v) in r {
                    if x.contains_key(k) {
                        x.remove(k);
                    };
                }
                json!(x)
            }
            (Value::Array(x), Value::Array(_r)) => {
                json!(x)
            }
            _ => y.clone(),
        };
        Some(n)
    }
}

pub struct Replace;
impl LayoutOp for Replace {
    fn merge_value(x: &mut Value, r: &Value) -> Option<Value> {
        let y = match (x, r) {
            (Value::Number(_x), Value::Number(r)) => {
                json!(r.as_f64().unwrap())
            }
            (Value::Bool(_x), Value::Bool(r)) => {
                json!(*r)
            }
            (Value::String(_x), Value::String(r)) => {
                json!(r)
            }
            (Value::Object(_x), Value::Object(r)) => {
                json!(r)
            }
            (Value::Array(_x), Value::Array(r)) => {
                json!(r)
            }
            _ => r.clone(),
        };
        Some(y)
    }
}
