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
use layout::{Attrs, Bind, BindVariant, Layout, Settings};
use maplit::hashmap;
use message::session::SessionInfo;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use short_uuid::ShortUuid;
use std::borrow::Cow;
use std::path::Path as OsPath;
use tracing::info;

#[derive(Deserialize)]
pub struct Opts {
    pub layout: Option<bool>,
}

async fn users(_opts: Query<Opts>, State(db): State<Db>) -> HttpResult<Json<Vec<Value>>> {
    let v = db.list_account().await?;
    let v = v.iter().map(|x| json!(x.name)).collect();
    Ok(Json(v)).into()
}

async fn user(
    _opts: Query<Opts>,
    Path(user): Path<String>,
    State(db): State<Db>,
) -> HttpResult<Json<Account>> {
    let x = db.get_account(&user).await?;
    Ok(Json(x))
}

async fn create_chan(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(create): Json<CreateChan>,
) -> HttpResult<Json<Channel>> {
    let chan = db.create_channel(&create).await?;
    Ok(Json(chan))
}

async fn join_chan(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(join): Json<JoinChan>,
) -> HttpResult<Json<Value>> {
    db.join_channel(&join).await?;
    Ok(Json::default()).into()
}

async fn channel(
    opts: Query<Opts>,
    State(db): State<Db>,
    Json(session): Json<SessionInfo>,
) -> HttpResult<Json<Value>> {
    let channel = db.list_channel((&session.id).into()).await?;
    if let Some(layout) = opts.layout
        && layout
    {
        let content: Vec<_> = vec!["1", "2", "3"]
            .iter()
            .map(|x| {
                Content::Join(Influx {
                    event: "channel".into(),
                    channel: None,
                    method: Method::Replace,
                    data: Layout {
                        kind: "text".into(),
                        bind: Some(hashmap! {
                            "value".to_owned() => Bind {
                                variant: BindVariant::Default {  },
                                default: Some(serde_json::to_value(x).unwrap()),
                                ..Default::default()
                            }
                        }),
                        ..Default::default()
                    },
                })
            })
            .collect();
        Ok(Json(serde_json::to_value(content)?))
    } else {
        Ok(Json(serde_json::to_value(channel)?))
    }
}

async fn history(
    opts: Query<Opts>,
    State(db): State<Db>,
    Json(session): Json<SessionInfo>,
) -> HttpResult<Json<Value>> {
    info!(">> history: {:?}", &session);
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

#[derive(Debug, Deserialize)]
pub struct LoginOpts {
    pub key: Option<String>,
}

async fn login(
    opts: Query<LoginOpts>,
    State(db): State<Db>,
    Json(mut payload): Json<Map<String, Value>>,
) -> HttpResult<Json<SessionInfo>> {
    let tk = match &opts.key {
        Some(key) => key,
        None => "token",
    };
    let token = if let Some(token) = payload.get(tk) {
        Cow::Borrowed(token.as_str().unwrap_or(""))
    } else {
        Cow::Owned(ShortUuid::generate().to_string())
    };
    let (id, name) = db.login(&token).await?;
    info!("login {}: {}\n  token: {:?}", id, name, &token);
    payload.insert("username".into(), name.into());
    Ok(Json(SessionInfo {
        id: id.as_str().into(),
        info: payload,
    }))
}

async fn logout(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(session): Json<SessionInfo>,
) -> HttpResult<Json<SessionInfo>> {
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
