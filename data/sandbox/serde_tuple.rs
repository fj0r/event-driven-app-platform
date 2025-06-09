//! ```cargo
//! [dependencies]
//! kafka = "0.10.0"
//! serde = { version = "1.0.219", features = ["derive"] }
//! minijinja = { version = "2.10.2", features = ["loader"] }
//! serde_derive = "1.0.219"
//! serde_json = "1.0.140"
//! ```
use serde_json::{to_value, Value, from_str, from_value};
use std::collections::HashMap;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let a = ("a", 1);
    let b = to_value(&a);
    dbg!(b);
    let c: (&str, usize) = from_str("[\"a\", 1]")?;
    dbg!(c);
    let d: Value = from_str("[\"a\", 1]")?;
    dbg!(&d);
    let z = from_value::<(String, usize)>(d);
    dbg!(z);

    let mut x = HashMap::new();
    {
        let y = "asd";
        x.insert(&y, 123);
        dbg!(x);
    }
    // dbg!(x);
    Ok(())
}
