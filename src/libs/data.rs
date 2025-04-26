use std::ops::AddAssign;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

type Session = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub sender: Session,
    pub content: Content,
}

impl From<(String, String, Option<String>, Value)> for Content {
    fn from(value: (String, String, Option<String>, Value)) -> Self {
        Content::merge(Action {
            event: value.0,
            data: Layout {
                kind: value.1,
                id: value.2,
                value: Some(value.3),
                ..Default::default()
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Content {
    #[warn(non_camel_case_types)]
    create(Layout),

    #[warn(non_camel_case_types)]
    merge(Action),

    #[warn(non_camel_case_types)]
    join(Action),

    #[warn(non_camel_case_types)]
    #[default]
    empty,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Action {
    pub event: String,
    pub data: Layout,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Bind {
    #[serde(default)]
    pub upload: bool,
    #[serde(default)]
    pub list: bool,
    pub event: String,
    // TODO: local event when upload
    pub local: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize, Default)]
pub struct Layout {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: Option<String>,
    pub attrs: Option<Value>,
    pub data: Option<Bind>,
    pub value: Option<Value>,
    pub item: Option<Vec<Layout>>,
    pub children: Option<Vec<Layout>>,
}

impl Layout {
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
        return false;
    }
    pub fn merge(&mut self, rhs: Self) {
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
        if let Some(children) = &mut self.children {
            if let Some(rchildren) = rhs.children {
                for (l, r) in children.into_iter().zip(rchildren) {
                    l.merge(r);
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;
