//! ```cargo
//! [dependencies]
//! tokio = { version = "1.45.1", features = ["full"] }
//! axum = "0.8.4"
//! sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }
//! uuid = { version = "1.0", features = ["v4", "serde"] }
//! chrono = { version = "0.4", features = ["serde"] }
//! dotenvy = "0.15"
//! serde_json = "1.0.140"
//! ```

use axum::{extract::State, Json, routing::get, Router};
use serde_json::{json, Value};
use std::env;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool, // 数据库连接池
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        // 创建数据库连接池
        let db = PgPool::connect(database_url).await?;
        Ok(Self { db })
    }
}

pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    // 测试数据库连接
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => Json(json!({
            "status": "ok",
            "database": "connected"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Json(json!({
                "status": "error",
                "database": "disconnected",
                "error": e.to_string()
            }))
        }
    }
}

#[tokio::main]
async fn main() {
    // 从 .env 文件加载环境变量
    dotenvy::dotenv().ok();

    // 读取数据库 URL
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file or environment");

    // 创建共享状态
    let app_state = AppState::new(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database successfully!");

    // 设置路由
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(app_state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
