use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use uuid::Uuid;
use warp::{path, Filter, Rejection, Reply};

use crate::database::connection::get_client;
use crate::database::models::image::PaintingImage;
use crate::database::models::painting::Painting;
use crate::errors::api_error::InternalServerError;

#[derive(Serialize)]
pub struct GetPaintingResult {
    pub painting: Painting,
    pub images: Vec<PaintingImage>,
}

async fn get_painting(id: Uuid) -> Result<impl Reply, Rejection> {
    let client: Arc<Pool<Postgres>> = Arc::new(get_client().await.unwrap().clone());
    debug!(target: "api", "paintings:get - database client connection aquired");

    let id1 = id.clone();
    let painting_client = Arc::clone(&client);
    debug!(target: "api", "paintings:get - id cloned {:?}", &id1);

    let painting_task = tokio::spawn(async move {
        let query = Painting::get_one_query(id1);
        debug!(target: "db", "paintings:get - Painting::get_one_query {}", &query);

        let painting = sqlx::query_as::<_, Painting>(&query)
            .fetch_one(&*painting_client)
            .await;

        match painting {
            Ok(value) => {
                debug!(target: "api", "paintings:get - queried painting row {:?}", &value);
                Ok(value)
            }
            Err(error) => {
                error!(target: "api", "paitings:get - Painting::get_one_query failed {:?}", error);
                Err(error)
            }
        }
    });

    let id2: Uuid = id.clone();
    let images_client = Arc::clone(&client);
    debug!(target: "api", "paintings:get - id cloned {:?}", &id2);

    let images_task = tokio::spawn(async move {
        let query = PaintingImage::get_all_for_query(id2);
        debug!(target: "api", "paintings:get - PaintingImage::get_all_for_query {}", &query);

        let images = sqlx::query_as::<_, PaintingImage>(&query)
            .fetch_all(&*images_client)
            .await;

        match images {
            Ok(value) => {
                debug!(target: "api", "paintings:get - queried image rows {:?}", &value);
                Ok(value)
            }
            Err(error) => {
                error!(target: "api", "paitings:get - PaintingImage::get_all_for_query failed {:?}", error);
                Err(error)
            }
        }
    });

    let (painting_result, images_result) = tokio::join!(painting_task, images_task);

    let painting = match painting_result {
        Ok(Ok(value)) => value,
        Ok(Err(_)) | Err(_) => return Ok(InternalServerError::new().response().await),
    };

    let images = match images_result {
        Ok(Ok(value)) => value,
        Ok(Err(_)) | Err(_) => return Ok(InternalServerError::new().response().await),
    };

    let result = GetPaintingResult { painting, images };

    Ok(warp::reply::with_status(
        warp::reply::json(&result),
        warp::http::StatusCode::OK,
    ))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("api"))
        .and(path("v1.0"))
        .and(path("paintings"))
        .and(path::param::<Uuid>())
        .and(path::end())
        .map(|uuid: Uuid| {
            debug!(target: "api", "Route matched with UUID: {}", uuid); // Debug print
            uuid
        })
        .and_then(get_painting)
}
