use super::super::handler::{ArcShared, ChatMessage, Envelope, Sender};
use std::fmt::Debug;

pub async fn chat<T: Debug>(e: ChatMessage<T>, s: ArcShared, x: Sender<T>) {
    println!("chat => {:?}", e);
}
