use warp::Filter;
use warp::http::{ Response, StatusCode };
use crate::requests::dto::salute_you::SaluteYou;
use memory_stats::memory_stats;

async fn get_salute(person: SaluteYou) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(usage) = memory_stats() {
        println!("Current physical memory usage: {}", usage.physical_mem);
        println!("Current virtual memory usage: {}", usage.virtual_mem);
    }
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .body(String::from(format!("Hello {} {}", person.first_name, person.last_name)))
    )
}

pub fn get() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("salute"))
        .and(warp::query::<SaluteYou>())
        .and_then(get_salute)
        .recover(crate::requests::routes::test::not_found::handle_not_found)
}