use sqlx::{Pool, Postgres};
use std::net::{Ipv4Addr, SocketAddr};
use warp::Filter;

mod client;
mod config;
mod database;
mod errors;
mod requests;
mod utils;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let _init_config = config::load::init().await?;
    let _test_config = config::load::test().await?;
    debug!("app config loaded and tested");

    debug!(target: "app", "Database connecting");
    let client: Pool<Postgres> = database::connection::init_connection().await?;
    debug!(target: "app", "Database connected");

    utils::auth::token::set_keys().await?;
    debug!(target: "app", "JWT secrets initialized");

    let rows: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&client)
        .await?;

    assert_eq!(rows.0, 150_i64);
    debug!(target: "app", "Database connection checked");

    let routes = requests::router::router().recover(errors::api_error::handle_rejection);
    debug!(target: "app", "Router routes initialized");

    let addr: SocketAddr = {
        #[cfg(debug_assertions)]
        {
            (Ipv4Addr::LOCALHOST, 3030).into()
        }
        #[cfg(not(debug_assertions))]
        {
            (Ipv4Addr::UNSPECIFIED, 3030).into()
        }
    };
    debug!(target: "app", "App is listening on {}", addr.to_string());

    warp::serve(routes).run(addr).await;
    Ok(())
}
