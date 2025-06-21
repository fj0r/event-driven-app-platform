use figment::{
    Figment, Result,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;
use kafka::config::Queue;


#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    #[serde(rename = "type")]
    pub kind: String,
    pub host: String,
    pub port: u16,
    pub db: String,
    pub schema: Option<String>,
    pub user: String,
    pub passwd: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Config {
    pub queue: Queue,
    pub database: Database,
}

impl Config {
    pub fn new() -> Result<Self> {
        Figment::new()
            .merge(Toml::file("chat.toml"))
            .merge(Env::prefixed("CHAT_").split("_"))
            .extract()
    }
}
