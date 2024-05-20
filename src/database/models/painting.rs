use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::database::models::generics::Translation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Painting {
    id: Uuid,
    created: DateTime<Utc>,
    deleted: Option<Option<DateTime<Utc>>>,
    price: Option<i64>,
    painting_title: Option<Translation>,
    painting_description: Option<Translation>,
    data: Option<HashMap<String, String>>,
    width: Option<i64>,
    height: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingImage {
    id: Uuid,
    preview: bool,
    url: String,
    alt: Option<String>,
    title: Option<String>,
    painting_id: Uuid,
}
