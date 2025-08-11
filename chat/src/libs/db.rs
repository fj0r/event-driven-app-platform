use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{
    Error, FromRow, Pool, Postgres, query_as,
    types::JsonValue,
    types::chrono::{DateTime, NaiveDateTime, Utc},
};

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, FromRow)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub email: String,
    pub x: Option<JsonValue>,
}

impl Account {
    pub async fn get(name: &str, db: &Executor) -> Result<Self> {
        query_as("select * from account where name = $1")
            .bind(name)
            .fetch_one(db)
            .await
    }

    pub async fn list(db: &Executor) -> Result<Vec<Self>> {
        let mut x = query_as("select * from account").fetch(db);
        let mut v = Vec::new();
        while let Some(r) = x.try_next().await? {
            v.push(r)
        }
        Ok(v)
    }
}
