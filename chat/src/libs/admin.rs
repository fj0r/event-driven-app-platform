use super::shared::{Pg, Shared};
use axum::{Json, Router, extract::State, routing::get};

async fn users(State(db): State<Pg>) -> Json<Vec<String>> {
    let db = db.read().await;
    Json(Vec::new())
}

pub fn admin_router() -> Router<Shared> {
    Router::new().route("/users", get(users))
}
