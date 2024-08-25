use sqlx::{Pool, Postgres};

mod requests;
mod utils;
mod database;
mod client;
mod config;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let _init_config = config::load::init().await?;
    let _test_config = config::load::test().await?;
    debug!("app config loaded and tested");

    debug!(target: "app", "Database connecting");
    let client: Pool<Postgres> = database::connection::init_connection().await?;
    debug!(target: "app", "Database connected");

    let rows: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&client)
        .await?;

    assert_eq!(rows.0, 150_i64);
    debug!(target: "app", "Database connection checked");

    let routes = requests::router::router();
    debug!(target: "app", "Router routes initialized");

    debug!(target: "app", "App is listening on 127.0.0.1:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
