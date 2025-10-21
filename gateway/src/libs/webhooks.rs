use super::config::{Hook, HookVariant};
use super::template::Tmpls;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_str};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum HookError {
    #[error("reqwest error")]
    Reqwest(#[from] Error),
    #[error("not a webhook")]
    NotWebhook,
    #[error("render error")]
    Render(#[from] minijinja::Error),
    #[error("deserialize error")]
    JsonError(#[from] serde_json::Error),
    #[error("disabled")]
    Disabled,
}

pub async fn handle_hook<T>(
    hook: &Hook,
    msg: &Map<String, Value>,
    tmpls: Arc<Tmpls<'_>>,
) -> Result<T, HookError>
where
    T: for<'de> Deserialize<'de> + Debug,
{
    if hook.disable {
        return Err(HookError::Disabled);
    }

    match &hook.variant {
        HookVariant::Path { path } => {
            let tmpl = tmpls.get_template(path).unwrap();
            let r = tmpl.render(msg)?;
            let r = from_str::<T>(&r)?;
            Ok(r)
        }
        HookVariant::Webhook {
            endpoint,
            accept: _,
        } => {
            let client = reqwest::Client::new();
            let r = client.post(endpoint).json(&msg).send().await?;
            let r = r.json::<T>().await?;
            Ok(r)
        }
    }
}

pub async fn webhook_post<T>(wh: &HookVariant, msg: T) -> Result<T, HookError>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
{
    if let HookVariant::Webhook {
        endpoint,
        accept: _,
    } = wh
    {
        let client = reqwest::Client::new();
        let r = client.post(endpoint).json(&msg).send().await?;
        Ok(r.json::<T>().await?)
    } else {
        Err(HookError::NotWebhook)
    }
}
