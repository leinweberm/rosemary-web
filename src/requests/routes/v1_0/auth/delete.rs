use uuid::Uuid;
use warp::{path, query, Filter, Rejection, Reply};

use crate::config::load::{get, ConfigField};
use crate::database::connection::get_client;
use crate::database::models::user::{User, UserDelete};
use crate::errors::api_error::{InternalServerError, UnauthorizedError};
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};

async fn delete_user(user_id: Uuid, params: UserDelete) -> Result<impl Reply, Rejection> {
    let client = get_client().await.unwrap().clone();
    debug!(target: "api", "users:delete - database client aquired");
    debug!(target: "api", "users:delete data - user_id: {} force: {}", &user_id, &params.force);

    let register_secret = match get::<String>(ConfigField::RegisterUserSecret).await {
        Ok(value) => {
            debug!(target: "api", "users:delete - register secret {}", &value);
            value
        }
        Err(error) => {
            error!(target: "api", "users:delete - error when getting secret {}", error);
            return Ok(UnauthorizedError::new().response().await);
        }
    };

    if params.secret != register_secret {
        error!(target: "api", "users:delete - provided secret is invalid!");
        return Ok(UnauthorizedError::new().response().await);
    }

    let query = User::delete_query(user_id, params.force);
    debug!(target: "db", "users:delete User::delete_query {}", &query);
    let deleted = sqlx::query(&query).fetch_optional(&client).await;

    match deleted {
        Ok(_) => {
            let response = GenericResponse::<UserDelete> {
                status: Status::Success,
                message: "userDeleted",
                data: None,
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                warp::http::StatusCode::OK,
            ))
        }
        Err(error) => {
            error!(target: "api", "users:delete - failed to delete user {}", error);
            Ok(InternalServerError::new().response().await)
        }
    }
}

pub fn delete() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(path("api"))
        .and(path("v1.0"))
        .and(path("users"))
        .and(path::param::<Uuid>())
        .and(path::end())
        .and(query::<UserDelete>())
        .and(jwt_auth())
        .and_then(
            |user_id: Uuid, params: UserDelete, _claims: Claims| async move {
                delete_user(user_id, params).await
            },
        )
        .with(warp::log("api"))
}
