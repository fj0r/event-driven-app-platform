mod libs;
use anyhow::{Ok, Result};
use axum::{
    Router,
    extract::{Query, State, ws::WebSocketUpgrade},
    http::Response,
    routing::get,
};
use kafka::{Created, KafkaManagerIncome, KafkaManagerOutgo};
use libs::admin::*;
use libs::auth::auth;
use libs::config::{ASSETS_PATH, Config, LogFormat, Settings};
use libs::shared::{Sender, StateChat};
use libs::template::Tmpls;
use libs::websocket::{handle_ws, send_to_ws};
use message::{
    Envelope,
    queue::{MessageQueueIncome, MessageQueueOutgo},
};
use serde_json::{Map, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{
    EnvFilter, fmt::layer, prelude::__tracing_subscriber_SubscriberExt, registry,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<()> {
    #[allow(unused_mut)]
    let mut config = Config::new()?;
    // config.listen().await.unwrap();
    dbg!(&config.data);

    let settings = Settings::new()?;
    // console_subscriber::init();
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    match &settings.trace.format {
        LogFormat::compact => {
            registry().with(layer().compact()).with(filter).init();
        }
        LogFormat::json => {
            registry().with(layer().json()).with(filter).init();
        }
    };

    let settings = Arc::new(RwLock::new(settings));
    //dbg!(&settings);
    let tmpls: Arc<Tmpls<'static>> = Arc::new(Tmpls::new(ASSETS_PATH).unwrap());

    let shared = StateChat::<Sender>::new(settings.clone());

    let queue = settings.read().await.queue.clone();

    let outgo_tx = if queue.enable {
        let income_mq: KafkaManagerIncome<Envelope<Created>> = match queue.income.kind.as_str() {
            "kafka" => {
                let mut mq = KafkaManagerIncome::new(queue.income);
                mq.run().await;
                mq
            }
            _ => unreachable!(),
        };
        let shared = shared.clone();
        let Some(income_rx) = income_mq.get_rx() else {
            unreachable!()
        };
        send_to_ws(income_rx, &shared).await;

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

    let app = Router::new()
        .route(
            "/channel",
            get(
                |ws: WebSocketUpgrade,
                 Query(q): Query<Map<String, Value>>,
                 State(state): State<StateChat<Sender>>| async move {
                    let s = state.settings.read().await;
                    let login = s.login.clone();
                    let logout = s.logout.clone();
                    drop(s);
                    let Some(a) = auth(&login, &q).await else {
                        return Response::new("UNAUTHORIZED".into());
                    };
                    let r = ws.on_upgrade(|socket| {
                        handle_ws(socket, outgo_tx, state, settings, tmpls, a)
                    });
                    auth(&logout, &q).await;
                    r
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
