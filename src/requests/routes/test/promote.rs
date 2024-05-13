use warp::Filter;
use crate::requests::dto::employee::Employee;

async fn post_promote(rate: u32, employee: Employee) -> Result<impl warp::Reply, warp::Rejection> {
    let promoted = Employee {
        name: employee.name,
        rate,
    };
    Ok(warp::reply::json(&promoted))
}

pub fn post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("promote"))
        .and(warp::path::param::<u32>())
        .and(warp::body::content_length_limit(1024*1000))
        .and(warp::body::json())
        .and_then(post_promote)
        .recover(crate::requests::routes::test::not_found::handle_not_found)
}