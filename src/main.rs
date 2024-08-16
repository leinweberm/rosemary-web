use sqlx::{Pool, Postgres};

mod requests;
mod utils;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Pool<Postgres> = database::connection::init_connection().await?;

    let rows: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&client)
        .await?;

    assert_eq!(rows.0, 150_i64);

    let routes = requests::router::router();

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
