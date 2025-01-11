use warp::Filter;
use crate::requests;
use crate::errors::api_error::handle_rejection;
use crate::utils::cors::cors_setting::settings;

pub fn router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	// GET /
	requests::routes::frontend::index::get()
	// GET /api/v0.0/paintings/:Uuid
	.or(requests::routes::v1_0::paintings::get::get())
	// GET /api/v1.0/paintings
	.or(requests::routes::v1_0::paintings::get_all::get())
	// POST /api/v1.0/paintings
	.or(requests::routes::v1_0::paintings::create::create())
	// UPDATE /api/v1.0/paintings/:Uuid
	.or(requests::routes::v1_0::paintings::update::update())
	// DELETE /api/v1.0/paintings/:Uuid
	.or(requests::routes::v1_0::paintings::delete::delete())
	// POST /api/v1.0/images
	.or(requests::routes::v1_0::paintings_images::create::create())
	// PATCH /api/v1.0/images/:Uuid
	.or(requests::routes::v1_0::paintings_images::update::update())
	// GET /api/v1.0/images/painting/:Uuid
	.or(requests::routes::v1_0::paintings_images::get_painting_images::get())
	// DELETE /api/v1.0/images/Uuid
	.or(requests::routes::v1_0::paintings_images::delete::delete())
	// POST /api/v1.0/users/login
	.or(requests::routes::v1_0::auth::login::login())
	// POST /api/v1.0/users/register
	.or(requests::routes::v1_0::auth::create::create())
	// DELETE /api/v1.0/users/:Uuid
	.or(requests::routes::v1_0::auth::delete::delete())
	// GET /api/v1.0/users/refresh/token
	.or(requests::routes::v1_0::auth::refresh::refresh())
	// Error handling
	.recover(handle_rejection)
	// Allow CORS
	.with(settings())
	// Logging
	.with(warp::log("api"))
}