//! ```cargo
//! [dependencies]
//! kafka = "0.10.0"
//! serde = { version = "1.0.219", features = ["derive"] }
//! minijinja = { version = "2.10.2", features = ["loader"] }
//! serde_derive = "1.0.219"
//! serde_json = "1.0.140"
//! ```
use serde_json::{to_value, Value, from_str, to_string};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FoldText {
    #[allow(non_camel_case_types)]
    begin,
    #[allow(non_camel_case_types)]
    end,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    fold: Option<FoldText>
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let x = Text { fold: Some(FoldText::begin) };
    let y = to_value(&x);
    println!("{}", to_string(&x)?);

    let a = from_str::<Text>(r#"{"fold":"end"}"#);
    dbg!(a);

    Ok(())
}
