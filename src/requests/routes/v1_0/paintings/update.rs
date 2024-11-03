use uuid::Uuid;
use warp::{Filter, Rejection, Reply, path, body};

use crate::database::connection::get_client;
use crate::database::models::painting::{Painting, PaintingBase, PaintingUpdate};
use crate::errors::api_error::InternalServerError;
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};

async fn update_painting(
	painting_uid: Uuid,
	data: PaintingUpdate
) -> Result<impl Reply, Rejection> {
	let client = get_client().await.unwrap().clone();
	debug!(target: "api", "paintings:update - database client aquired");
	debug!(target: "api", "paintings:update - data {:?}", &data);

	let query = Painting::update_query(painting_uid, data);
	debug!(target: "db", "paintings:update - Painting::update_query {}", &query);
	let update_result = sqlx::query_as::<_, PaintingBase>(&query).fetch_one(&client).await;

	match update_result {
		Ok(painting) => {
			debug!(target: "api", "paintings:update - result {:?}", &painting);
			let response = GenericResponse::<PaintingBase> {
				status: Status::Success,
				message: "Painting updated successfully",
				data: Some(painting),
			};
			Ok(warp::reply::with_status(
				warp::reply::json(&response),
				warp::http::StatusCode::OK)
			)
		},
		Err(error) => {
			error!(target: "api", "paintings:update - error {:?}", error);
			Ok(InternalServerError::new().response().await)
		}
	}
}

pub fn update() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::patch()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("paintings"))
		.and(path::param::<Uuid>())
		.and(path::end())
		.and(body::content_length_limit(1024 * 1024))
		.and(body::json::<PaintingUpdate>())
		.and(jwt_auth())
		.and_then(|painting_uid: Uuid, data: PaintingUpdate, _claims: Claims| async move {
			update_painting(painting_uid, data).await
		})
		.with(warp::log("api"))
}