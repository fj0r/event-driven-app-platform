use chrono::{DateTime, Utc};
use layout::Layout;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Created(DateTime<Utc>);

impl Default for Created {
    fn default() -> Self {
        Self(Utc::now())
    }
}

type Session = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub sender: Session,
    pub created: Option<Created>,
    pub content: Content,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Outflow {
    pub event: String,
    pub id: Option<String>,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Content {
    #[serde(rename = "create")]
    Create(Influx),

    #[serde(rename = "tmpl")]
    Tmpl(InfluxTmpl),

    #[serde(rename = "set")]
    Set(Influx),

    #[serde(rename = "join")]
    Join(Influx),

    #[serde(rename = "empty")]
    #[default]
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfluxTmpl {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "concat")]
    Concat,
    #[serde(rename = "delete")]
    Delete,
}

impl Default for Method {
    fn default() -> Self {
        Self::Replace
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Influx {
    pub event: String,
    pub data: Layout,
    #[serde(default)]
    pub method: Method,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub column: usize,
    #[serde(default)]
    pub header: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;
