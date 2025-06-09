//! ```cargo
//! [dependencies]
//! serde_json = "1.0.140"
//! serde = { version = "1.0.219", features = ["derive"] }
//! serde_json_path = "0.7.2"
//! ```

use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json_path::JsonPath;

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
pub struct InfluxTmpl {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub enum Target {
    Map,
    List
}

impl Default for Target {
    fn default() -> Self {
        Self::Map;
    }
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Influx {
    pub event: String,
    pub data: Layout,
    #[serde(default)]
    pub kind: Target
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Content {
    #[allow(non_camel_case_types)]
    create(Influx),

    #[allow(non_camel_case_types)]
    tmpl(InfluxTmpl),

    #[allow(non_camel_case_types)]
    merge(Influx),

    #[allow(non_camel_case_types)]
    join(Influx),

    #[allow(non_camel_case_types)]
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
    let p = JsonPath::parse("$.children[0].types")?;
    let x1 = serde_json::from_str(&s)?;
    let x = p.query(&x1).exactly_one();
    println!("---\n{:?}", x);
    Ok(())
}

