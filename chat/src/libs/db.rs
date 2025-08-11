use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{
    Error, FromRow, Pool, Postgres, query_as,
    types::JsonValue,
    types::chrono::{DateTime, NaiveDateTime, Utc},
};
use std::ops::Deref;

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Model(pub Executor);

impl Deref for Model {
    type Target = Executor;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub email: String,
    pub x: Option<JsonValue>,
}

impl Model {
    pub async fn get_account(&self, name: &str) -> Result<Account> {
        query_as("select * from account where name = $1")
            .bind(name)
            .fetch_one(self.deref())
            .await
    }
    pub async fn list_account(&self) -> Result<Vec<Account>> {
        let mut x = query_as("select * from account").fetch(self.deref());
        let mut v = Vec::new();
        while let Some(r) = x.try_next().await? {
            v.push(r)
        }
        Ok(v)
    }
}
