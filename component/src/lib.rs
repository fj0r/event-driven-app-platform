#[cfg(feature = "dioxus")]
use dioxus::prelude::*;
use itertools::FoldWhile;
#[cfg(feature = "schema")]
use schemars::JsonSchema;

use serde::ser::SerializeTupleStruct;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, to_value};
use std::collections::HashMap;
use std::fmt::Debug;
use std::num::ParseFloatError;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Placeholder {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Chart {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Diagram {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FloatComp {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FoldComp {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FormComp {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Popup {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Svg {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Rack {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Button {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Image {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Input {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Select {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Table {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Text {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TextArea {
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CaseAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal: Option<bool>,
    #[allow(non_camel_case_types)]
    pub grid: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Case {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<CaseAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<Render>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Component>>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(tag = "type")]
pub enum Component {
    case(Case),
    placeholder(Placeholder),
    chart(Chart),
    diagram(Diagram),
    float(FloatComp),
    fold(FoldComp),
    form(FormComp),
    popup(Popup),
    svg(Svg),
    rack(Rack),
    button(Button),
    image(Image),
    input(Input),
    select(Select),
    table(Table),
    text(Text),
    textarea(TextArea),
}
