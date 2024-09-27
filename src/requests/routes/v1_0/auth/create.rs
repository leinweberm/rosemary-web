// use warp::{Filter, Rejection, Reply, path, body};
// use sqlx::{Postgres, Pool};
//
// use crate::database::connection::get_client;
// use crate::requests::dto::generic_response::{GenericResponse, Status};
//
// pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
// 	warp::post()
// 		.and(path("api"))
// 		.and(path("v1.0"))
// 		.and(path("users"))
// 		.and(path::end())
// 		.and(body::content_length_limit(1024 * 1024))
// 		.and(body::json())
//
// }