use super::shared::Shared;
use anyhow::Result;
pub use message::{ChatMessage, Envelope};
use std::marker::Send;
use std::sync::Arc;
use tokio::sync::{
    Mutex, RwLock,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

pub type Sender<T> = UnboundedSender<Envelope<T>>;
pub type ArcShared = Arc<RwLock<Shared>>;

pub async fn handler<T, F, Fut>(
    tx: Sender<T>,
    rx: Arc<Mutex<UnboundedReceiver<ChatMessage<T>>>>,
    shared: Shared,
    mut f: F,
) -> Result<()>
where
    T: Send + 'static,
    F: FnMut(ChatMessage<T>, ArcShared, Sender<T>) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Result<()>> + Send,
{
    let shared = Arc::new(RwLock::new(shared));

    tokio::spawn(async move {
        let mut rx = rx.lock().await;
        while let Some(x) = rx.recv().await {
            f(x, shared.clone(), tx.clone()).await;
        }
    });
    Ok(())
}
