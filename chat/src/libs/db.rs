use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{
    Error, FromRow, Pool, Postgres, Row, query, query_as,
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
    pub async fn login(&self, session_id: &str, _token: Option<&str>) -> Result<(String, String)> {
        let mut s = query(
            "
            with a as (
                select a.id, a.name from account as a
                left outer join session as s on a.id = s.account_id
                where s.id is null
                limit 1
            )
            , s as (
                insert into session (id, account_id)
                select $1, id from a returning id
            )
            select a.name, s.id from a, s;
            ",
        )
        .bind(session_id)
        .fetch(self.deref());
        if let Some(r) = s.try_next().await? {
            let name: &str = r.try_get("name")?;
            let id: &str = r.try_get("id")?;
            Ok((id.to_string(), name.to_string()))
        } else {
            Err(Error::RowNotFound)
        }
    }
    pub async fn logout(&self, session_id: &str) -> Result<()> {
        query(
            "
            delete from session where id = $1
            ",
        )
        .bind(session_id)
        .execute(self.deref())
        .await?;
        Ok(())
    }
}
