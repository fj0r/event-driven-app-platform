use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::{
    Mutex,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

pub trait MessageQueueOutgo {
    type Item: Debug + Send + Serialize + for<'a> Deserialize<'a>;

    #[allow(unused)]
    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send;

    #[allow(unused)]
    fn get_tx(&self) -> Option<UnboundedSender<Self::Item>>;
}

pub trait MessageQueueIncome {
    type Item: Debug + Send + Serialize + for<'a> Deserialize<'a>;

    #[allow(unused)]
    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send;

    #[allow(unused)]
    fn get_rx(&self) -> Option<Arc<Mutex<UnboundedReceiver<Self::Item>>>>;
}
