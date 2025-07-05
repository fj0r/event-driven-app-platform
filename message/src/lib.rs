use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Envelope<Session, Created> {
    pub receiver: Vec<Session>,
    #[serde(flatten)]
    pub message: ChatMessage<Session, Created>,
}

impl<Session, Created> Event<Created> for Envelope<Session, Created> {
    fn event(&self) -> Option<&str> {
        self.message.event()
    }
    fn set_time(&mut self, time: Created) {
        self.message.set_time(time);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ChatMessage<Session, Created> {
    pub sender: Session,
    pub created: Option<Created>,
    pub content: Value,
}

impl<Session, Created> From<(Session, Value)> for ChatMessage<Session, Created>
where
    Created: Default,
{
    fn from(value: (Session, Value)) -> Self {
        ChatMessage {
            sender: value.0,
            created: Some(Created::default()),
            content: value.1,
        }
    }
}

fn get_value_event(v: &Value) -> Option<&str> {
    if v.is_object()
        && let Some(m) = v.as_object()
    {
        let r = m.get("event").and_then(|x| x.as_str());
        return r;
    };
    None
}

impl<Session, Created> Event<Created> for ChatMessage<Session, Created> {
    fn event(&self) -> Option<&str> {
        get_value_event(&self.content)
    }

    fn set_time(&mut self, time: Created) {
        self.created = Some(time);
    }
}
