mod libs;
use std::sync::Arc;

use anyhow::Result;
use axum::{
    Router,
    extract::Json,
    routing::{get, post},
};
use libs::config::{Config, LogFormat};
use libs::error::HttpResult;
use libs::postgres::conn;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::{
    EnvFilter, fmt::layer, prelude::__tracing_subscriber_SubscriberExt, registry,
    util::SubscriberInitExt,
};

async fn health() -> HttpResult<Json<Value>> {
    Ok(axum::Json("ok".into()))
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
    let mut client = conn(&cfg.database).await?;
    let state = Arc::new(Mutex::new(client));
    let app = Router::new()
        .route("/health", get(health))
        .with_state(state);

    let addr = "0.0.0.0:3003";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
