//! ```cargo
//! [dependencies]
//! serde_json = "1.0.140"
//! serde_json_path = "0.7.2"
//! ```
use serde_json::{from_str, Value};
use serde_json_path::JsonPath;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jt = r#"
    {
      "action": "create",
      "attrs": {
        "class": "maximize"
      },
      "type": "container",
      "children": [
        {
          "type": "text",
          "value": "chat with AI",
          "attrs": {
            "class": "shadow nogrow",
            "format": "md"
          },
          "data": {
              "event": "title"
          }
        },
        {
          "type": "list",
          "attrs": {
            "class": "card xs gap scrolly"
          },
          "data": {
            "event": "chat"
          },
          "item": [
            {
              "type": "card",
              "attrs": {
                "class": "nogrow"
              }
            }
          ]
        },
        {
          "type": "container",
          "attrs": {
            "class": "nogrow card",
            "horizontal": true
          },
          "children": [
            {
              "type": "container",
              "attrs": {
                "horizontal": true
              },
              "children": [
                {
                  "type": "input",
                  "data": {
                    "upload": true,
                    "event": "message"
                  }
                }
              ]
            }
          ]
        }
      ]
    }
    "#;
    let j: Value = from_str(&jt)?;
    let p: JsonPath = JsonPath::parse("$.children[0].data.event")?;
    let r = p.query(&j).exactly_one()?.as_str();
    println!("{:?}", &r);
    Ok(())
}
