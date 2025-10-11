use chrono::{DateTime, Utc};
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
pub struct Message<T> {
    pub sender: Session,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Created>,
    pub content: Content<T>,
}

impl<T> From<(Session, Content<T>)> for Message<T> {
    fn from(value: (Session, Content<T>)) -> Self {
        Message {
            sender: value.0,
            created: Some(Created::default()),
            content: value.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Outflow {
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Content<T> {
    #[serde(rename = "create")]
    Create(Influx<T>),

    #[serde(rename = "tmpl")]
    Tmpl(InfluxTmpl),

    #[serde(rename = "set")]
    Set(Influx<T>),

    #[serde(rename = "join")]
    Join(Influx<T>),

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
pub struct Influx<T> {
    pub event: String,
    pub data: T,
    #[serde(default)]
    pub method: Method,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub column: usize,
    #[serde(default)]
    pub header: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;
