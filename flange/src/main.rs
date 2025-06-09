use axum::{
    Router,
    extract::{Query, State, ws::WebSocketUpgrade},
    routing::get,
};
use libs::message::{Envelope, MessageQueueEvent, MessageQueuePush};
use serde_json::{Map, Value};
use tower_http::services::ServeDir;
use tracing::info;
mod libs;
use anyhow::{Ok, Result};
use libs::admin::*;
use libs::kafka::{KafkaManagerEvent, KafkaManagerPush};
use libs::settings::{Config, Settings};
use libs::shared::{Sender, StateChat};
use libs::websocket::{handle_ws, send_to_ws};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<()> {
    // console_subscriber::init();
    tracing_subscriber::fmt::init();

    #[allow(unused_mut)]
    let mut config = Config::new()?;
    // config.listen().await.unwrap();
    dbg!(&config.data);

    let settings = Arc::new(RwLock::new(Settings::new()?));

    //dbg!(&settings);

    let shared = StateChat::<Sender>::new(settings.clone());

    let queue = settings.read().await.queue.clone();

    let event_tx = if queue.enable {
        let push_mq: KafkaManagerPush<Envelope> = match queue.push.kind.as_str() {
            "kafka" => {
                let mut push_mq = KafkaManagerPush::new(queue.push);
                push_mq.run().await;
                push_mq
            }
            _ => unreachable!(),
        };
        let shared = shared.clone();
        let Some(mqrx) = push_mq.get_rx() else {
            unreachable!()
        };
        send_to_ws(mqrx, &shared).await;

        match queue.event.kind.as_str() {
            "kafka" => {
                let mut event_mq = KafkaManagerEvent::new(queue.event);
                event_mq.run().await;
                event_mq.get_tx()
            }
            _ => unreachable!(),
        }
    } else {
        None
    };

    let app = Router::new()
        .route(
            "/channel",
            get(
                |ws: WebSocketUpgrade,
                 Query(q): Query<Map<String, Value>>,
                 State(state): State<StateChat<Sender>>| async move {
                    ws.on_upgrade(|socket| handle_ws(socket, event_tx, state, settings, q))
                },
            ),
        )
        .nest("/admin", admin_router())
        .nest("/config", config_router())
        .nest("/debug", debug_router())
        .fallback_service(ServeDir::new("./static"))
        .with_state(shared);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
