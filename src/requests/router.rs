use warp::Filter;
use crate::requests;

pub fn router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /api/v1.0/paintings
    requests::routes::v1_0::paintings::get_all::get()
    // Error handling
    // .recover(requests::routes::test::not_found::handle_not_found)
}
