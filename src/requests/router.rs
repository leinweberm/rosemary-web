use warp::Filter;
use crate::requests;

pub fn router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /salute
    requests::routes::test::salute::get()
    // POST /promote
    .or(requests::routes::test::promote::post())
    // POST /file
    .or(requests::routes::test::file::post())
    // POST /upload
    .or(requests::routes::test::upload::post())
    // Error handling
    .recover(requests::routes::test::not_found::handle_not_found)
}