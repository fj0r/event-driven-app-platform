use super::error::HttpResult;
use super::shared::{Pg, Shared};
use axum::{Json, Router, extract::State, routing::get};
use futures::TryStreamExt;
use sqlx::{Row, query};
use std::ops::Deref;

async fn users(State(db): State<Pg>) -> HttpResult<Json<Vec<String>>> {
    let db = db.read().await;
    let mut x = query("select * from account").fetch(db.deref());
    let mut v = Vec::new();
    while let Some(r) = x.try_next().await? {
        let n: &str = r.try_get("name")?;
        v.push(n.to_string());
    }
    Ok(Json(v)).into()
}

pub fn admin_router() -> Router<Shared> {
    Router::new().route("/users", get(users))
}
