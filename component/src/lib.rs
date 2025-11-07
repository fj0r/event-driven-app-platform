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

pub trait ComponentProps {
    fn get_children(&mut self) -> Option<&mut Vec<JsonComponent>>;
    fn set_children(&mut self, component: Vec<JsonComponent>);
    fn get_bind(&mut self) -> Option<&mut HashMap<String, Bind>>;
    fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>);
    fn get_render(&self) -> Option<&Render>;
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ClassAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Placeholder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Chart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Diagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FloatComp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FoldComp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FormComp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Popup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Svg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RackAttr {
    #[serde(default)]
    pub scroll: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Rack {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<RackAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<Render>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ButtonAttr {
    #[serde(default)]
    pub oneshot: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Button {
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ButtonAttr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ImageAttr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(default)]
    pub thumb: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ImageAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Input {
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Select {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Table {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TextAttr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TextArea {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Case {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<CaseAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<Render>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(tag = "type")]
pub enum JsonComponent {
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

impl ComponentProps for JsonComponent {
    fn get_children(&mut self) -> Option<&mut Vec<JsonComponent>> {
        match self {
            JsonComponent::placeholder(c) => c.children.as_mut(),
            JsonComponent::case(c) => c.children.as_mut(),
            JsonComponent::rack(c) => c.children.as_mut(),
            JsonComponent::float(c) => c.children.as_mut(),
            JsonComponent::fold(c) => c.children.as_mut(),
            JsonComponent::popup(c) => c.children.as_mut(),
            JsonComponent::table(c) => c.children.as_mut(),
            JsonComponent::form(c) => c.children.as_mut(),
            JsonComponent::select(c) => c.children.as_mut(),
            JsonComponent::svg(c) => c.children.as_mut(),
            JsonComponent::chart(c) => c.children.as_mut(),
            JsonComponent::diagram(c) => c.children.as_mut(),
            _ => None,
        }
    }
    fn set_children(&mut self, component: Vec<JsonComponent>) {
        match self {
            JsonComponent::placeholder(c) => c.children = Some(component),
            JsonComponent::case(c) => c.children = Some(component),
            JsonComponent::rack(c) => c.children = Some(component),
            JsonComponent::float(c) => c.children = Some(component),
            JsonComponent::fold(c) => c.children = Some(component),
            JsonComponent::popup(c) => c.children = Some(component),
            JsonComponent::table(c) => c.children = Some(component),
            JsonComponent::form(c) => c.children = Some(component),
            JsonComponent::select(c) => c.children = Some(component),
            JsonComponent::svg(c) => c.children = Some(component),
            JsonComponent::chart(c) => c.children = Some(component),
            JsonComponent::diagram(c) => c.children = Some(component),
            _ => {}
        };
    }

    fn get_bind(&mut self) -> Option<&mut HashMap<String, Bind>> {
        match self {
            //Component::placeholder(c) => c.bind.as_mut(),
            JsonComponent::case(c) => c.bind.as_mut(),
            JsonComponent::rack(c) => c.bind.as_mut(),
            //Component::float(c) => c.bind.as_mut(),
            //Component::fold(c) => c.bind.as_mut(),
            //Component::popup(c) => c.bind.as_mut(),
            //Component::table(c) => c.bind.as_mut(),
            //Component::form(c) => c.bind.as_mut(),
            JsonComponent::select(c) => c.bind.as_mut(),
            //Component::svg(c) => c.bind.as_mut(),
            //Component::chart(c) => c.bind.as_mut(),
            //Component::diagram(c) => c.bind.as_mut(),
            _ => None,
        }
    }

    fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>) {
        match self {
            //Component::placeholder(c) => c.bind = bind,
            JsonComponent::case(c) => c.bind = bind,
            JsonComponent::rack(c) => c.bind = bind,
            //Component::float(c) => c.bind = bind,
            //Component::fold(c) => c.bind = bind,
            //Component::popup(c) => c.bind = bind,
            //Component::table(c) => c.bind = bind,
            //Component::form(c) => c.bind = bind,
            JsonComponent::select(c) => c.bind = bind,
            //Component::svg(c) => c.bind = bind,
            //Component::chart(c) => c.bind = bind,
            //Component::diagram(c) => c.bind = bind,
            _ => {}
        }
    }

    fn get_render(&self) -> Option<&Render> {
        match self {
            //Component::placeholder(c) => None,
            JsonComponent::case(c) => c.render.as_ref(),
            JsonComponent::rack(c) => c.render.as_ref(),
            //Component::float(c) => c.bind = bind,
            //Component::fold(c) => c.bind = bind,
            //Component::popup(c) => c.bind = bind,
            //Component::table(c) => c.bind = bind,
            //Component::form(c) => c.bind = bind,
            //Component::select(c) => None,
            //Component::svg(c) => c.bind = bind,
            //Component::chart(c) => c.bind = bind,
            //Component::diagram(c) => c.bind = bind,
            _ => None,
        }
    }
}
