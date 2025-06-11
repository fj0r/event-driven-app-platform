use super::config::{Login, LoginVariant};
use super::shared::{Info, Session};
use super::webhooks::login_post;
use serde_json::{Map, Value};

pub async fn auth(login: &Login, query: &Map<String, Value>) -> Option<(Session, Info)> {
    if login.enable {
        if let Some(LoginVariant::Endpoint { endpoint }) = &login.variant {
            let r = login_post(endpoint, query).await.ok()?;
            return Some((r.0, r.1));
        };
    }
    None
}
