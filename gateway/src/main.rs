mod libs;
use anyhow::{Ok as Okk, Result, bail};
use axum::{
    Router,
    extract::{Query, State, ws::WebSocketUpgrade},
    http::{Response, StatusCode},
    routing::get,
};
use kafka::split_mq;
use libs::config::{ASSETS_PATH, Config, LogFormat, Settings};
use libs::shared::{Sender, StateChat};
use libs::template::Tmpls;
use libs::websocket::{handle_ws, send_to_ws};
use libs::{admin::*, webhooks::handle_hook};
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

    let (outgo_tx, income_rx) = if !queue.disable {
        split_mq(queue).await
    } else {
        (None, None)
    };

    let Some(rx) = income_rx else {
        bail!("income channel invalid");
    };
    let Some(tx) = outgo_tx else {
        bail!("outgo channel invalid");
    };

    send_to_ws(rx, &shared).await;

    let app = Router::new()
        .route(
            "/channel",
            get(
                |ws: WebSocketUpgrade,
                 Query(q): Query<Map<String, Value>>,
                 State(state): State<StateChat<Sender>>| async move {
                    let s = state.settings.read().await;
                    let login = &s.hooks.get("login").cloned().unwrap()[0];
                    let logout = s.hooks.get("logout").unwrap()[0].clone();
                    drop(s);

                    let Ok(a) = handle_hook(&login, &q, tmpls.clone()).await else {
                        return Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body("UNAUTHORIZED".into())
                            .unwrap();
                    };
                    ws.on_upgrade(async move |socket| {
                        handle_ws(socket, tx, state, settings, tmpls.clone(), &a).await;
                        let _ = handle_hook::<Value>(&logout, &a.into(), tmpls.clone()).await;
                    })
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
    Okk(())
}
