use super::config::Database;
use anyhow::Result;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio_postgres::{Client, NoTls, connect};

pub async fn conn(config: &Database) -> Result<Client> {
    let c: String = config.to_st();
    let (client, conn) = connect(&c, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

pub async fn connx(config: &Database) -> Result<Pool<Postgres>> {
    let c: String = config.to_url();
    let pool = PgPoolOptions::new().max_connections(5).connect(&c).await?;
    Ok(pool)
}
