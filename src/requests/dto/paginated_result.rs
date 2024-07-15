use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PaginatedResult<T> {
    pub rows: Vec<T>,
    pub count: i64,
}

impl <T> PaginatedResult<T> {
    pub fn new() -> Self {
        PaginatedResult {
            rows: vec![],
            count: 0,
        }
    }
}