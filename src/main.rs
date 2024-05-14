mod requests;
mod utils;

#[tokio::main]
async fn main() {
    // Routes init
    let routes = requests::router::router();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}