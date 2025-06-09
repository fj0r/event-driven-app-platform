use super::settings::{HookVariant, Webhook};
use super::shared::{Info, Session};
use reqwest::Error;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;
use serde_json::{Value, Map};

pub async fn webhook_post<T>(wh: &Webhook, msg: T) -> Result<T, Error>
where
    T: Debug + Serialize + DeserializeOwned,
{
    let client = reqwest::Client::new();
    let r = client.post(&wh.endpoint).json(&msg).send().await?;
    r.json::<T>().await
}

#[derive(thiserror::Error, Debug)]
pub enum GreetError {
    #[error("reqwest error")]
    Reqwest(#[from] Error),
    #[error("not a webhook")]
    NotWebhook,
}

pub async fn greet_post(wh: &HookVariant, msg: &Map<String, Value>) -> Result<String, GreetError> {
    let client = reqwest::Client::new();
    match wh {
        HookVariant::Webhook {
            endpoint,
            accept: _,
        } => {
            let r = client.post(endpoint).json(&msg).send().await?;
            Ok(r.text().await?)
        }
        _ => Err(GreetError::NotWebhook),
    }
}

pub async fn login_post<T: Serialize>(
    url: impl AsRef<str>,
    query: &T,
) -> Result<(Session, Info), GreetError> {
    let client = reqwest::Client::new();
    let r = client.post(url.as_ref()).json(query).send().await?;
    Ok(r.json::<(Session, Info)>().await?)
}
