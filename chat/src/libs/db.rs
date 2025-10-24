use futures::TryStreamExt;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use sqlx::{
    Error, FromRow, Pool, Postgres, Row, query, query_as,
    types::JsonValue,
    types::chrono::{DateTime, NaiveDateTime, Utc},
};
use std::ops::Deref;
use tracing::error;

type Executor = Pool<Postgres>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
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

    pub async fn login(&self, token: &str) -> Result<(String, String)> {
        let mut s = query(indoc! {
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
            "
        })
        .bind(token)
        .fetch(self.deref());
        if let Some(r) = s.try_next().await? {
            let name: &str = r.try_get("name")?;
            let id: &str = r.try_get("id")?;
            Ok((id.to_string(), name.to_string()))
        } else {
            error!("insert session failed: {}", token);
            Err(Error::RowNotFound)
        }
    }

    pub async fn logout(&self, session_id: &str) -> Result<()> {
        query(indoc! {
            "
            delete from session where id = $1
            "
        })
        .bind(session_id)
        .execute(self.deref())
        .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Default, FromRow)]
pub struct Channel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateChan {
    pub session: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JoinChan {
    pub session: String,
    pub channel: String,
    pub account: Vec<String>,
}

impl Model {
    pub async fn list_channel(&self, session_id: &str) -> Result<Vec<Channel>> {
        query_as(indoc! {
            "
            select c.id, c.name from channel as c
            join channel_account as ca on ca.channel_id = c.id
            join session as s on ca.account_id = s.account_id
            where s.id = $1
            "
        })
        .bind(session_id)
        .fetch_all(self.deref())
        .await
    }

    pub async fn create_channel(&self, arg: &CreateChan) -> Result<Channel> {
        query_as(indoc! {
            "
            with x as (
                insert into channel(name) values($1) returning id, name
            )
            , a as (
                select account_id from session where id = $2
            )
            , r as (
                insert into channel_account(channel_id, account_id, owner)
                select x.id, a.account_id, true from x, a
            )
            select * from x
            "
        })
        .bind(&arg.name)
        .bind(&arg.session)
        .fetch_one(self.deref())
        .await
    }

    pub async fn join_channel(&self, arg: &JoinChan) -> Result<()> {
        // Only the owner can add others to the channel
        query(indoc! {
            "
            with c as (
                select c.id, ca.owner from session as s
                join channel_account as ca
                on s.account_id = ca.account_id
                join channel as c
                on ca.channel_id = c.id
                where s.id = $1 and c.name = $2
            )
            , a as (
                select id from account where name = any($3)
            )
            insert into channel_account(channel_id, account_id)
            select c.id, a.id from c, a where c.owner
            on conflict(channel_id, account_id) do nothing
            "
        })
        .bind(&arg.session)
        .bind(&arg.channel)
        .bind(&arg.account)
        .execute(self.deref())
        .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct ActiveUser {
    pub id: i32,
    pub name: String,
    pub session_id: String,
}

impl Model {
    pub async fn list_channel_account(&self, channel_id: i32) -> Result<Vec<ActiveUser>> {
        query_as(indoc! {
            "
            select a.id, a.name, s.id as session_id
            from channel_account as ca
            join account as a on ca.account_id = a.id
            join session as s on s.account_id = a.id
            where ca.channel_id = $1
            "
        })
        .bind(channel_id)
        .fetch_all(self.deref())
        .await
    }
}
