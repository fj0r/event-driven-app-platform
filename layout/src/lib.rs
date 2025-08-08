use dioxus::prelude::*;
use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};
use minijinja::Environment;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct Attrs {
    pub class: Option<String>,
    // for selector
    pub kind: Option<String>,
    pub horizontal: Option<bool>,
    #[serde(flatten)]
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
        width: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Bind {
    Event {
        event: String,
        #[serde(rename = "type")]
        // number, bool, [text]
        kind: Option<String>,
        // TODO:
        local: Option<String>,
    },
    Field {
        field: String,
        #[serde(rename = "type")]
        // number, bool, [text]
        kind: Option<String>,
        payload: Option<Value>,
        #[allow(dead_code)]
        #[serde(skip)]
        signal: Option<Signal<Value>>,
    },
    Submit {
        submit: bool,
        #[allow(dead_code)]
        #[serde(skip)]
        signal: Option<Signal<Value>>,
    },
}

fn kind_empty() -> String {
    "empty".to_string()
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize, Default, JsonSchema)]
pub struct Layout {
    #[serde(rename = "type", default = "kind_empty")]
    pub kind: String,
    pub id: Option<String>,
    pub attrs: Option<Attrs>,
    pub bind: Option<Bind>,
    pub data: Option<Value>,
    pub render: Option<Render>,
    pub item: Option<Vec<Layout>>,
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
            ..Layout::default()
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

    pub fn merge(&mut self, op: &(impl LayoutOp + ?Sized), rhs: Self) {
        op.merge(self, &rhs);
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
    fn merge(&self, lhs: &mut Layout, rhs: &Layout);
}

pub struct Concat;
impl LayoutOp for Concat {
    fn merge(&self, lhs: &mut Layout, rhs: &Layout) {
        let data = match &mut lhs.data {
            Some(x) => {
                if let Some(r) = &rhs.data {
                    let y = match (x, r) {
                        (Value::Number(x), Value::Number(r)) => {
                            json!(x.as_f64().unwrap() + r.as_f64().unwrap())
                        }
                        (Value::Bool(x), Value::Bool(r)) => {
                            json!(*x || *r)
                        }
                        (Value::String(x), Value::String(r)) => {
                            x.push_str(r);
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
                        _ => r.clone(),
                    };
                    Some(y)
                } else {
                    Some(x.clone())
                }
            }
            None => rhs.data.clone(),
        };
        lhs.data = data;
    }
}

pub struct Delete;
impl LayoutOp for Delete {
    fn merge(&self, lhs: &mut Layout, rhs: &Layout) {
        let data = match &mut lhs.data {
            Some(x) => {
                if let Some(r) = &rhs.data {
                    let y = match (x, r) {
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
                        _ => lhs.data.clone().unwrap_or_else(|| r.clone()),
                    };
                    Some(y)
                } else {
                    Some(x.clone())
                }
            }
            None => rhs.data.clone(),
        };
        lhs.data = data;
    }
}

pub struct Replace;
impl LayoutOp for Replace {
    fn merge(&self, lhs: &mut Layout, rhs: &Layout) {
        let data = match &lhs.data {
            Some(x) => {
                if let Some(r) = &rhs.data {
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
                } else {
                    Some(x.clone())
                }
            }
            None => rhs.data.clone(),
        };
        lhs.data = data;
    }
}
