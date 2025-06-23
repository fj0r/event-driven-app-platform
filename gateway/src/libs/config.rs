use figment::{
    Figment, Result,
    providers::{Env, Format, Toml},
};
use notify::{Event, RecursiveMode, Result as ResultN, Watcher, recommended_watcher};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, mpsc::channel};
use tokio::sync::Mutex;
use kafka::config::Queue;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum HookVariant {
    Path {
        path: String,
    },
    Webhook {
        endpoint: String,
        #[serde(default = "default_accept")]
        accept: String,
    },
}

impl From<HookVariant> for Webhook {
    fn from(value: HookVariant) -> Self {
        if let HookVariant::Webhook { endpoint, accept } = value {
            Self {
                enable: true,
                accept,
                endpoint,
            }
        } else {
            Self {
                enable: false,
                accept: default_accept(),
                endpoint: "".to_string(),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Hooks {
    #[serde(default)]
    pub enable: bool,
    #[serde(flatten)]
    pub variant: HookVariant,
}

pub type HookList = Vec<Hooks>;

fn default_accept() -> String {
    "application/json".to_owned()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Webhook {
    pub enable: bool,
    pub endpoint: String,
    #[serde(default = "default_accept")]
    pub accept: String,
}

pub type WebhookMap = HashMap<String, Webhook>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Login {
    pub endpoint: String
}

pub const ASSETS_PATH: &str = "manifest";

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub queue: Queue,
    pub webhooks: WebhookMap,
    pub greet: HookList,
    pub login: Login,
    pub logout: Login,
}

impl Settings {
    pub fn new() -> Result<Self> {
        Figment::new()
            .merge(Toml::file("gateway.toml"))
            .merge(Env::prefixed("GATEWAY_").split("_"))
            .extract()
    }
}

pub struct Config {
    pub data: Arc<Mutex<Settings>>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let x = Settings::new()?;
        Ok(Self {
            data: Arc::new(Mutex::new(x)),
        })
    }

    #[allow(dead_code)]
    pub async fn listen(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = channel::<ResultN<Event>>();
        let mut watcher = recommended_watcher(tx)?;
        watcher.watch(Path::new("config.toml"), RecursiveMode::Recursive)?;
        let d = self.data.clone();
        tokio::task::spawn_blocking(|| async move {
            for res in rx {
                if res?.kind.is_modify() {
                    let n = Settings::new()?;
                    dbg!("config update: {:?}", &n);
                    let mut x = d.lock().await;
                    *x = n;
                }
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        });
        Ok(())
    }
}
