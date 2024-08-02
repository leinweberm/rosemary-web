use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

lazy_static! {
    pub static ref CLIENT: OnceCell<Pool<Postgres>> = OnceCell::new();
}

pub async fn init_connection() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("database_url")
        .expect("$database_url must be set");

    let mut connect_options: PgConnectOptions = database_url.parse()?;
    connect_options = connect_options.ssl_mode(PgSslMode::Require);
    connect_options = connect_options.ssl_root_cert("certs/root.crt");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    CLIENT.set(pool)
        .expect("Failed to set client");

    Ok(())
}

pub async fn get_client() -> Result<&'static Pool<Postgres>, std::io::Error> {
    CLIENT.get().ok_or_else(||std::io::Error::new(
        std::io::ErrorKind::Other,
        "Client does not exist"
    ))
}
