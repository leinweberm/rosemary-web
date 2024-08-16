use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;
use std::collections::HashMap;
use crate::database::models::generics::Translation;
use sqlx::types::Json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Painting {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub deleted: Option<DateTime<Utc>>,
    pub price: i64,
    pub painting_title: Option<Translation>,
    pub painting_description: Option<Translation>,
    pub data: Option<HashMap<String, String>>,
    pub width: i64,
    pub height: i64,
    pub preview: Json<PaintingImage>,
}

impl Painting {
    // pub fn get_one_query(id: Uuid) -> String {
    //     format!(r#"
    //         SELECT
    //             p.*,
    //             (JSON_BUILD_OBJECT(
    //                 'id', pi.id,
    //                 'preview', pi.preview,
    //                 'url', pi.url,
    //                 'alt', pi.alt,
    //                 'title', pi.title,
    //                 'painting_id', pi.painting_id
    //             )) AS preview
    //         FROM paintings p
    //         LEFT JOIN painting_images pi ON pi.painting_id = p.id AND pi.preview = TRUE
    //         WHERE id = {}
    //             AND deleted IS NULL
    //         LIMIT 1
    //     "#, id)
    // }

    pub fn count_all_query() -> String {
        format!(r#"
            SELECT COUNT(p.id)
            FROM paintings p
            WHERE p.deleted IS NULL
        "#)
    }

    pub fn get_all_query(limit: u32, offset: u32) -> String {
        format!(r#"
            SELECT
                p.*,
                (JSON_BUILD_OBJECT(
                    'id', pi.id,
                    'preview', pi.preview,
                    'url', pi.url,
                    'alt', pi.alt,
                    'title', pi.title,
                    'painting_id', pi.painting_id
                )) AS preview
            FROM paintings p
            LEFT JOIN paintings_images pi ON pi.painting_id = p.id AND pi.preview = TRUE
            WHERE deleted IS NULL
            ORDER BY p.created DESC
            LIMIT {} OFFSET {}
        "#, limit, offset)
    }
}

impl <'r>FromRow<'r, PgRow> for Painting {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        let id: Uuid = row.try_get("id")?;
        let temp_created: String = row.try_get("created")?;
        let naive_created = NaiveDateTime::parse_from_str(&temp_created, "%Y-%m-%dT%H:%M:%S")
            .expect("Failed to parse naive datetime");
        let created: DateTime<Utc> = TimeZone::from_utc_datetime(&Utc, &naive_created);
        let price: i64 = row.try_get("price").unwrap_or(0);
        let width: i64 = row.try_get("width").unwrap_or(0);
        let height: i64 = row.try_get("height").unwrap_or(0);
        let deleted = row.try_get("deleted").unwrap_or(None);
        let preview: Json<PaintingImage> = row.try_get("preview")?;

        let title_json: Option<&str> = row.try_get("painting_title")?;
        let painting_title: Option<Translation> = match title_json
            .map(|json| serde_json::from_str(json))
            .transpose() {
                Ok(title) => title,
                Err(err) => return Err(sqlx::Error::Decode(Box::new(err)))
            };

        let description_json: Option<&str> = row.try_get("painting_description")?;
        let painting_description: Option<Translation> = match description_json
            .map(|json| serde_json::from_str(json))
            .transpose() {
            Ok(description) => description,
            Err(err) => return Err(sqlx::Error::Decode(Box::new(err)))
        };

        Ok(Self {
            id,
            created,
            deleted,
            price,
            painting_title,
            painting_description,
            data: None,
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

impl<'r> FromRow<'r, PgRow> for PaintingImage {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        Ok(PaintingImage {
            id: row.try_get("id")?,
            preview: row.try_get("preview")?,
            url: row.try_get("url")?,
            alt: row.try_get("alt")?,
            title: row.try_get("title")?,
            painting_id: row.try_get("painting_id")?,
        })
    }
}
