use super::super::handler::{ArcShared, ChatMessage, Envelope, Sender};
use anyhow::Result;
use brick::{Bind, Brick, Text, TextAttr};
use content::{Content, Influx, Method};
use maplit::hashmap;
use std::default::Default;
use std::fmt::Debug;

pub async fn echo<T: Debug + Default>(e: ChatMessage<T>, s: ArcShared, x: Sender<T>) -> Result<()> {
    let ChatMessage {
        sender,
        created: _,
        content,
    } = &e;

    if let Some(content) = content.as_object()
        && let Some(d) = content.get("data")
    {
        let content = Content::Join(Influx {
            event: "chat".into(),
            channel: None,
            data: Brick::text(Text {
                id: None,
                attrs: Some(TextAttr {
                    format: Some("md".to_string()),
                    ..Default::default()
                }),
                bind: Some(hashmap! {
                    "value".to_owned() => Bind {
                        default: Some(d.to_owned()),
                        ..Default::default()
                    }
                }),
            }),
            method: Method::Concat,
        });
        if let Ok(content) = serde_json::to_value(content) {
            let cm: ChatMessage<T> = ("chat".into(), content).into();
            let _ = x.send(Envelope {
                receiver: vec![sender.clone()],
                message: cm,
            });
        }
    };
    Ok(())
}
