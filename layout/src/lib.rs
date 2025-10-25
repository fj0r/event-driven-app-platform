#[cfg(feature = "dioxus")]
use dioxus::prelude::*;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "classify")]
pub mod classify;
#[cfg(feature = "merge")]
pub mod merge;
#[cfg(feature = "render")]
pub mod render;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, to_value};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Container {
    #[allow(non_camel_case_types)]
    grid(Map<String, Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Render {
    pub name: String,
    pub data: Value,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum JsType {
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

impl JsType {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(untagged)]
pub enum BindVariant {
    Source {
        source: String,
    },
    Target {
        target: String,
    },
    Event {
        event: String,
    },
    Field {
        field: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<Value>,
        #[cfg(feature = "dioxus")]
        #[allow(dead_code)]
        #[serde(skip)]
        signal: Option<Signal<Value>>,
    },
    Submit {
        submit: bool,
        #[cfg(feature = "dioxus")]
        #[allow(dead_code)]
        #[serde(skip)]
        signal: Option<Signal<Value>>,
    },
    Default {},
}

impl Default for BindVariant {
    fn default() -> Self {
        BindVariant::Default {}
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Bind {
    #[serde(flatten)]
    pub variant: BindVariant,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<JsType>,
}

fn kind_empty() -> String {
    "empty".to_string()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
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
}
