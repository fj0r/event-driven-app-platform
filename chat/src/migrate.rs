mod libs;
use anyhow::Result;
use libs::config::{Config, Database};
use tokio_postgres::NoTls;
use tracing::info;

mod embed {
    use refinery::embed_migrations;
    embed_migrations!();
}

macro_rules! concat_fields {
    (var $y:ident; $($t:tt)*) => {
        let mut x = Vec::new();
        concat_fields!(@v x, @k $y; $($t)*)
    };
    (@v $x:ident, @k $y:ident; $a:ident; $($t:tt)*) => {
        $x.push(format!("{}={}", stringify!($a), $y.$a));
        concat_fields!(@v $x, @k $y; $($t)*)
    };
    (@v $x:ident, @k $y:ident; $a:ident = $b:ident; $($t:tt)*) => {
        $x.push(format!("{}={}", stringify!($a), $y.$b));
        concat_fields!(@v $x, @k $y; $($t)*)
    };
    (@v $x:ident, @k $y:ident; $($t:tt)*) => {
        $x.join(" ")
    };
}

impl From<&Database> for String {
    fn from(value: &Database) -> Self {
        concat_fields!{
            var value;
            host;
            port;
            dbname = db;
            user;
            password = passwd;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::new()?;

    tracing_subscriber::fmt::init();

    let dbc: String = (&cfg.database).into();
    dbg!(&dbc);
    let (mut client, conn) = tokio_postgres::connect(&dbc, NoTls).await?;
    conn.await?;
    embed::migrations::runner().run_async(&mut client).await?;

    Ok(())
}
