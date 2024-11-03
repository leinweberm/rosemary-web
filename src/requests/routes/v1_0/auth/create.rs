use warp::{Filter, Rejection, Reply, path, body};
use tokio::time::{sleep, Duration};
use rand::Rng;

use crate::database::connection::get_client;
use crate::database::models::user::{User, UserCreate, LoginResponse};
use crate::errors::api_error::{InternalServerError, UnauthorizedError};
use crate::requests::dto::generic_response::Status;
use crate::config::load::{ConfigField, get};
use crate::utils::auth::password::hash_password;

async fn user_create(mut data: UserCreate) -> Result<impl Reply, Rejection> {
	debug!(target: "api", "users:create - starting the proccess");
	let duration = rand::thread_rng().gen_range(10..101);
	sleep(Duration::from_millis(duration)).await;

	let register_secret = match get::<String>(ConfigField::RegisterUserSecret).await {
		Ok(value) => {
			debug!(target: "api", "users:create - register secret {}", &value);
			value
		},
		Err(error) => {
			error!(target: "api", "users:create - error when getting secret {}", error);
			return Ok(UnauthorizedError::new().response().await)
		}
	};

	if data.secret != register_secret {
		error!(target: "api", "users:create - provided create user secret is invalid!");
		return Ok(UnauthorizedError::new().response().await)
	}

	let client = get_client().await.unwrap().clone();
	debug!(target: "api", "users:create - database client aquired");
	debug!(target: "api", "users:create - user data {:?}", &data);

	let data_password = data.password.clone();
	let hash_password = hash_password(&data_password).await;
	match hash_password {
		Ok(password) => {
			debug!(target: "api", "users:create - password hashed {}", &password);
			data.password = password
		},
		Err(error) => {
			error!(target: "api", "users:create - hashing password failed {}", error);
			return Ok(InternalServerError::new().response().await)
		}
	};

	let query = User::create_query(data);
	debug!(target: "db", "users:create - User::create_query {}", &query);
	let created_user = sqlx::query_as::<_, User>(&query).fetch_one(&client).await;

	let user = match created_user {
		Ok(value) => {
			debug!(target: "api", "users:create - created user {:?}", &value);
			value
		},
		Err(error) => {
			error!(target: "api", "users:create - failed to create new user {}", error);
			return Ok(InternalServerError::new().response().await)
		}
	};

	let token = crate::utils::auth::token::create(&user.user_uid.to_string(), 3600usize).await;
	match token {
		Ok(value) => {
			debug!(target: "api", "users:create SUCCESS!");
			let response = LoginResponse {
				status: Status::Success,
				token: value,
				ui: user.user_uid
			};
			Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::CREATED))
		},
		Err(error) => {
			error!(target: "api", "users:create - failed to create JWT token {:?}", error);
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