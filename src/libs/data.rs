use serde::{Serialize, Deserialize};
use serde_json::Value;
use dioxus::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub sender: String,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Content {
    #[warn(non_camel_case_types)]
    layout(Layout),


    #[warn(non_camel_case_types)]
    data(Action),

    #[warn(non_camel_case_types)]
    append(Action),

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
    pub event: String,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Layout {
    pub kind: String,
    pub attrs: Option<Value>,
    pub data: Option<Bind>,
    pub value: Option<Value>,
    pub item: Option<Vec<Layout>>,
    pub children: Option<Vec<Layout>>
}

impl Layout {
    pub fn new(kind: impl AsRef<str>) -> Self {
        Layout {
            kind: kind.as_ref().to_string(),
            ..Layout::default()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;

