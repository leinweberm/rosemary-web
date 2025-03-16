use rand::Rng;
use tokio::time::{sleep, Duration};
use warp::{body, path, Filter, Rejection, Reply};

use crate::database::connection::get_client;
use crate::database::models::user::{LoginResponse, User, UserLogin};
use crate::errors::api_error::UnauthorizedError;
use crate::requests::dto::generic_response::Status;
use crate::utils::auth::password::password_verify;
use crate::utils::auth::token::create;

async fn user_login(data: UserLogin) -> Result<impl Reply, Rejection> {
    let client = get_client().await.unwrap().clone();
    debug!(target: "api", "users:login - database client aquired");
    debug!(target: "api", "users:login - data {:?}", &data);

    let duration = rand::thread_rng().gen_range(10..101);
    sleep(Duration::from_millis(duration)).await;

    let query = User::get_by_username_query(data.username);
    debug!(target: "db", "users:login - User::get_by_username_query {}", &query);
    let find_user = sqlx::query_as::<_, User>(&query).fetch_one(&client).await;

    let user = match find_user {
        Ok(value) => {
            debug!(target: "api", "users:login user {:?}", &value);
            value
        }
        Err(error) => {
            error!(target: "api", "users:login failed to get user from database {}", error);
            return Ok(UnauthorizedError::new().response().await);
        }
    };

    let password_validation = password_verify(&data.password, &user.password).await;
    let valid_password = match password_validation {
        Ok(value) => value,
        Err(error) => {
            error!(target: "api", "users:login password verification failed {}", error);
            return Ok(UnauthorizedError::new().response().await);
        }
    };

    if valid_password == true {
        let token = create(&user.user_uid.to_string(), 3600usize).await;
        match token {
            Ok(value) => {
                let response = LoginResponse {
                    status: Status::Success,
                    token: value,
                    ui: user.user_uid,
                };
                Ok(warp::reply::with_status(
                    warp::reply::json(&response),
                    warp::http::StatusCode::OK,
                ))
            }
            Err(error) => {
                error!(target: "api", "users:login failed to create JWT token {:?}", error);
                Ok(UnauthorizedError::new().response().await)
            }
        }
    } else {
        Ok(UnauthorizedError::new().response().await)
    }
}

pub fn login() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(path("api"))
        .and(path("v1.0"))
        .and(path("users"))
        .and(path("login"))
        .and(path::end())
        .and(body::content_length_limit(1024 * 1024))
        .and(body::json::<UserLogin>())
        .and_then(user_login)
        .with(warp::log("api"))
}
