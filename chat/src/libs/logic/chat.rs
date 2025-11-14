use super::super::handler::{ArcShared, ChatMessage, Envelope, Sender};
use anyhow::Result;
use brick::{Bind, BindVariant, Brick, Text, TextAttr};
use content::{Content, Influx, Method};
use maplit::hashmap;
use std::default::Default;
use std::fmt::Debug;

pub async fn chat<T: Debug + Default>(e: ChatMessage<T>, s: ArcShared, x: Sender<T>) -> Result<()> {
    let ChatMessage {
        sender,
        // TODO: channel_id,
        created: _,
        content,
    } = &e;

    let s = s.read().await;
    // TODO: channel id
    let users = s.db.list_channel_account(3).await;
    dbg!(&users);

    if let Some(content) = content.as_object()
        && let Some(d) = content.get("data")
    {
        let content = Content::Join(Influx {
            event: "chat".into(),
            channel: None,
            data: Brick::text(Text {
                attrs: Some(TextAttr {
                    format: Some("md".to_string()),
                    ..Default::default()
                }),
                bind: Some(hashmap! {
                    "value".to_owned() => Bind {
                        variant: BindVariant::Default {},
                        default: Some(d.to_owned()),
                        ..Default::default()
                    }
                }),
                ..Default::default()
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
