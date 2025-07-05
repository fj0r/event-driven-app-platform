mod libs;
use std::{error::Error, sync::Arc};

use anyhow::Result;
use axum::{
    Router,
    extract::Json,
    routing::{get, post},
};
use libs::admin::admin_router;
use libs::config::{Config, LogFormat};
use libs::error::HttpResult;
use libs::postgres::connx;
use libs::shared::Shared;
use serde_json::Value;
use tracing::info;
use tracing_subscriber::{
    EnvFilter, fmt::layer, prelude::__tracing_subscriber_SubscriberExt, registry,
    util::SubscriberInitExt,
};

use kafka::{Created, KafkaManagerEvent, KafkaManagerPush};
use message::Envelope;

async fn health() -> HttpResult<Json<Value>> {
    Ok(axum::Json("ok".into())).into()
}


#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::new()?;

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    match &cfg.trace.format {
        LogFormat::compact => {
            registry().with(layer().compact()).with(filter).init();
        }
        LogFormat::json => {
            registry().with(layer().json()).with(filter).init();
        }
    };

    dbg!(&cfg);

    let queue = cfg.queue;

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

    let client = connx(&cfg.database).await?;
    let shared = Shared::new(client);
    let app = Router::new()
        .nest("/admin", admin_router())
        .route("/health", get(health))
        .with_state(shared);

    let addr = "0.0.0.0:3003";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
