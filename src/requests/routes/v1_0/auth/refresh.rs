use warp::{Filter, Rejection, Reply, path};

use crate::requests::dto::generic_response::Status;
use crate::utils::auth::token::{jwt_auth, Claims, create};
use crate::database::models::user::LoginResponse;

async fn refresh_token(claims: Claims) -> Result<impl Reply, Rejection> {
	debug!(target: "api", "users:refresh_token claims {:?}", &claims);
	match create(&claims.ui.to_string(), 3600usize).await {
		Ok(value) => {
			let response = LoginResponse {
				status: Status::Success,
				ui: claims.ui,
				token: value,
			};
			Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK))
		},
		Err(error) => {
			error!(target: "api", "users:refresh_token refreshing token failed {:?}", error);
			return Err(error)
		}
	}
}

pub fn refresh() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::get()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("users"))
		.and(path("refresh"))
		.and(path("token"))
		.and(path::end())
		.and(jwt_auth())
		.and_then(|claims: Claims| async move {
			refresh_token(claims).await
		})
		.with(warp::log("api"))
}