use crate::errors::api_error::handle_rejection;
use crate::requests::routes;
use crate::utils::cors::cors_setting::settings;
use warp::Filter;

pub fn router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET index page
    routes::frontend::index::get()
        .or(routes::frontend::index::get_cz())
        .or(routes::frontend::index::get_en())
        // GET /gallery
        .or(routes::frontend::gallery::get())
        .or(routes::frontend::gallery::get_cz())
        .or(routes::frontend::gallery::get_en())
        // GET /contact
        .or(routes::frontend::contact::get())
        .or(routes::frontend::contact::get_cz())
        .or(routes::frontend::contact::get_en())
        // GET /api/v0.0/paintings/:Uuid
        .or(routes::v1_0::paintings::get::get())
        // GET /api/v1.0/paintings
        .or(routes::v1_0::paintings::get_all::get())
        // POST /api/v1.0/paintings
        .or(routes::v1_0::paintings::create::create())
        // UPDATE /api/v1.0/paintings/:Uuid
        .or(routes::v1_0::paintings::update::update())
        // DELETE /api/v1.0/paintings/:Uuid
        .or(routes::v1_0::paintings::delete::delete())
        // POST /api/v1.0/images
        .or(routes::v1_0::paintings_images::create::create())
        // PATCH /api/v1.0/images/:Uuid
        .or(routes::v1_0::paintings_images::update::update())
        // GET /api/v1.0/images/painting/:Uuid
        .or(routes::v1_0::paintings_images::get_painting_images::get())
        // DELETE /api/v1.0/images/Uuid
        .or(routes::v1_0::paintings_images::delete::delete())
        // POST /api/v1.0/users/login
        .or(routes::v1_0::auth::login::login())
        // POST /api/v1.0/users/register
        .or(routes::v1_0::auth::create::create())
        // DELETE /api/v1.0/users/:Uuid
        .or(routes::v1_0::auth::delete::delete())
        // GET /api/v1.0/users/refresh/token
        .or(routes::v1_0::auth::refresh::refresh())
        // Error handling
        .recover(handle_rejection)
        // Allow CORS
        .with(settings())
        // Logging
        .with(warp::log("api"))
}
