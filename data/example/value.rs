
//! ```cargo
//! [dependencies]
//! serde_json = "1.0.140"
//! serde = { version = "1.0.219", features = ["derive"] }
//! ```

use serde::{Serialize, Deserialize};
use serde_json::{Value, Map, from_str};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let a = r#"{"a": "123"}"#;
    let x: Map<String, Value> = from_str(&a)?;
    println!("{:?}", x);
    let mut a = Map::new();
    a.insert("a".to_string(), "a".into());
    let b: Value = Value::Object(a);
    println!("{:?}", b);
    Ok(())
}
