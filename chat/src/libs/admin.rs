use super::db::Account;
use super::error::HttpResult;
use super::shared::{Db, Shared};
use axum::{
    Json, Router,
    extract::{Path, State},
    response::Response,
    routing::{get, post},
};
use content::{Content, Influx, Message, Method};
use layout::{Attrs, Layout, Settings};
use message::session::SessionInfo;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use short_uuid::ShortUuid;
use tracing::info;

async fn users(State(db): State<Db>) -> HttpResult<Json<Vec<Value>>> {
    let db = db.read().await;
    let v = db.list_account().await?;
    let v = v.iter().map(|x| json!(x.name)).collect();
    Ok(Json(v)).into()
}

async fn user(Path(user): Path<String>, State(db): State<Db>) -> HttpResult<Json<Account>> {
    let db = db.read().await;
    let x = db.get_account(&user).await?;
    Ok(Json(x))
}

#[derive(Debug, Deserialize)]
pub struct Join {
    pub channel: Option<String>,
    pub user: String,
}

async fn join(State(_db): State<Db>, Json(join): Json<Join>) -> HttpResult<Json<Value>> {
    Ok(Json::default()).into()
}

async fn history(State(_db): State<Db>, Json(session): Json<SessionInfo>) -> HttpResult<Response> {
    info!("history: {:?}", session);
    let content = Content::Join(Influx {
        event: "chat/history".into(),
        data: Layout {
            kind: "text".into(),
            attrs: Some(Attrs {
                settings: Some(Settings::Text {
                    format: "md".into(),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        method: Method::Concat,
    });
    let msg: Message = ("chat".into(), content).into();
    let r = serde_json::to_string(&msg)?;
    dbg!(&r.to_string());
    Ok(Response::new(r.into()))
}

async fn channel(State(_db): State<Db>) -> HttpResult<Json<Value>> {
    Ok(Json::default()).into()
}

pub type Info = Map<String, Value>;

async fn login(
    State(db): State<Db>,
    Json(mut payload): Json<Map<String, Value>>,
) -> HttpResult<Json<SessionInfo>> {
    let uuid = ShortUuid::generate().to_string();
    let _db = db.read().await;

    payload.insert("username".into(), uuid[..6].into());
    info!("login: {:?}", payload);
    Ok(Json(SessionInfo {
        id: uuid.as_str().into(),
        info: payload,
    }))
}

async fn logout(
    State(_db): State<Db>,
    Json(payload): Json<Map<String, Value>>,
) -> HttpResult<Json<SessionInfo>> {
    info!("logout: {:?}", payload);
    Ok(Json(SessionInfo {
        id: "".into(),
        info: payload,
    }))
}

pub fn data_router() -> Router<Shared> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/channel", post(channel))
        .route("/join", post(join))
        .route("/history", post(history))
        .route("/users", get(users))
        .route("/user/{user}", get(user))
}
