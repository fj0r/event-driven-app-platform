use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct QueuePush {
    #[serde(rename = "type")]
    pub kind: String,
    pub broker: Vec<String>,
    pub topic: Vec<String>,
    pub group: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct QueueEvent {
    #[serde(rename = "type")]
    pub kind: String,
    pub broker: Vec<String>,
    pub topic: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Queue {
    pub enable: bool,
    pub event: QueueEvent,
    pub push: QueuePush,
}
