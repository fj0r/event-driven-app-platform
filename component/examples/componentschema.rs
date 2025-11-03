use component::Component;
use schemars::schema_for;

fn main() -> Result<(), &'static dyn std::error::Error> {
    let schema = schema_for!(Component);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    Ok(())
}
