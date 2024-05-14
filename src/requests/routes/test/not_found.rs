#![allow(dead_code)]
use warp::http::StatusCode;

pub async fn handle_not_found(reject: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if reject.is_not_found() {
        Ok(StatusCode::NOT_FOUND)
    } else {
        Err(reject)
    }
}
