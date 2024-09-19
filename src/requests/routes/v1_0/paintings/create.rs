use warp::{Filter, Rejection, Reply, path};
use std::sync::Arc;
use sqlx::{Postgres, Pool};

use crate::database::connection::get_client;
use crate::database::models::painting::{Painting, PaintingCreate, PaintingBase};
use crate::requests::dto::generic_response::{GenericResponse, Status};

async fn create_painting(data: PaintingCreate) -> Result<impl Reply, Rejection> {
	let client: Arc<Pool<Postgres>> = Arc::new(get_client().await.unwrap().clone());
	debug!(target: "api", "paintings_create:client - database client aquired");
	debug!(target: "api", "paintings_create:data - {:?}", data);

	let query = Painting::create_query(data);
	debug!(target: "api", "paitings_create:query - {}", &query);
	let create_result = sqlx::query_as::<_, PaintingBase>(&query)
		.fetch_one(&*client)
		.await;

	match create_result {
        Ok(painting) => {
						debug!(target: "api", "paintings_create:result - {:?}", &painting);
            let response = GenericResponse::<PaintingBase> {
                status: Status::Success,
                message: "Painting created successfully".to_string(),
								data: Some(painting),
            };
            Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::CREATED))
        }
        Err(e) => {
            error!(target: "api", "paintings_create:error - {:?}", e);
            let response = GenericResponse::<PaintingBase> {
                status: Status::Error,
                message: format!("Failed to create painting: {}", e),
								data: None,
            };
            Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::post()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("paintings"))
		.and(path::end())
		.and(warp::body::content_length_limit(1024 * 1024))
		.and(warp::body::json())
		.and_then(create_painting)
		.with(warp::log("api"))
}