use super::error::HttpResult;
use super::shared::{Pg, Shared};
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use futures::TryStreamExt;
use serde_json::{Value, json};
use sqlx::{Row, query};
use std::ops::Deref;

async fn user(State(db): State<Pg>) -> HttpResult<Json<Vec<Value>>> {
    let db = db.read().await;
    let mut x = query("select * from account").fetch(db.deref());
    let mut v = Vec::new();
    while let Some(r) = x.try_next().await? {
        let n: &str = r.try_get("name")?;
        v.push(json!(n));
    }
    Ok(Json(v)).into()
}

async fn join(State(db): State<Pg>) -> HttpResult<Json<Vec<Value>>> {
    Ok(Json::default()).into()
}

async fn channel(State(db): State<Pg>) -> HttpResult<Json<Vec<Value>>> {
    Ok(Json::default()).into()
}

async fn history(State(db): State<Pg>) -> HttpResult<Json<Vec<Value>>> {
    Ok(Json::default()).into()
}

pub fn data_router() -> Router<Shared> {
    Router::new()
        .route("/join", post(join))
        .route("/channel", post(channel))
        .route("/history", post(history))
        .route("/user", get(user))
}
