use warp::{Filter, Rejection, Reply, path, query};
use warp::http::{ Response, StatusCode };
use crate::requests::dto::salute_you::SaluteYou;
use memory_stats::memory_stats;

async fn get_salute(person: SaluteYou) -> Result<impl Reply, Rejection> {
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

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("salute"))
        .and(query::<SaluteYou>())
        .and_then(get_salute)
}