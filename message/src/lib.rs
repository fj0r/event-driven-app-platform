use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

pub mod session;
use session::Session;
pub mod queue;

pub trait Event<T> {
    fn event(&self) -> Option<&str>;
    fn set_time(&mut self, time: T);
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Envelope<Created> {
    pub receiver: Vec<Session>,
    #[serde(flatten)]
    pub message: ChatMessage<Created>,
}

impl<Created> Event<Created> for Envelope<Created> {
    fn event(&self) -> Option<&str> {
        self.message.event()
    }
    fn set_time(&mut self, time: Created) {
        self.message.set_time(time);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ChatMessage<Created> {
    pub sender: Session,
    pub created: Option<Created>,
    pub content: Value,
}

impl<Created> From<(Session, Value)> for ChatMessage<Created>
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

impl<Created> Event<Created> for ChatMessage<Created> {
    fn event(&self) -> Option<&str> {
        get_value_event(&self.content)
    }

    fn set_time(&mut self, time: Created) {
        self.created = Some(time);
    }
}
