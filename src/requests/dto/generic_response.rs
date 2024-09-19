use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Success,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericResponse<T> {
    pub status: Status,
    pub message: String,
		pub data: Option<T>
}