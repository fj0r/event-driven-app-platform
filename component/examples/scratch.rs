use component::{Case, JsonComponent as C, Text};
use serde_json::{from_str, to_string};
use std::fs::read_to_string;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let a = C::case(Case {
        children: Some(vec![C::text(Text {
            ..Default::default()
        })]),
        ..Default::default()
    });
    println!("{:?}", to_string(&a));

    let chat_layout = read_to_string("examples/layout.json")?;
    let chat_layout: C = from_str(&chat_layout)?;
    println!("{:?}", &chat_layout);

    Ok(())
}
