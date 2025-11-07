use component::{Case, JsonComponent as C, Text};
use serde_json::to_string;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let a = C::case(Case {
        children: Some(vec![C::text(Text {
            ..Default::default()
        })]),
        ..Default::default()
    });
    println!("{:?}", to_string(&a));
    Ok(())
}
