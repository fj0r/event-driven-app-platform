use figment::{
    Figment, Result,
    providers::{Env, Format, Toml},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

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

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct Settings {
    pub queue: Queue,
}

impl Settings {
    pub(crate) fn new() -> Result<Self> {
        Figment::new()
            .merge(Toml::file("chat.toml"))
            .merge(Env::prefixed("CHAT_").split("_"))
            .extract()
    }
}
