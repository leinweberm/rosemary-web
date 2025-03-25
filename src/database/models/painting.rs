use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sql_query_builder as sql;
use sqlx::postgres::PgRow;
use sqlx::prelude::FromRow;
use sqlx::types::{Json, JsonValue};
use sqlx::Row;
use std::collections::HashMap;
use uuid::Uuid;
use warp::Reply;

use crate::client::translations::Language;
use crate::database::models::generics::{deserialize_json_string, Translation};
use crate::database::models::image::PaintingImage;
use crate::requests::dto::get_paintings_query::{GetPaintingsQuery, GetPaintingsQueryParsed};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaintingStub {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub price: i64,
    pub size: String,
    pub title: String,
    pub preview_alt: String,
    pub preview: String,
}

impl<'r> FromRow<'r, PgRow> for PaintingStub {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            created: row.try_get("created")?,
            price: row.try_get("price")?,
            size: row.try_get("size")?,
            title: row.try_get("title")?,
            preview_alt: row.try_get("preview_alt")?,
            preview: row.try_get("preview")?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaintingBase {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub deleted: Option<DateTime<Utc>>,
    pub price: i64,
    #[serde(deserialize_with = "deserialize_json_string")]
    pub painting_title: Option<Translation>,
    #[serde(deserialize_with = "deserialize_json_string")]
    pub painting_description: Option<Translation>,
    #[serde(deserialize_with = "deserialize_json_string")]
    pub data: Option<HashMap<String, String>>,
    pub width: i64,
    pub height: i64,
}

impl<'r> FromRow<'r, PgRow> for PaintingBase {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self, sqlx::Error> {
        let title_json: JsonValue = row.try_get("painting_title")?;
        let title: Translation =
            serde_json::from_value(title_json).map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        let description_json: JsonValue = row.try_get("painting_description")?;
        let description: Translation = serde_json::from_value(description_json)
            .map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        let data_json: JsonValue = row.try_get("data")?;
        let data: HashMap<String, String> =
            serde_json::from_value(data_json).map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        Ok(Self {
            id: row.try_get("id")?,
            created: row.try_get("created")?,
            deleted: row.try_get("deleted").unwrap_or(None),
            price: row.try_get("price")?,
            painting_title: Some(title),
            painting_description: Some(description),
            data: Some(data),
            width: row.try_get("width")?,
            height: row.try_get("height")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingCreate {
    pub price: i64,
    pub title_cs: String,
    pub title_en: String,
    pub description_cs: String,
    pub description_en: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingUpdate {
    pub price: Option<i64>,
    pub title_cs: Option<String>,
    pub title_en: Option<String>,
    pub description_cs: Option<String>,
    pub description_en: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub sold: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingDelete {
    pub force: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Painting {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub deleted: Option<DateTime<Utc>>,
    pub price: i64,
    #[serde(deserialize_with = "deserialize_json_string")]
    pub painting_title: Option<Translation>,
    #[serde(deserialize_with = "deserialize_json_string")]
    pub painting_description: Option<Translation>,
    #[serde(deserialize_with = "deserialize_json_string")]
    pub data: Option<HashMap<String, String>>,
    pub width: i64,
    pub height: i64,
    pub preview: Json<PaintingImage>,
}

impl<'r> FromRow<'r, PgRow> for Painting {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        let id: Uuid = row.try_get("id")?;
        let created: DateTime<Utc> = row.try_get("created")?;
        let price: i64 = row.try_get("price").unwrap_or(0);
        let width: i64 = row.try_get("width").unwrap_or(0);
        let height: i64 = row.try_get("height").unwrap_or(0);
        let deleted = row.try_get("deleted").unwrap_or(None);

        let preview_json: JsonValue = row.try_get("preview")?;
        let preview: PaintingImage = serde_json::from_value(preview_json)
            .map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        let title_json: JsonValue = row.try_get("painting_title")?;
        let title: Translation =
            serde_json::from_value(title_json).map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        let description_json: JsonValue = row.try_get("painting_description")?;
        let description: Translation = serde_json::from_value(description_json)
            .map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        let data_json: JsonValue = row.try_get("painting_description")?;
        let data: HashMap<String, String> =
            serde_json::from_value(data_json).map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        Ok(Self {
            id,
            created,
            deleted,
            price,
            painting_title: Some(title),
            painting_description: Some(description),
            data: Some(data),
            width,
            height,
            preview: Json(preview),
        })
    }
}

impl Reply for Painting {
    fn into_response(self) -> warp::reply::Response {
        let json = json!({
            "id": self.id,
            "created": self.created,
            "deleted": self.deleted,
            "price": self.price,
            "painting_title": self.painting_title,
            "painting_description": self.painting_description,
            "data": self.data,
            "width": self.width,
            "height": self.height,
            "preview": self.preview,
        });

        warp::reply::json(&json).into_response()
    }
}

impl Painting {
    pub fn get_one_query(id: Uuid) -> String {
        format!(
            r#"
			SELECT
				p.*,
				(JSON_BUILD_OBJECT(
					'id', pi.id,
					'preview', pi.preview,
					'urls', pi.urls,
					'alt', pi.alt,
					'title', pi.title,
					'painting_id', pi.painting_id
				)) AS preview
				FROM rosemary.paintings p
				LEFT JOIN rosemary.painting_images pi ON pi.painting_id = p.id AND pi.preview = TRUE
				WHERE p.id = '{}'
					AND p.deleted IS NULL
				LIMIT 1
		"#,
            id
        )
    }

    pub fn count_all_query() -> String {
        r#"
			SELECT COUNT(p.id)
			FROM rosemary.paintings p
			WHERE p.deleted IS NULL
		"#
        .to_string()
    }

    pub fn get_all_query(query: GetPaintingsQuery, language: Option<Language>) -> String {
        let lang = if let Some(language_enum) = language {
            language_enum
        } else {
            Language::Cs
        };
        let parsed_query = query.safe_parse(Some(lang));

        format!(
            r#"
    	SELECT
    		p.*,
    		(JSON_BUILD_OBJECT(
    			'id', pi.id,
    			'preview', pi.preview,
    			'urls', pi.urls,
    			'alt', pi.alt,
    			'title', pi.title,
    			'painting_id', pi.painting_id,
    			'status', status
    		)) AS preview
    	FROM rosemary.paintings p
    	LEFT JOIN rosemary.painting_images pi ON pi.painting_id = p.id AND pi.preview = TRUE
    	WHERE deleted IS NULL
    	ORDER BY {} {}
    	LIMIT {} OFFSET {}
    "#,
            parsed_query.sort, parsed_query.order, parsed_query.limit, parsed_query.offset
        )
    }
    pub fn get_all_stubs_query(
        parsed_query: GetPaintingsQueryParsed,
        language: Option<Language>,
        base_static_files_url: &str,
    ) -> String {
        let lang = if let Some(language_enum) = language {
            language_enum
        } else {
            Language::Cs
        };
        let lang_string = lang.to_string();

        let mut select = sql::Select::new()
            .select("p.id AS id")
            .select("p.created AS created")
            .select("p.price AS price")
            .select("CONCAT(p.width, 'cm x ', p.height, 'cm') AS size")
            .select(&format!("painting_title->>'{}' AS title", &lang_string))
            .select(&format!("pi.alt->>'{}' AS preview_alt", &lang_string))
            .select(&format!(
                "CONCAT('{}', pi.urls->>0) AS preview",
                base_static_files_url
            ))
            .from("rosemary.paintings p")
            .left_join("rosemary.painting_images pi on pi.painting_id = p.id AND pi.preview = TRUE")
            .where_clause("p.deleted IS NULL")
            .limit(&format!("{}", parsed_query.limit))
            .offset(&format!("{}", parsed_query.offset))
            .order_by(&format!("{} {}", parsed_query.sort, parsed_query.order));

        if let Some(search_value) = &parsed_query.search {
            select = select
                .where_clause(&format!(
                    "painting_title->>'{}' LIKE '%{}%'",
                    &lang_string, &search_value
                ))
                .where_clause(&format!(
                    "painting_description->>'{}' LIKE '%{}%'",
                    &lang_string, &search_value
                ));
        }

        select.to_string()
    }

    pub fn create_query(data: PaintingCreate) -> String {
        format!(
            r#"INSERT INTO rosemary.paintings(
				created,
				deleted,
				price,
				painting_title,
				painting_description,
				data,
				width,
				height
			) VALUES (
				now(),
				NULL,
				{},
				JSON_BUILD_OBJECT(
					'cs', '{}',
					'en', '{}'
				),
				JSON_BUILD_OBJECT(
					'cs', '{}',
					'en', '{}'
				),
				JSON_BUILD_OBJECT(),
				{},
				{}
			)
			RETURNING *"#,
            data.price,
            data.title_cs,
            data.title_en,
            data.description_cs,
            data.description_en,
            data.width,
            data.height
        )
    }

    pub fn update_query(id: Uuid, data: PaintingUpdate) -> String {
        let mut values: Vec<String> = Vec::new();
        let mut query = String::from("UPDATE rosemary.paintings SET ");

        if let Some(value) = data.price {
            values.push(format!("price = {}", value));
        }

        if let (Some(cs), Some(en)) = (data.title_cs.as_ref(), data.title_en.as_ref()) {
            values.push(format!(
				"painting_title = JSONB_SET(JSONB_SET(painting_title::jsonb, '{{cs}}', '\"{}\"', true), '{{en}}', '\"{}\"', true)",
				cs,
				en
			));
        } else if let Some(value) = data.title_cs {
            values.push(format!(
                "painting_title = JSONB_SET(painting_title::jsonb, '{{cs}}', '\"{}\"', true)",
                value
            ));
        } else if let Some(value) = data.title_en {
            values.push(format!(
                "painting_title = JSONB_SET(painting_title::jsonb, '{{en}}', '\"{}\"', true)",
                value
            ));
        }

        if let (Some(cs), Some(en)) = (data.description_cs.as_ref(), data.description_en.as_ref()) {
            values.push(format!(
				"painting_description = JSONB_SET(JSONB_SET(painting_description::jsonb, '{{cs}}', '\"{}\"', true), '{{en}}', '\"{}\"', true)",
				cs,
				en
			));
        } else if let Some(value) = data.description_cs {
            values.push(format!("painting_description = JSONB_SET(painting_description::jsonb, '{{cs}}', '\"{}\"', true)", value));
        } else if let Some(value) = data.description_en {
            values.push(format!("painting_description = JSONB_SET(painting_description::jsonb, '{{en}}', '\"{}\"', true)", value));
        }

        if let Some(value) = data.height {
            values.push(format!("height = {}", value));
        }

        if let Some(value) = data.width {
            values.push(format!("width = {}", value));
        }

        if let Some(value) = data.sold {
            values.push(format!(
                "data = JSONB_SET(data::jsonb, '{{sold}}', {}, true)",
                value
            ));
        }

        query.push_str(&values.join(", "));
        query.push_str(&format!(
            " WHERE id = '{}' AND deleted IS NULL RETURNING *;",
            id
        ));
        query
    }

    pub fn delete_query(id: Uuid, data: PaintingDelete) -> String {
        if data.force {
            format!("DELETE FROM rosemary.paintings WHERE id = '{}'", id)
        } else {
            format!(
                "UPDATE rosemary.paintings SET deleted = now() WHERE id = '{}'",
                id
            )
        }
    }
}
