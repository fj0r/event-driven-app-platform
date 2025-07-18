use super::config::{Hook, Settings};
use super::shared::{Client, Info, StateChat};
use super::template::Tmpls;
use super::webhooks::{handle_hook, webhook_post};
use anyhow::{Ok as Okk, Result};
use axum::extract::ws::WebSocket;
use futures::{sink::SinkExt, stream::StreamExt};
use kafka::Created;
use message::{Event, session::Session};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
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

async fn handle_greet<T>(
    asset: &Hook,
    context: &Map<String, Value>,
    tmpls: Arc<Tmpls<'_>>,
) -> Result<String>
where
    T: Event<Created> + Serialize + From<(Session, Value)>,
{
    let v = handle_hook(asset, context, tmpls).await?;
    let msg: T = (Session::default(), v).into();
    let msg = serde_json::to_string(&msg)?;
    Ok(msg)
}

pub async fn handle_ws<T>(
    socket: WebSocket,
    outgo_tx: UnboundedSender<T>,
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

    let mut s = state.session.write().await;
    s.insert(
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

    if let Some(greet) = setting1.hooks.get("greet") {
        for g in greet.iter() {
            match handle_greet::<T>(g, &context, tmpls.clone()).await {
                Ok(text) => {
                    let _ = sender
                        .send(axum::extract::ws::Message::Text(text.into()))
                        .await;
                }
                Err(e) => {
                    println!("GreetError => {:?}", e)
                }
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
    let hooks = setting1.hooks.clone();
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

            if let Some(ev) = chat_msg.event()
                && hooks.contains_key(ev)
                && let Some(wh) = hooks.get(ev)
            {
                for h in wh {
                    if h.disable {
                        continue;
                    }
                    if let Ok(r) = webhook_post(&h.variant, chat_msg.clone()).await {
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
            } else {
                // send to event MQ
                let _ = outgo_tx.send(chat_msg.clone());
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
    let mut s = state.session.write().await;
    s.remove(&sid);
}

use message::{ChatMessage, Envelope};

pub async fn send_to_ws(
    income_rx: Arc<Mutex<UnboundedReceiver<Envelope<Created>>>>,
    shared: &StateChat<UnboundedSender<ChatMessage<Created>>>,
) {
    let shared = shared.clone();
    tokio::spawn(async move {
        let mut rx = income_rx.lock().await;

        while let Some(x) = rx.recv().await {
            if !x.receiver.is_empty() {
                let s = shared.session.write().await;
                for r in x.receiver {
                    if s.contains_key(&r) {
                        let s = s.get(&r)?;
                        let _ = s.send(x.message.clone());
                    }
                }
            }
        }
        Some(())
    });
}
