use warp::Filter;
use crate::requests;
use crate::errors::api_error::handle_rejection;

pub fn router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	// GET /api/v1.0/paintings
	requests::routes::v1_0::paintings::get_all::get()
	// GET /api/v1.0/paintings/:Uuid
	.or(requests::routes::v1_0::paintings::get::get())
	// POST /api/v1.0/paintings
	.or(requests::routes::v1_0::paintings::create::create())
	// POST /api/v1.0/user/login
	.or(requests::routes::v1_0::auth::login::login())
	// Error handling
	.recover(handle_rejection)
}