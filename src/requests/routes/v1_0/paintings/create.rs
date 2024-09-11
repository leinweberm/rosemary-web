use warp::{Filter, Rejection, Reply, path};
use std::sync::Arc;
use sqlx::{Postgres, Pool};

use crate::database::connection::get_client;
use crate::database::models::painting::{Painting, PaintingCreate};

async fn create_painting(data: PaintingCreate) -> Result<impl Reply, Rejection> {
	let client: Arc<Pool<Postgres>> = Arc::new(get_client().await.unwrap().clone());
	let query = Painting::create_query(data);
	let create_result = sqlx::query_as::<_, Painting>(&query)
		.fetch_one(&*client)
		.await
		.expect("Failed to create new painting");

	Ok(warp::reply::json(&create_result))
}

pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::post()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("paintngs"))
		.and(warp::body::content_length_limit(1024 * 1024))
		.and(warp::body::json())
		.and_then(create_painting)
		.with(warp::log("api"))
}