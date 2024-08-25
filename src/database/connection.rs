use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
// use std::env;
// use dotenv::dotenv;
use lazy_static::lazy_static;
use tokio::sync::OnceCell;

use crate::config::load::{ConfigField, get};

lazy_static! {
    pub static ref CLIENT: OnceCell<Pool<Postgres>> = OnceCell::new();
}

pub async fn init_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = get::<String>(ConfigField::DatabaseUrl).await?;
    let cert_path = get::<String>(ConfigField::DatabaseCertPath).await?;

    let mut connect_options: PgConnectOptions = database_url.parse()?;
    debug!(target: "db", "initialized basic connection options");
    connect_options = connect_options.ssl_mode(PgSslMode::Require);
    debug!(target: "db", "setting connection SSL mode");
    connect_options = connect_options.ssl_root_cert(&cert_path);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;
    debug!(target: "db", "aquired database connection pool");

    let pool_clone = pool.clone();
    debug!(target: "db", "database connection pool cloned");

    CLIENT.set(pool_clone)
        .expect("Failed to set client to client as static reference");
    debug!(target: "db", "global static pool reference set");

    Ok(pool)
}

pub async fn get_client() -> Result<&'static Pool<Postgres>, std::io::Error> {
    debug!(target: "db", "getting static datatabase pool reference");
    CLIENT
        .get()
        .ok_or_else(||std::io::Error::new(std::io::ErrorKind::Other, "Client does not exist"))
}
