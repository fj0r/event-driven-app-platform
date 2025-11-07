use component::{Bind, BindVariant, Case, Input, JsonComponent as C, Text, TextAttr};
#[cfg(feature = "scratch")]
use maplit::hashmap;
use serde_json::{from_str, to_string};
use std::fs::read_to_string;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let a = C::case(Case {
        children: Some(vec![
            C::text(Text {
                attrs: Some(TextAttr {
                    format: Some("md".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            C::input(Input {
                bind: Some(hashmap! {
                    "value".to_owned() => Bind {
                        variant: BindVariant::Default {},
                        default: Some(from_str("\"test\"")?),
                        ..Default::default()
                    }
                }),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    println!("{:?}", to_string(&a));

    let chat_layout = read_to_string("component/examples/layout.json")?;
    let chat_layout: C = from_str(&chat_layout)?;
    println!("{:?}", &chat_layout);

    Ok(())
}
