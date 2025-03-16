use std::path::Path;
use uuid::Uuid;
use warp::{path, Filter, Rejection, Reply};

use crate::database::connection::get_client;
use crate::database::models::image::PaintingImage;
use crate::errors::api_error::InternalServerError;
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};
use crate::utils::file_system::fs_delete::remove_file;
use crate::utils::file_system::fs_read::file_exists;

async fn delete_painting_image(id: Uuid) -> Result<impl Reply, Rejection> {
    let client = get_client().await.unwrap();

    let query = PaintingImage::get_by_id_query(id);
    debug!(target: "db", "images:delete PaintingImage::get_by_id_query {}", &query);
    let find_image = sqlx::query_as::<_, PaintingImage>(&query)
        .fetch_one(&*client)
        .await;

    let image = match find_image {
        Ok(row) => row,
        Err(error) => {
            error!(target: "api", "images:delete - get image data from database {:?}", error);
            return Ok(InternalServerError::new().response().await);
        }
    };

    if image.urls.is_empty() {
        return Ok(GenericResponse::<PaintingImage>::send(
            Status::Success,
            "noFileFound",
            None,
            warp::http::StatusCode::OK,
        ));
    }

    for url in &image.urls {
        let location = image.file_location.as_ref().unwrap();
        let removal_path = Path::new(location).join(url);
        let file_exists = file_exists(removal_path.to_string_lossy().to_string().as_ref()).await;

        if !file_exists {
            continue;
        }

        let removed_file = remove_file(removal_path.to_string_lossy().to_string().as_ref()).await;

        if !removed_file {
            debug!(target: "api", "images:delete - failed to remove image {}", removal_path.display());
            return Ok(InternalServerError::new().response().await);
        }
    }

    let delete_query = PaintingImage::delete_query(id);
    debug!(target: "api", "images:delete PaintingImage::delete_query {}", &delete_query);
    let removed_row = sqlx::query(&delete_query).fetch_optional(&*client).await;

    match removed_row {
        Ok(_) => {
            debug!(target: "api", "images:delete result OK");
        }
        Err(error) => {
            error!(target: "api", "images:delete error {:?}", error);
            return Ok(InternalServerError::new().response().await);
        }
    }

    Ok(GenericResponse::<PaintingImage>::send(
        Status::Success,
        "imageDeleted",
        None,
        warp::http::StatusCode::OK,
    ))
}

pub fn delete() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(path("api"))
        .and(path("v1.0"))
        .and(path("images"))
        .and(path::param::<Uuid>())
        .and(path::end())
        .and(jwt_auth())
        .and_then(|id: Uuid, _claims: Claims| async move { delete_painting_image(id).await })
}
