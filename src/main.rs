mod requests;
mod utils;

#[tokio::main]
async fn main() {
    let routes = requests::router::router();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}