use layout::Layout;
use schemars::schema_for;

fn main() -> Result<(), &'static dyn std::error::Error> {
    let schema = schema_for!(Layout);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    Ok(())
}
