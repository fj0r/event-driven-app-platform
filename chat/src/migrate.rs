mod libs;
use anyhow::Result;
use libs::config::Config;
use libs::postgres::conn;
use tracing::info;

mod embed {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cfg = Config::new()?;
    let mut client = conn(&cfg.database).await?;
    embed::migrations::runner().run_async(&mut client).await?;

    info!("Done");

    Ok(())
}
