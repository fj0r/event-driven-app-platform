use anyhow::Result;
mod libs;
use axum::{
    Router,
    extract::Json,
    routing::{get, post},
};
use libs::error::HttpResult;
use serde_json::Value;
use tracing::info;
use libs::config::Config;

async fn health() -> HttpResult<Json<Value>> {
    Ok(axum::Json("ok".into()))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cfg = Config::new();
    dbg!(&cfg);
    let app = Router::new().route("/health", get(health));

    let addr = "0.0.0.0:3003";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
