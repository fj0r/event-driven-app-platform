//! ```cargo
//! [dependencies]
//! serde_json = "1.0.140"
//! serde = { version = "1.0.219", features = ["derive"] }
//! serde_json_path = "0.7.2"
//! ```

use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use serde_json_path::JsonPath;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Attrs {
    pub class: Option<String>,
    // for selector
    pub kind: Option<String>,
    pub horizontal: Option<bool>,
    #[serde(flatten)]
    pub settings: Option<Settings>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Settings {
    Container(Container),
    List {
        scroll: bool
    },
    Text {
        format: String,
    },
    Item {
        selector: String
    },
    Button {
        oneshot: bool
    },
    Input {
        target: String,
        instant: bool
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Container {
    table(Table),
    grid(Map<String, Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    column: u8,
    #[serde(default)]
    pub header: bool
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let a = r#"
    {
      "class": "maximize",
      "kind": "sdfa",
      "table": {"column": 3}
    }
        "#;
    let b = serde_json::from_str::<Attrs>(a).unwrap();
    println!("{:?}", b);
    if let Some(s) = b.settings {
        println!("{:?}", s);
    };
    Ok(())
}
