use super::config::Login;
use super::shared::{Info, Session};
use super::webhooks::login_post;
use serde_json::{Map, Value};

pub async fn auth(login: &Login, query: &Map<String, Value>) -> Option<(Session, Info)> {
    if let Login::Endpoint { endpoint } = &login {
        let r = login_post(endpoint, query).await.ok()?;
        Some((r.0, r.1))
    } else {
        None
    }
}
