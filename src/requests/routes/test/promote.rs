use warp::{Filter, Rejection, Reply, body, path};
use crate::requests::dto::employee::Employee;

async fn post_promote(rate: u32, employee: Employee) -> Result<impl Reply, Rejection> {
    let promoted = Employee {
        name: employee.name,
        rate,
    };
    Ok(warp::reply::json(&promoted))
}

pub fn post() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(path("promote"))
        .and(path::param::<u32>())
        .and(body::content_length_limit(1024*1000))
        .and(body::json())
        .and_then(post_promote)
}