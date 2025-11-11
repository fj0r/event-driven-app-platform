#[cfg(feature = "dioxus")]
use dioxus::prelude::*;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "classify")]
pub mod classify;
#[cfg(feature = "classify")]
use classify::Classify;
#[cfg(feature = "merge")]
pub mod merge;
#[cfg(feature = "render")]
pub mod render;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, to_value};
use std::collections::HashMap;
use std::fmt::Debug;

pub trait ComponentProps {
    fn get_type(&self) -> &str;
    fn get_children(&mut self) -> Option<&mut Vec<JsonComponent>>;
    fn set_children(&mut self, component: Vec<JsonComponent>);
    fn get_attrs(&self) -> Option<&dyn Classify>;
    fn get_bind(&self) -> Option<&HashMap<String, Bind>>;
    fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>);
    fn get_id(&self) -> &Option<String>;
    //fn cmp_id(&self, other: &Self) -> bool;
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
pub struct ClassAttr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Placeholder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Chart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Diagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Float {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Fold {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Form {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Popup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Svg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Path {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<ClassAttr>,
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
    pub class: Option<Vec<String>>,
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
    pub class: Option<Vec<String>>,
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
    pub class: Option<Vec<String>>,
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
    pub attrs: Option<ClassAttr>,
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
    pub attrs: Option<ClassAttr>,
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
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Thead {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Tbody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Tr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Th {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Td {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TextAttr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<Vec<String>>,
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
    pub attrs: Option<TextAttr>,
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
    pub attrs: Option<ClassAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "dioxus", derive(Props))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CaseAttr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<Vec<String>>,
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
    pub attrs: Option<CaseAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, Bind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<Render>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<JsonComponent>>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(tag = "type")]
pub enum JsonComponent {
    case(Case),
    placeholder(Placeholder),
    chart(Chart),
    diagram(Diagram),
    float(Float),
    fold(Fold),
    form(Form),
    popup(Popup),
    svg(Svg),
    group(Group),
    path(Path),
    rack(Rack),
    button(Button),
    image(Image),
    input(Input),
    select(Select),
    table(Table),
    thead(Thead),
    tbody(Tbody),
    tr(Tr),
    th(Th),
    td(Td),
    text(Text),
    textarea(TextArea),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(tag = "type")]
pub enum JsonTableComponent {
    thead,
    tbody,
    tr,
    th,
    td,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(tag = "type")]
pub enum JsonSvgComponent {
    group,
    path,
}

macro_rules! impl_component_props {
    ($($type: ident),*) => {};
}

impl_component_props![];

impl ComponentProps for Placeholder {
    fn get_id(&self) -> &Option<String> {
        &self.id
    }
    fn get_type(&self) -> &str {
        "placeholder"
    }
    fn get_children(&mut self) -> Option<&mut Vec<JsonComponent>> {
        self.children.as_mut()
    }
    fn set_children(&mut self, component: Vec<JsonComponent>) {
        self.children = Some(component);
    }
    fn get_attrs(&self) -> Option<&dyn Classify> {
        Some(&self.attrs)
    }
    fn get_bind(&self) -> Option<&HashMap<String, Bind>> {
        //self.bind.as_ref()
        None
    }
    fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>) {}
    fn get_render(&self) -> Option<&Render> {
        //self.render.as_ref()
        None
    }
}

impl JsonComponent {
    pub fn cmp_id(&self, other: &Self) -> bool {
        let Some(id) = self.get_id() else {
            return false;
        };
        let Some(oid) = other.get_id() else {
            return false;
        };
        id == oid
    }
}

impl ComponentProps for JsonComponent {
    fn get_id(&self) -> &Option<String> {
        macro_rules! m {
            ($s:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(c) => &c.id,)*
                    _ => &None
                }
            };
        }
        m![self =>
            placeholder, case, rack, float, fold, popup,
            table, form, select, svg, chart, diagram,
        ]
    }

    fn get_children(&mut self) -> Option<&mut Vec<JsonComponent>> {
        macro_rules! m {
            ($s:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(c) => c.children.as_mut(),)*
                    _ => None
                }
            };
        }
        m![self =>
            placeholder, case, rack, float, fold, popup,
            table, thead, tbody, tr, th, td,
            form, select, svg, group, chart, diagram,
        ]
    }

    fn set_children(&mut self, component: Vec<JsonComponent>) {
        macro_rules! m {
            ($s:ident, $p:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(c) => { c.children = Some($p) })*
                    _ => {}
                }
            };
        }
        m![self, component =>
            placeholder, case, rack, float, fold, popup,
            table, thead, tbody, tr, th, td,
            form, select, svg, group, chart, diagram,
        ];
    }

    fn get_bind(&self) -> Option<&HashMap<String, Bind>> {
        macro_rules! m {
            ($s:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(c) => { c.bind.as_ref() })*
                    _ => None
                }
            }
        }
        m![self =>
            case, rack, select,
            // placeholder, float, fold, popup,
            // table, form, svg, chart, diagram,
        ]
    }

    fn set_bind(&mut self, bind: Option<HashMap<String, Bind>>) {
        macro_rules! m {
            ($s:ident, $p:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(c) => { c.bind = $p })*
                    _ => {}
                }
            }
        }
        m![ self, bind =>
            case, rack, select,
            // placeholder, float, fold, popup,
            // table, form, svg, chart, diagram,
        ]
    }

    fn get_render(&self) -> Option<&Render> {
        macro_rules! m {
            ($s:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(c) => c.render.as_ref(), )*
                    _ => None
                }
            }
        }
        m![ self =>
            case, rack,
            // placeholder, float, fold, popup,
            // table, form, select, svg, chart, diagram,
        ]
    }

    fn get_type(&self) -> &str {
        macro_rules! m {
            ($s:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(_) => stringify!($c),)*
                    _ => &"!"
                }
            };
        }
        m![self =>
            placeholder, case, rack, float, fold, popup,
            table, form, select, svg, chart, diagram,
        ]
    }

    fn get_attrs(&self) -> Option<&dyn Classify> {
        macro_rules! m {
            ($s:ident => $($c: ident),* $(,)?) => {
                match $s {
                    //$(JsonComponent::$c(c) => c.attrs.clone().map(|x| &x as &dyn Classify),)*
                    $(JsonComponent::$c(c) => Some(&c.attrs) ,)*
                    _ => None
                }
            };
        }
        m![self =>
            placeholder, case, rack, float, fold, popup,
            form, select, svg, chart, diagram,
        ]
    }
}
