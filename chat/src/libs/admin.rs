use super::config::ASSETS_PATH;
use super::db::{Account, Channel, CreateChan, JoinChan};
use super::error::HttpResult;
use super::shared::{Db, Shared};
use async_fs::read_to_string;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use content::{Content, Influx, Message, Method};
use layout::{Attrs, Layout, Settings};
use message::session::SessionInfo;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use short_uuid::ShortUuid;
use std::path::Path as OsPath;
use tracing::info;

#[derive(Deserialize)]
pub struct Opts {
    pub layout: Option<bool>,
}

async fn users(_opts: Query<Opts>, State(db): State<Db>) -> HttpResult<Json<Vec<Value>>> {
    let db = db.read().await;
    let v = db.list_account().await?;
    let v = v.iter().map(|x| json!(x.name)).collect();
    Ok(Json(v)).into()
}

async fn user(
    _opts: Query<Opts>,
    Path(user): Path<String>,
    State(db): State<Db>,
) -> HttpResult<Json<Account>> {
    let db = db.read().await;
    let x = db.get_account(&user).await?;
    Ok(Json(x))
}

async fn create_chan(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(create): Json<CreateChan>,
) -> HttpResult<Json<Channel>> {
    let db = db.read().await;
    let chan = db.create_channel(&create).await?;
    Ok(Json(chan))
}

async fn join_chan(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(join): Json<JoinChan>,
) -> HttpResult<Json<Value>> {
    let db = db.read().await;
    db.join_channel(&join).await?;
    Ok(Json::default()).into()
}

async fn channel(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(session): Json<SessionInfo>,
) -> HttpResult<Json<Vec<Channel>>> {
    let db = db.read().await;
    let channel = db.list_channel((&session.id).into()).await?;
    Ok(Json(channel))
}

async fn history(
    opts: Query<Opts>,
    State(db): State<Db>,
    Json(session): Json<SessionInfo>,
) -> HttpResult<Json<Value>> {
    let _db = db.read().await;
    info!("history: {:?}", session);
    if let Some(layout) = opts.layout
        && layout
    {
        let content = Content::Join(Influx {
            event: "chat/history".into(),
            channel: None,
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
        //let msg: Message = ("chat".into(), content).into();
        let r = serde_json::to_value(&content)?;
        Ok(Json(r.into()))
    } else {
        Ok(Json(Value::Array(Vec::new())))
    }
}

async fn yaml(opts: Query<Opts>, Path(name): Path<String>) -> HttpResult<Json<serde_yaml::Value>> {
    let path = OsPath::new(ASSETS_PATH);
    let content = read_to_string(path.join(&name)).await?;
    let v: serde_yaml::Value = serde_yaml::from_str(&content)?;
    Ok(Json(v))
}

async fn login(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(mut payload): Json<Map<String, Value>>,
) -> HttpResult<Json<SessionInfo>> {
    let uuid = ShortUuid::generate().to_string();
    let token = payload.get("token").and_then(|x| x.as_str());
    let db = db.read().await;
    let (id, name) = db.login(&uuid, token).await?;
    info!("login {}: {}", id, name);
    payload.insert("username".into(), name.into());
    Ok(Json(SessionInfo {
        id: uuid.as_str().into(),
        info: payload,
    }))
}

async fn logout(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(session): Json<SessionInfo>,
) -> HttpResult<Json<SessionInfo>> {
    let db = db.read().await;
    db.logout(&session.id).await?;
    info!("logout: {}", session.id);
    Ok(Json(session))
}

pub fn data_router() -> Router<Shared> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/yaml/{name}", post(yaml))
        .route("/channel", post(channel))
        .route("/channel/join", post(join_chan))
        .route("/channel/create", post(create_chan))
        .route("/history", post(history))
        .route("/users", get(users))
        .route("/user/{user}", get(user))
}
