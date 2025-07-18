mod libs;
use anyhow::{Result, bail};
use axum::{Router, extract::Json, routing::get};
use libs::admin::data_router;
use libs::config::{Config, LogFormat, Logic};
use libs::error::HttpResult;
use libs::postgres::connx;
use libs::shared::Shared;
use serde_json::Value;
use tracing::info;
use tracing_subscriber::{
    EnvFilter, fmt::layer, prelude::__tracing_subscriber_SubscriberExt, registry,
    util::SubscriberInitExt,
};

use kafka::{Created, split_mq};
use libs::handler::{ChatMessage, Envelope, handler};
use libs::logic::*;

async fn is_ready() -> HttpResult<Json<Value>> {
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
        split_mq::<ChatMessage<Created>, Envelope<Created>>(queue).await
    } else {
        (None, None)
    };

    let Some(income_rx) = income_rx else {
        bail!("income channel invalid");
    };
    let Some(outgo_tx) = outgo_tx else {
        bail!("outgo channel invalid");
    };

    match cfg.logic {
        Logic::Chat => {
            let _ = handler(outgo_tx, income_rx, shared.clone(), chat).await;
        }
    }

    let app = Router::new()
        .nest("/v1", data_router())
        .route("/is_ready", get(is_ready))
        .with_state(shared);

    let addr = "0.0.0.0:3003";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
