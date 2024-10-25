use std::path::Path;
use uuid::Uuid;
use warp::{path, Filter, Rejection, Reply};

use crate::database::models::image::PaintingImage;
use crate::errors::api_error::InternalServerError;
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};
use crate::database::connection::get_client;
use crate::config::load::{ConfigField, get};
use crate::utils::file_system::fs_delete::remove_file;

async fn delete_painting_image(id: Uuid) -> Result<impl Reply, Rejection> {
	let client = get_client().await.unwrap();

	let query = PaintingImage::get_by_id_query(id);
	debug!(target: "api", "image_delete:query - {}", &query);
	let find_image = sqlx::query_as::<_, PaintingImage>(&query).fetch_one(&*client).await;

	let image = match find_image {
		Ok(row) => row,
		Err(error) => {
			error!(target: "api", "image_delete:error - get image data from database {:?}", error);
			return Ok(InternalServerError::new().response().await)
		}
	};

	let file_path = match get::<String>(ConfigField::StaticFilesDir).await {
		Ok(static_dir_path) => {
			if let Some(pos) = image.url.rfind('/') {
				let file_name = &image.url[pos + 1..];
				Path::new(&static_dir_path).join(format!("/images/{}", file_name))
			} else {
				error!(target: "api", "image_delete:error filename not found");
				return Ok(InternalServerError::new().response().await)
			}
		},
		Err(error) => {
			error!(target: "api", "image_delete:error generate file path - {:?}", error);
			return Ok(InternalServerError::new().response().await)
		}
	};
	debug!(target: "api", "image_delete:path - {:?}", &file_path);

	if let Some(path) = file_path.to_str() {
		let removed = remove_file(path).await;
		if !removed {
			return Ok(InternalServerError::new().response().await)
		};
	}

	let delete_query = PaintingImage::delete_query(id);
	debug!(target: "api", "image_delete:query - {}", &delete_query);
	let removed_row = sqlx::query(&delete_query).fetch_one(&*client).await;

	match removed_row {
		Ok(_) => {
			debug!(target: "api", "image_delete:result OK");
		},
		Err(error) => {
			error!(target: "api", "image_delete:error - {:?}", error);
			return Ok(InternalServerError::new().response().await)
		}
	}

	let response = GenericResponse::<PaintingImage> {
		status: Status::Success,
		message: "Painting image deleted successfuly",
		data: None
	};
	Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK))
}

pub fn delete() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::delete()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("images"))
		.and(path::param::<Uuid>())
		.and(path::end())
		.and(jwt_auth())
		.and_then(|id: Uuid, _claims: Claims| async move {
			delete_painting_image(id).await
		})
		.with(warp::log("api"))
}