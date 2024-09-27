use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Status {
    Success,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericResponse<'a, T> {
    pub status: Status,
    pub message: &'a str,
		pub data: Option<T>
}