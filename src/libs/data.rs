use serde::{Serialize, Deserialize};
use serde_json::Value;
use dioxus::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub user: String,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Content {
    #[warn(non_camel_case_types)]
    layout(Layout),

    #[warn(non_camel_case_types)]
    #[default]
    empty,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Layout {
    pub kind: String,
    pub data: Option<String>,
    pub item: Option<Vec<Layout>>,
    pub value: Option<Value>,
    pub children: Option<Vec<Layout>>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;

