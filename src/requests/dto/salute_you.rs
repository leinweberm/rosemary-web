use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaluteYou {
    pub first_name: String,
    pub last_name: String,
}