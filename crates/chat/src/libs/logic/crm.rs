use super::super::handler::{ArcShared, ChatMessage, Envelope, Sender};
use anyhow::Result;
use std::fmt::Debug;

pub async fn crm<T: Debug>(e: ChatMessage<T>, s: ArcShared, x: Sender<T>) -> Result<()> {
    println!("crm => {:?}", e);
    Ok(())
}
