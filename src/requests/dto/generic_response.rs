use serde::{Deserialize, Serialize};
use warp::reply::{Json, WithStatus};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Status {
    Success,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericResponse<'a, T>
where
    T: Serialize,
{
    pub status: Status,
    pub message: &'a str,
    pub data: Option<T>,
}

impl<'a, T> GenericResponse<'a, T>
where
    T: Serialize,
{
    pub fn send(
        status: Status,
        message: &'a str,
        data: Option<T>,
        status_code: warp::http::StatusCode,
    ) -> WithStatus<Json> {
        let response = GenericResponse {
            status,
            message,
            data,
        };
        warp::reply::with_status(warp::reply::json(&response), status_code)
    }
}
