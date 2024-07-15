use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::database::models::generics::Translation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Painting {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub deleted: Option<Option<DateTime<Utc>>>,
    pub price: Option<i64>,
    pub painting_title: Option<Translation>,
    pub painting_description: Option<Translation>,
    pub data: Option<HashMap<String, String>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub preview: PaintingImage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingImage {
    pub id: Uuid,
    pub preview: bool,
    pub url: String,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub painting_id: Uuid,
}
