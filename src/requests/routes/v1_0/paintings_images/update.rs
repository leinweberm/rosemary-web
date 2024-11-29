use uuid::Uuid;
use warp::{Filter, Rejection, Reply, path, body};

use crate::database::connection::get_client;
use crate::database::models::image::{PaintingImage, PaintingImageUpdate};
use crate::errors::api_error::InternalServerError;
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};

async fn update_image(
	image_uid: Uuid,
	data: PaintingImageUpdate
) -> Result<impl Reply, Rejection> {
	let client = get_client().await.unwrap().clone();
	debug!(target: "api", "painting_images:update - database client acquired");
	debug!(target: "api", "painting_images:update - data {:?}", &data);

	let query = PaintingImage::update_query(data, image_uid);
	debug!(target: "db", "painting_images:update - PaintingImage::update_query {}", &query);
	let update_result = sqlx::query_as::<_, PaintingImage>(&query).fetch_one(&client).await;

	match update_result {
		Ok(image) => {
			Ok(GenericResponse::send(
				Status::Success,
				"Painting image updated",
				Some(image),
				warp::http::StatusCode::OK
			))
		},
		Err(error) => {
			error!(target: "api", "painting_images:update - error {:?}", error);
			Ok(InternalServerError::new().response().await)
		}
	}
}

pub fn update() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::patch()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("images"))
		.and(path::param::<Uuid>())
		.and(path::end())
		.and(body::content_length_limit(1024 * 1024))
		.and(body::json::<PaintingImageUpdate>())
		.and(jwt_auth())
		.and_then(|image_uid: Uuid, data: PaintingImageUpdate, _claims: Claims| async move {
			update_image(image_uid, data).await
		})
}