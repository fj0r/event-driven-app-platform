mod libs;
use libs::config::Config;
use sqlx::Connection;
use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    let cfg = Config::new()?;
    dbg!(&cfg.database);

    tracing_subscriber::fmt::init();

    Ok(())
}
