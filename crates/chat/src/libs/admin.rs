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
use brick::{Bind, BindVariant, Brick, Text, TextAttr};
use content::{Content, Influx, Method};
use maplit::hashmap;
use message::session::SessionInfo;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use short_uuid::ShortUuid;
use std::borrow::Cow;
use std::path::Path as OsPath;
use tracing::info;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Opts {
    pub layout: Option<bool>,
}

async fn users(_opts: Query<Opts>, State(db): State<Db>) -> HttpResult<Json<Vec<Value>>> {
    let v = db.list_account().await?;
    let v = v.iter().map(|x| json!(x.name)).collect();
    Ok(Json(v))
}

async fn user(
    _opts: Query<Opts>,
    Path(user): Path<String>,
    State(db): State<Db>,
) -> HttpResult<Json<Account>> {
    let x = db.get_account(&user).await?;
    Ok(Json(x))
}

async fn select_chan(
    opts: Query<Opts>,
    State(db): State<Db>,
    Json(select): Json<Value>,
) -> HttpResult<Json<Value>> {
    let chan = Channel::default();
    info!("select_chan {:?}", &select);
    if let Some(layout) = opts.layout
        && layout
    {
        let content: Vec<_> = ["1", "4", "5", "6"]
            .iter()
            .map(|x| {
                Content::Join(Influx {
                    event: "channel::list".into(),
                    channel: None,
                    method: Method::Replace,
                    data: Brick::text(Text {
                        id: Some(x.to_string()),
                        attrs: Some(TextAttr {
                            class: Some(vec!["box".to_string()]),
                            ..Default::default()
                        }),
                        bind: Some(hashmap! {
                            "value".to_owned() => Bind {
                                variant: BindVariant::Default {  },
                                default: Some(serde_json::to_value(x).unwrap()),
                                ..Default::default()
                            }
                        }),
                    }),
                })
            })
            .collect();
        Ok(Json(serde_json::to_value(content)?))
    } else {
        Ok(Json(serde_json::to_value(chan)?))
    }
}

async fn join_chan(
    _opts: Query<Opts>,
    State(db): State<Db>,
    Json(join): Json<JoinChan>,
) -> HttpResult<Json<Value>> {
    db.join_channel(&join).await?;
    Ok(Json::default())
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
        let content: Vec<_> = ["1", "2", "3"]
            .iter()
            .map(|x| {
                Content::Join(Influx {
                    event: "channel::list".into(),
                    channel: None,
                    method: Method::Replace,
                    data: Brick::text(Text {
                        id: Some(x.to_string()),
                        attrs: Some(TextAttr {
                            class: Some(vec!["box".to_string()]),
                            ..Default::default()
                        }),
                        bind: Some(hashmap! {
                            "value".to_owned() => Bind {
                                variant: BindVariant::Default {  },
                                default: Some(serde_json::to_value(x).unwrap()),
                                ..Default::default()
                            }
                        }),
                    }),
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
            event: "chat-history".into(),
            channel: None,
            data: Brick::text(Text {
                attrs: Some(TextAttr {
                    format: Some("md".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            method: Method::Concat,
        });
        //let msg: Message = ("chat".into(), content).into();
        let r = serde_json::to_value(&content)?;
        Ok(Json(r))
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
    info!(
        "login {}: {}\ntoken : {:?}\n{:?}",
        id, name, &token, payload
    );
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
        .route("/channel/select", post(select_chan))
        .route("/channel/join", post(join_chan))
        .route("/history", post(history))
        .route("/users", get(users))
        .route("/user/{user}", get(user))
}
