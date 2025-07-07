use axum::extract::FromRef;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Pg = Arc<RwLock<Pool<Postgres>>>;

#[derive(Debug, Clone)]
pub struct Shared {
    pub db: Pg,
}

impl FromRef<Shared> for Pg {
    fn from_ref(input: &Shared) -> Self {
        input.db.clone()
    }
}

impl Shared {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self {
            db: Arc::new(RwLock::new(db)),
        }
    }
}
