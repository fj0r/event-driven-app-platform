use super::config::Login;
use super::shared::Info;
use super::webhooks::login_post;
use message::session::Session;
use serde_json::{Map, Value};

pub async fn auth(login: &Login, query: &Map<String, Value>) -> Option<(Session, Info)> {
    let r = login_post(&login.endpoint, query).await.ok()?;
    Some((r.0, r.1))
}
