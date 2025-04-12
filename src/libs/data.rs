use serde::{Serialize, Deserialize};

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Layout {
    pub kind: String,
    pub data: Option<String>,
    pub item: Option<Vec<Box<Layout>>>,
    pub children: Option<Vec<Box<Layout>>>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;

