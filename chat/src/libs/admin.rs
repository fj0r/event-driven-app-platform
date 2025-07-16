use super::error::HttpResult;
use super::shared::{Pg, Shared};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{
    FromRow, Row, query, query_as,
    types::JsonValue,
    types::chrono::{DateTime, NaiveDateTime, Utc},
};
use std::ops::Deref;

async fn users(State(db): State<Pg>) -> HttpResult<Json<Vec<Value>>> {
    let db = db.read().await;
    let mut x = query("select * from account").fetch(db.deref());
    let mut v = Vec::new();
    while let Some(r) = x.try_next().await? {
        let n: &str = r.try_get("name")?;
        v.push(json!(n));
    }
    Ok(Json(v)).into()
}

#[derive(Debug, Serialize, FromRow)]
struct Account {
    id: i32,
    name: String,
    created: NaiveDateTime,
    updated: NaiveDateTime,
    email: String,
    x: Option<JsonValue>,
}

async fn user(Path(user): Path<String>, State(db): State<Pg>) -> HttpResult<Json<Account>> {
    let db = db.read().await;
    let x: Account = query_as("select * from account where name = $1")
        .bind(&user)
        .fetch_one(db.deref())
        .await?;
    Ok(Json(x))
}

#[derive(Debug, Deserialize)]
pub struct Join {
    pub channel: Option<String>,
    pub user: String,
}

async fn join(State(db): State<Pg>, Json(join): Json<Join>) -> HttpResult<Json<Value>> {
    Ok(Json::default()).into()
}

async fn history(State(db): State<Pg>, Json(join): Json<Join>) -> HttpResult<Json<Vec<Value>>> {
    Ok(Json::default()).into()
}

async fn channel(State(db): State<Pg>) -> HttpResult<Json<Value>> {
    Ok(Json::default()).into()
}

pub fn data_router() -> Router<Shared> {
    Router::new()
        .route("/channel", post(channel))
        .route("/join", post(join))
        .route("/history", post(history))
        .route("/users", get(users))
        .route("/user/{user}", get(user))
}
