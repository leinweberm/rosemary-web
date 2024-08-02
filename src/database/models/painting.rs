use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use sqlx::{postgres::PgRow, Error, FromRow, Row};

use crate::database::models::generics::Translation;

#[derive(Debug, Deserialize, Serialize)]
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

impl <'r>FromRow<'r, PgRow> for Painting {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let id = row.try_get("id")?;
        let created = row.try_get("created")?;
        let deleted = row.try_get("deleted")?;
        let price = row.try_get("price")?;
        let painting_title = row.try_get("painting_title")?;
        let painting_description = row.try_get("painting_description")?;
        let data = row.try_get("data")?;
        let width = row.try_get("width")?;
        let height = row.try_get("height")?;
        let preview = row.try_get("preview")?;

        Ok(Painting{
            id,
            created,
            deleted,
            price,
            painting_title,
            painting_description,
            data,
            width,
            height,
            preview
        })
    }
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

impl <'r>FromRof<'r, PgRow> for PaintingImage {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let id = row.try_get("id")?;
        let preview = row.try_get("preview")?;
        let url = row.try_get("url")?;
        let alt = row.try_get("alt")?;
        let title = row.try_get("titke")?;
        let painting_id = row.try_get("painting_id")?;

        Ok(PaintingImage{
            id,
            preview,
            url,
            alt,
            title,
            painting_id
        })
    }
}
