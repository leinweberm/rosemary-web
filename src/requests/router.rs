use warp::Filter;
use crate::requests;

pub fn router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    requests::routes::test::promote::post()
    .or(requests::routes::test::salute::get())
    .or(requests::routes::test::file::post())
}