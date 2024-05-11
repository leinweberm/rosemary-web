use warp::Filter;
use warp::http::{ Response, StatusCode };
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Employee {
    rate: u32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SaluteYou {
    first_name: String,
    last_name: String,
}

async fn get_salute_handler(person: SaluteYou) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .body(String::from(format!("Hello {} {}", person.first_name, person.last_name)))
    )
}

fn get_salute() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("salute"))
        .and(warp::query::<SaluteYou>())
        .and_then(get_salute_handler)
}

async fn post_promote_handler(rate: u32, employee: Employee) -> Result<impl warp::Reply, warp::Rejection> {
    let promoted = Employee {
        name: employee.name,
        rate,
    };
    Ok(warp::reply::json(&promoted))
}

fn post_promote() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("promote"))
        .and(warp::path::param::<u32>())
        .and(warp::body::content_length_limit(1024*1000))
        .and(warp::body::json())
        .and_then(post_promote_handler)
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post_promote()
        .or(get_salute())
}

#[tokio::main]
async fn main() {
    let routes = routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}