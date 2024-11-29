use warp::{Filter, Rejection, Reply, path};

use crate::database::connection::get_client;
use crate::database::models::painting::{Painting, PaintingCreate, PaintingBase};
use crate::errors::api_error::InternalServerError;
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};

async fn create_painting(data: PaintingCreate) -> Result<impl Reply, Rejection> {
	let client = get_client().await.unwrap().clone();
	debug!(target: "api", "paintings:create - database client aquired");
	debug!(target: "api", "paintings:create - data {:?}", &data);

	let query = Painting::create_query(data);
	debug!(target: "db", "paitings:create - Painting::create_query {}", &query);
	let create_result = sqlx::query_as::<_, PaintingBase>(&query).fetch_one(&client).await;

	let painting_base = match create_result {
		Ok(painting) => {
			debug!(target: "api", "paintings:create - result {:?}", &painting);
			painting
		}
		Err(error) => {
			error!(target: "api", "paintings:create - error {:?}", error);
			return Ok(InternalServerError::new().response().await)
		}
	};

	let response = GenericResponse::<PaintingBase> {
		status: Status::Success,
		message: "Painting created successfully",
		data: Some(painting_base),
	};
	Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::CREATED))
}

pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::post()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("paintings"))
		.and(path::end())
		.and(warp::body::content_length_limit(1024 * 1024))
		.and(warp::body::json::<PaintingCreate>())
		.and(jwt_auth())
		.and_then(|painting: PaintingCreate, _claims: Claims| async move {
			create_painting(painting).await
		})
}