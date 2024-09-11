use sqlx::{Pool, Postgres};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply, path};
use uuid::Uuid;
use serde::Serialize;

use crate::database::models::painting::Painting;
use crate::database::models::image::PaintingImage;
use crate::database::connection::get_client;

#[derive(Serialize)]
pub struct GetPaintingResult {
	pub painting: Painting,
	pub images: Vec<PaintingImage>,
}

#[derive(Debug)]
struct PaintingNotFound;
impl warp::reject::Reject for PaintingNotFound {}

async fn get_painting(id: Uuid) -> Result<impl Reply, Rejection> {
	let client: Arc<Pool<Postgres>> = Arc::new(get_client().await.unwrap().clone());

	let id1 = id.clone();
	let painting_client = Arc::clone(&client);
	let painting_task = tokio::spawn(async move {
		let query = Painting::get_one_query(id1);
		let painting = sqlx::query_as::<_, Painting>(&query)
			.fetch_one(&*painting_client)
			.await;
		painting
	});

	let id2: Uuid = id.clone();
	let images_client = Arc::clone(&client);
	let images_task = tokio::spawn(async move {
		let query = PaintingImage::get_all_for_query(&id2);
		let images = sqlx::query_as::<_, PaintingImage>(&query)
			.fetch_all(&*images_client)
			.await;
		images
}	);

	let (painting_result, images_result) = tokio::join!(painting_task, images_task);

	let painting = match painting_result {
		Ok(Ok(p)) => p,
		Ok(Err(_)) | Err(_) => return Err(warp::reject::not_found()),
	};

	let images = match images_result {
		Ok(Ok(i)) => i,
		Ok(Err(_)) | Err(_) => Vec::new(),
	};

	let result = GetPaintingResult {
		painting,
		images,
	};

	Ok(warp::reply::json(&result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::get()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("paintings"))
		.and(warp::path::param::<Uuid>())
		.and_then(get_painting)
		.with(warp::log("api"))
}