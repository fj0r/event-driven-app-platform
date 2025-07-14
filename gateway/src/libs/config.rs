use figment::{
    Figment, Result,
    providers::{Env, Format, Toml},
};
use kafka::config::Queue;
use notify::{Event, RecursiveMode, Result as ResultN, Watcher, recommended_watcher};
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, serde_as};
use std::path::Path;
use std::sync::{Arc, mpsc::channel};
use std::{collections::HashMap, ops::Deref};
use tokio::sync::Mutex;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Hook {
    #[serde(default)]
    pub disable: bool,
    #[serde(flatten)]
    pub variant: HookVariant,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hooks(#[serde_as(as = "OneOrMany<_>")] pub Vec<Hook>);

impl Deref for Hooks {
    type Target = Vec<Hook>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn default_accept() -> String {
    "application/json".to_owned()
}

pub type HookMap = HashMap<String, Hooks>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Login {
    pub endpoint: String,
}

pub const ASSETS_PATH: &str = "manifest";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum LogFormat {
    #[allow(non_camel_case_types)]
    json,
    #[default]
    #[allow(non_camel_case_types)]
    compact,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub format: LogFormat,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub queue: Queue,
    pub hooks: HookMap,
    pub trace: Log,
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
