use warp::{Filter, Rejection, Reply, path, query};
use uuid::Uuid;

use crate::database::connection::get_client;
use crate::database::models::user::{User, UserDelete};
use crate::errors::api_error::InternalServerError;
use crate::requests::dto::generic_response::{Status, GenericResponse};
use crate::utils::auth::token::{jwt_auth, Claims};

async fn delete_user(user_id: Uuid, params: UserDelete) -> Result<impl Reply, Rejection> {
	let client = get_client().await.unwrap().clone();
	debug!(target: "api", "user_delete:client - database client aquired");
	debug!(target: "api", "user_delete:data - user_id: {} force: {}", &user_id, &params.force);

	let query = User::delete_query(user_id, params.force);
	let deleted = sqlx::query(&query).fetch_one(&client).await;

	match deleted {
		Ok(_) => {
			let response = GenericResponse::<UserDelete> {
				status: Status::Success,
				message: "userDeleted",
				data: None,
			};
			Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK))
		},
		Err(error) => {
			error!(target: "api", "Failed to delete user {}", error);
			Ok(InternalServerError::new().response().await)
		}
	}
}

pub fn delete() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::delete()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("users"))
		.and(warp::path::param::<Uuid>())
		.and(path::end())
		.and(query::<UserDelete>())
		.and(jwt_auth())
		.and_then(|user_id: Uuid, params: UserDelete, _claims: Claims| async move {
			delete_user(user_id, params).await
		})
		.with(warp::log("api"))
}