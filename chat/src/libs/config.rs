use figment::{
    Figment, Result,
    providers::{Env, Format, Toml},
};
use kafka::config::Queue;
use serde::Deserialize;
use crate::concat_fields;


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


impl From<&Database> for String {
    fn from(value: &Database) -> Self {
        concat_fields!{
            var value;
            host;
            port;
            dbname = db;
            user;
            password = passwd;
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub enum LogFormat {
    #[allow(non_camel_case_types)]
    json,
    #[default]
    #[allow(non_camel_case_types)]
    compact,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub format: LogFormat,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Config {
    pub queue: Queue,
    pub database: Database,
    pub trace: Log,
}

impl Config {
    pub fn new() -> Result<Self> {
        Figment::new()
            .merge(Toml::file("chat.toml"))
            .merge(Env::prefixed("CHAT_").split("_"))
            .extract()
    }
}
