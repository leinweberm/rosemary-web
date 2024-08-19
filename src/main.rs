use sqlx::{Pool, Postgres};

mod requests;
mod utils;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Database connecting...");
    let client: Pool<Postgres> = database::connection::init_connection().await?;
    println!("Database connected");

    let rows: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&client)
        .await?;

    assert_eq!(rows.0, 150_i64);
    println!("Database connection checked");

    let routes = requests::router::router();
    println!("Router routes initialized");

    println!("App is listening on 127.0.0.1:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
