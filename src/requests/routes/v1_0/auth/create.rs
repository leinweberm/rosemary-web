use warp::{Filter, Rejection, Reply, path, body};
use tokio::time::{sleep, Duration};
use rand::Rng;

use crate::database::connection::get_client;
use crate::database::models::user::{User, UserCreate, LoginResponse};
use crate::errors::api_error::{InternalServerError, UnauthorizedError};
use crate::requests::dto::generic_response::Status;
use crate::config::load::{ConfigField, get};

async fn user_create(data: UserCreate) -> Result<impl Reply, Rejection> {
	let duration = rand::thread_rng().gen_range(10..101);
	sleep(Duration::from_millis(duration)).await;

	let register_secret = match get::<String>(ConfigField::RegisterUserSecret).await {
		Ok(value) => value,
		Err(_) => {
			return Ok(UnauthorizedError::new().response().await)
		}
	};

	if data.secret != register_secret {
		return Ok(UnauthorizedError::new().response().await)
	}

	let client = get_client().await.unwrap().clone();
	debug!(target: "api", "user_create:client - database client aquired");
	debug!(target: "api", "user_create:data - {:?}", &data);

	let query = User::create_query(data);
	debug!(target: "api", "user_create:query - {}", &query);
	let created_user = sqlx::query_as::<_, User>(&query).fetch_one(&client).await;

	let user = match created_user {
		Ok(value) => value,
		Err(error) => {
			error!(target: "api", "Failed to create new user - {}", error);
			return Ok(InternalServerError::new().response().await)
		}
	};

	let token = crate::utils::auth::token::create(&user.user_uid.to_string(), 3600usize).await;
	match token {
		Ok(value) => {
			let response = LoginResponse {
				status: Status::Success,
				token: value,
				ui: user.user_uid
			};
			Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::CREATED))
		},
		Err(error) => {
			error!(target: "api", "Failed to create JWT token - {:?}", error);
			Ok(InternalServerError::new().response().await)
		}
	}
}

pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::post()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("users"))
		.and(path::end())
		.and(body::content_length_limit(1024 * 1024))
		.and(body::json::<UserCreate>())
		.and_then(user_create)
		.with(warp::log("api"))
}