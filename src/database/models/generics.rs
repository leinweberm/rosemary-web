use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
    pub en: String,
    pub cs: String,
}

