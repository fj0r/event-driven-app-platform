use dioxus::prelude::*;
use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};
use minijinja::Environment;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use time::serde::rfc3339;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Created(#[serde(with = "rfc3339")] OffsetDateTime);

impl Default for Created {
    fn default() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

type Session = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub sender: Session,
    pub created: Option<Created>,
    pub content: Content,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Outflow {
    pub event: String,
    pub id: Option<String>,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Content {
    #[allow(non_camel_case_types)]
    create(Influx),

    #[allow(non_camel_case_types)]
    tmpl(InfluxTmpl),

    #[allow(non_camel_case_types)]
    merge(Influx),

    #[allow(non_camel_case_types)]
    join(Influx),

    #[allow(non_camel_case_types)]
    #[default]
    empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfluxTmpl {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Influx {
    pub event: String,
    pub data: Layout,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bind {
    Event {
        event: String,
        #[serde(rename = "type")]
        // number, bool, [text]
        kind: Option<String>,
        // Abandon
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Attrs {
    pub class: Option<String>,
    // for selector
    pub kind: Option<String>,
    pub horizontal: Option<bool>,
    #[serde(flatten)]
    pub settings: Option<Settings>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Settings {
    Container(Container),
    Rack {
        scroll: bool,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Container {
    #[allow(non_camel_case_types)]
    grid(Map<String, Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub column: usize,
    #[serde(default)]
    pub header: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Render {
    pub name: String,
    pub data: Value,
}

fn kind_empty() -> String {
    "empty".to_string()
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize, Default)]
pub struct Layout {
    #[serde(rename = "type", default = "kind_empty")]
    pub kind: String,
    pub id: Option<String>,
    pub attrs: Option<Attrs>,
    pub data: Option<Bind>,
    pub value: Option<Value>,
    pub render: Option<Render>,
    pub item: Option<Vec<Layout>>,
    pub children: Option<Vec<Layout>>,
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
        if let Some(id) = &self.id {
            if let Some(oid) = &other.id {
                if id == oid {
                    return true;
                }
            }
        }
        false
    }

    pub fn render(&mut self, env: &Environment) {
        if let Some(r) = &self.render {
            let n = &r.name;
            let cx = &r.data;
            let n = env
                .get_template(&n)
                .map_err(|e| e.to_string())
                .and_then(|t| t.render(cx).map_err(|e| format!("render failed: {}", e)))
                .and_then(|t| {
                    serde_json::from_str::<Layout>(&t)
                        .map_err(|e| format!("deserialize failed: {}", e))
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

    pub fn join(&mut self, rhs: Self) {
        let value = match &self.value {
            Some(x) => {
                if let Some(r) = rhs.value {
                    let y = match (x, &r) {
                        (Value::Number(x), Value::Number(r)) => {
                            json!(x.as_f64().unwrap() + r.as_f64().unwrap())
                        }
                        (Value::Bool(x), Value::Bool(r)) => {
                            json!(*x && *r)
                        }
                        (Value::String(x), Value::String(r)) => {
                            let mut x = x.clone();
                            x.push_str(r);
                            json!(x)
                        }
                        (Value::Object(x), Value::Object(r)) => {
                            let mut x = x.clone();
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
            None => rhs.value,
        };
        self.value = value;
        if let Some(rchildren) = rhs.children {
            if let Some(children) = &mut self.children {
                let children = children
                    .iter_mut()
                    .zip_longest(rchildren)
                    .map(|x| match x {
                        Both(l, r) => {
                            l.join(r);
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;
