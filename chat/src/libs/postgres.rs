use tokio_postgres::{NoTls, connect, Client};
use super::config::Database;
use anyhow::Result;

pub async fn conn(config: &Database) -> Result<Client>  {
    let c: String = config.into();
    let (client, conn) = connect(&c, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)

}
