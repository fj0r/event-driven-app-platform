use super::config::{HookVariant, Hooks, Login, Settings};
use super::shared::{Client, Info, Session, StateChat};
use super::template::Tmpls;
use super::webhooks::{greet_post, webhook_post};
use anyhow::Result;
use anyhow::{Context, Ok as Okk};
use axum::extract::ws::WebSocket;
use futures::{sink::SinkExt, stream::StreamExt};
use kafka::Created;
use message::Event;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_str, from_value};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{Mutex, RwLock};

/* TODO:
use std::async_iter;

struct GreetIter<'a> {
    index: usize,
    greet: &'a AssetsList,
    context: &'a minijinja::Value,
}

impl<'a> AsyncIterator for GreetIter<'a> {
}
*/

async fn handle_greet<T: Debug>(
    asset: &Hooks,
    context: &Map<String, Value>,
    tmpls: Arc<Tmpls<'_>>,
) -> Result<String>
where
    T: Event<Created> + Serialize + From<(Session, Value)>,
{
    if !asset.enable {
        return Ok("disabled".into());
    }
    let content = match &asset.variant {
        HookVariant::Path { path } => {
            let tmpl = tmpls.get_template(path).unwrap();
            tmpl.render(context).ok()
        }
        wh @ HookVariant::Webhook { .. } => greet_post(wh, context).await.ok(),
    };
    let v = from_str(&content.context("render failed")?)?;
    let msg: T = (Session::default(), v).into();
    let msg = serde_json::to_string(&msg)?;
    Ok(msg)
}

pub async fn handle_ws<T>(
    socket: WebSocket,
    event_tx: Option<UnboundedSender<T>>,
    state: StateChat<UnboundedSender<T>>,
    settings: Arc<RwLock<Settings>>,
    tmpls: Arc<Tmpls<'static>>,
    (sid, info): (Session, Info),
) where
    T: Event<Created>
        + for<'a> Deserialize<'a>
        + Serialize
        + From<(Session, Value)>
        + Clone
        + Debug
        + Send
        + 'static,
{
    let setting1 = settings.read().await;
    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<T>();

    let mut s = state.write().await;
    s.session.insert(
        sid.clone(),
        Client {
            sender: tx.clone(),
            info: info.clone(),
        },
    );
    drop(s);

    tracing::info!("Connection opened for {}", &sid);

    let mut context = Map::new();
    context.insert("session_id".into(), sid.clone().into());
    context.insert("info".into(), Value::Object(info));

    for g in setting1.greet.iter() {
        match handle_greet::<T>(g, &context, tmpls.clone()).await {
            Ok(text) => {
                let _ = sender
                    .send(axum::extract::ws::Message::Text(text.into()))
                    .await;
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let text = serde_json::to_string(&msg)?;
            // to ws client
            if sender
                .send(axum::extract::ws::Message::Text(text.into()))
                .await
                .is_err()
            {
                break;
            }
        }
        Okk(())
    });

    let sid_cloned = sid.clone();
    let webhooks = setting1.webhooks.clone();
    drop(setting1); // release lock
    let mut recv_task = tokio::spawn(async move {
        // update sid after the message queue login event
        // UnboundedSender does not implement `Eq` and `Hash`
        // Therefore, it cannot be used to look up sid in reverse.
        // If another reverse lookup `r_key` is added, then `r_key` is immutable, while sid is mutable
        // It would be better if sid were immutable, and the name was placed in `info`
        #[allow(unused_mut)]
        let mut sid = sid_cloned;

        while let Some(Ok(msg)) = receiver.next().await {
            // text protocol of ws
            let text = msg.to_text()?;
            let value = serde_json::from_str(text)?;
            let chat_msg: T = (sid.clone(), value).into();

            let mut is_webhook: bool = false;
            if let Some(ev) = chat_msg.event() {
                if webhooks.contains_key(ev) {
                    if let Some(wh) = webhooks.get(ev) {
                        if wh.enable {
                            is_webhook = true;
                            if let Ok(r) = webhook_post(wh, chat_msg.clone()).await {
                                let _ = tx.send(r);
                            } else {
                                context.insert("event".into(), ev.into());
                                let t = tmpls
                                    .get_template("webhook_error.json")
                                    .unwrap()
                                    .render(&context)
                                    .unwrap();
                                let _ = tx.send(serde_json::from_str(&t)?);
                            }
                        }
                    }
                }
            }

            // send to event MQ
            if !is_webhook {
                if let Some(ref m) = event_tx {
                    let _ = m.send(chat_msg.clone());
                }
            }

            tracing::debug!("[ws] {:?}", &chat_msg);
        }
        Okk(())
    });

    tokio::select! {
        _ = &mut recv_task => recv_task.abort(),
        _ = &mut send_task => send_task.abort(),
    };

    tracing::info!("Connection closed for {}", &sid);
    let mut s = state.write().await;
    s.session.remove(&sid);
}

use super::message::{ChatMessage, Envelope};

pub async fn send_to_ws(
    mqrx: Arc<Mutex<UnboundedReceiver<Envelope>>>,
    shared: &StateChat<UnboundedSender<ChatMessage>>,
) {
    let shared = shared.clone();
    tokio::spawn(async move {
        let mut rx = mqrx.lock().await;

        while let Some(x) = rx.recv().await {
            if !x.receiver.is_empty() {
                let mut s = shared.write().await;
                for r in x.receiver {
                    if s.session.contains_key(&r) {
                        let mut is_login = false;
                        let e = x.message.event();
                        let l = s.settings.clone();
                        let l = l.read().await;
                        if let Login::Event { event } = &l.login {
                            if let Some(e) = e {
                                if event == e {
                                    if let Some(info) = x
                                        .message
                                        .content
                                        .as_object()
                                        .and_then(|x| x.get("data"))
                                        .and_then(|x| {
                                            from_value::<Map<String, Value>>(x.to_owned()).ok()
                                        })
                                    {
                                        s.session.entry(r.clone()).and_modify(|x| x.info = info);
                                    }
                                    is_login = true;
                                };
                            }
                        };
                        if !is_login {
                            let s = s.session.get(&r)?;
                            let _ = s.send(x.message.clone());
                        }
                    }
                }
            }
        }
        Some(())
    });
}
