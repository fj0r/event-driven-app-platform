use super::super::handler::{ArcShared, ChatMessage, Envelope, Sender};
use message::session::Session;
use std::fmt::Debug;

pub async fn chat<T: Debug>(e: ChatMessage<T>, s: ArcShared, x: Sender<T>) {
    let ChatMessage {
        sender,
        created,
        content,
    } = &e;

    if let Some(content) = content.as_object()
        && let Some(d) = content.get("data")
        && let Some(d) = d.as_str()
    {
        let _ = x.send(Envelope {
            receiver: vec![sender.clone()],
            message: e,
        });
    };
}
