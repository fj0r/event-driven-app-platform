use super::shared::Session;
use kafka::Created;
use message::Event;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Envelope {
    pub receiver: Vec<Session>,
    #[serde(flatten)]
    pub message: ChatMessage,
}

impl Event<Created> for Envelope {
    fn event(&self) -> Option<&str> {
        self.message.event()
    }
    fn set_time(&mut self, time: Created) {
        self.message.set_time(time);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ChatMessage {
    pub sender: Session,
    pub created: Option<Created>,
    pub content: Value,
}

impl From<(Session, Value)> for ChatMessage {
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

impl Event<Created> for ChatMessage {
    fn event(&self) -> Option<&str> {
        get_value_event(&self.content)
    }

    fn set_time(&mut self, time: Created) {
        self.created = Some(time);
    }
}
