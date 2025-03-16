use uuid::Uuid;
use warp::{path, query, Filter, Rejection, Reply};

use crate::database::connection::get_client;
use crate::database::models::painting::{Painting, PaintingDelete};
use crate::errors::api_error::InternalServerError;
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};

async fn delete_painting(
    painting_uid: Uuid,
    params: PaintingDelete,
) -> Result<impl Reply, Rejection> {
    let client = get_client().await.unwrap().clone();
    debug!(target: "api", "paintings:delete - database client aquired");
    debug!(target: "api", "paintings:delete - painting_uid: {}, force: {}", &painting_uid, &params.force);

    let query = Painting::delete_query(painting_uid, params);
    debug!(target: "db", "paintings:delete - Painting::delete_query {}", &query);
    let deleted = sqlx::query(&query).fetch_optional(&client).await;

    match deleted {
        Ok(_) => {
            let response = GenericResponse::<PaintingDelete> {
                status: Status::Success,
                message: "paintingDeleted",
                data: None,
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                warp::http::StatusCode::OK,
            ))
        }
        Err(error) => {
            error!(target: "api", "paintings:delete - failed to delete painting {}", error);
            Ok(InternalServerError::new().response().await)
        }
    }
}

pub fn delete() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(path("api"))
        .and(path("v1.0"))
        .and(path("paintings"))
        .and(path::param::<Uuid>())
        .and(path::end())
        .and(query::<PaintingDelete>())
        .and(jwt_auth())
        .and_then(
            |painting_uid: Uuid, params: PaintingDelete, _claims: Claims| async move {
                delete_painting(painting_uid, params).await
            },
        )
}
