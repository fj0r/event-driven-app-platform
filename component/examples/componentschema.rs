use component::JsonComponent;
use schemars::schema_for;

fn main() -> Result<(), &'static dyn std::error::Error> {
    let schema = schema_for!(JsonComponent);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    Ok(())
}
