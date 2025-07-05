mod libs;
use std::{error::Error, sync::Arc};

use anyhow::Result;
use axum::{Router, extract::Json, routing::get};
use libs::admin::admin_router;
use libs::config::{Config, LogFormat};
use libs::error::HttpResult;
use libs::postgres::connx;
use libs::shared::Shared;
use serde_json::Value;
use std::fmt::Debug;
use tracing::info;
use tracing_subscriber::{
    EnvFilter, fmt::layer, prelude::__tracing_subscriber_SubscriberExt, registry,
    util::SubscriberInitExt,
};

use kafka::{Created, split_mq};
use libs::logic::{ChatMessage, Envelope, Sender, aShared, logic};

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

    let client = connx(&cfg.database).await?;
    let shared = Shared::new(client);

    let queue = cfg.queue;

    let (outgo_tx, income_rx) = if queue.enable {
        split_mq(queue).await
    } else {
        (None, None)
    };

    async fn x<T: Debug>(e: ChatMessage<T>, s: aShared, x: Sender<T>) {
        println!("{:?}", e);
    }
    logic(outgo_tx, income_rx, shared.clone(), x);

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
