use uuid::Uuid;
use warp::{path, Filter, Rejection, Reply};

use crate::{
    database::{connection::get_client, models::image::PaintingImage},
    errors::api_error::InternalServerError,
    requests::dto::generic_response::{GenericResponse, Status},
};

async fn get_painting_images(id: Uuid) -> Result<impl Reply, Rejection> {
    let client = get_client().await.unwrap();

    let query = PaintingImage::get_all_for_query(id);
    debug!(target: "api", "images:get_all PaintingImage::get_all_for_query {}", &query);
    let images_found = sqlx::query_as::<_, PaintingImage>(&query)
        .fetch_all(&*client)
        .await;

    match images_found {
        Ok(rows) => {
            debug!(target: "api", "images:get_all result OK");
            let response = GenericResponse::<Vec<PaintingImage>> {
                status: Status::Success,
                message: "All images for selected painting",
                data: Some(rows),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                warp::http::StatusCode::OK,
            ))
        }
        Err(error) => {
            error!(target: "api", "imaages:get_all failed {:?}", error);
            Ok(InternalServerError::new().response().await)
        }
    }
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("api"))
        .and(path("v1.0"))
        .and(path("images"))
        .and(path("painting"))
        .and(path::param::<Uuid>())
        .and_then(|id: Uuid| async move { get_painting_images(id).await })
}
