mod requests;
mod utils;
mod database;

#[tokio::main]
async fn main() {
    // Database init
    database::connection::init_connection()
        .await
        .unwrap();
    let client = database::connection::get_client()
        .await
        .unwrap();

    // Test static CLIENT inicialization
    let rows = client
        .query("SELECT 1 + 1", &[])
        .await
        .unwrap();
    let value: i64 = rows[0].get(0);
    assert_eq!(value, 2);

    // Routes init
    let routes = requests::router::router();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}