use super::super::handler::{ArcShared, ChatMessage, Envelope, Sender};
use content::{Content, Influx, Method};
use layout::{Attrs, Layout, Settings};
use message::session::Session;
use std::default::Default;
use std::fmt::Debug;

pub async fn chat<T: Debug + Default>(e: ChatMessage<T>, s: ArcShared, x: Sender<T>) {
    let ChatMessage {
        sender,
        created,
        content,
    } = &e;

    if let Some(content) = content.as_object()
        && let Some(d) = content.get("data")
    {
        let content = Content::Join(Influx {
            event: "chat".into(),
            data: Layout {
                kind: "text".into(),
                attrs: Some(Attrs {
                    settings: Some(Settings::Text {
                        format: "md".into(),
                    }),
                    ..Default::default()
                }),
                data: Some(d.to_owned()),
                ..Default::default()
            },
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
}
