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

use kafka::{Created, KafkaManagerOutgo, KafkaManagerIncome};
use message::{
    Envelope,
    queue::{MessageQueueOutgo, MessageQueueIncome},
};

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
        let income_mq: KafkaManagerIncome<Envelope<Created>> = match queue.income.kind.as_str() {
            "kafka" => {
                let mut imcome_mq = KafkaManagerIncome::new(queue.income);
                imcome_mq.run().await;
                imcome_mq
            }
            _ => unreachable!(),
        };
        let Some(mqrx) = income_mq.get_rx() else {
            unreachable!()
        };

        match queue.outgo.kind.as_str() {
            "kafka" => {
                let mut outgo_mq = KafkaManagerOutgo::new(queue.outgo);
                outgo_mq.run().await;
                outgo_mq.get_tx()
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
