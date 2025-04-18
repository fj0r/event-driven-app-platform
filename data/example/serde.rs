//! ```cargo
//! [dependencies]
//! serde_json = "1.0.140"
//! serde = { version = "1.0.219", features = ["derive"] }
//! ```

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Layout {
    #[serde(rename = "type")]
    pub kind: String,
    pub data: Option<String>,
    pub item: Option<Vec<Box<Layout>>>,
    pub children: Option<Vec<Box<Layout>>>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Message {
    #[warn(non_camel_case_types)]
    layout(Layout),

    #[warn(non_camel_case_types)]
    #[default]
    empty,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = r#"
{
  "action": "layout",
  "type": "box",
  "children": [
    {
      "type": "header",
      "title": "test"
    },
    {
      "type": "scroll",
      "data": "chat",
      "item": [
        {
          "type": "card"
        }
      ]
    },
    {
      "type": "input",
      "data": "message"
    }
  ]
}
    "#;
    let a = serde_json::from_str::<Message>(&s)?;
    println!("{:?}", a);
    println!("{}", serde_json::to_string::<Message>(&a)?);
    Ok(())
}
