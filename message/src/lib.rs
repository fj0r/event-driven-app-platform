use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::{
    Mutex,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

pub trait Event<T> {
    fn event(&self) -> Option<&str>;
    fn set_time(&mut self, time: T);
}

pub trait MessageQueueEvent {
    type Item: Debug + Send + Serialize + serde::de::DeserializeOwned;

    #[allow(unused)]
    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send;

    #[allow(unused)]
    fn get_tx(&self) -> Option<UnboundedSender<Self::Item>>;
}

pub trait MessageQueuePush {
    type Item: Debug + Send + Serialize + serde::de::DeserializeOwned;

    #[allow(unused)]
    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send;

    #[allow(unused)]
    fn get_rx(&self) -> Option<Arc<Mutex<UnboundedReceiver<Self::Item>>>>;
}
