use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::types::{Json, JsonValue};
use uuid::Uuid;
use std::collections::HashMap;
use warp::Reply;
use serde_json::json;

use crate::database::models::generics::{Translation, deserialize_json_string};
use crate::database::models::image::PaintingImage;

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

impl <'r>FromRow<'r, PgRow> for PaintingBase {
	fn from_row(row: &'r PgRow) -> sqlx::Result<Self, sqlx::Error> {
		let title_json: JsonValue = row.try_get("painting_title")?;
		let title: Translation = serde_json::from_value(title_json)
			.map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

		let description_json: JsonValue = row.try_get("painting_description")?;
		let description: Translation = serde_json::from_value(description_json)
			.map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

		let data_json: JsonValue = row.try_get("data")?;
		let data: HashMap<String, String> = serde_json::from_value(data_json)
			.map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

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

impl Painting {
	pub fn get_one_query(id: Uuid) -> String {
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
				FROM rosemary.paintings p
				LEFT JOIN rosemary.painting_images pi ON pi.painting_id = p.id AND pi.preview = TRUE AND pi.status = 'PROCESSED'
				WHERE p.id = '{}'
					AND p.deleted IS NULL
				LIMIT 1
		"#, id)
	}

	pub fn count_all_query() -> String {
		r#"
			SELECT COUNT(p.id)
			FROM rosemary.paintings p
			WHERE p.deleted IS NULL
		"#.to_string()
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
			FROM rosemary.paintings p
			LEFT JOIN rosemary.painting_images pi ON pi.painting_id = p.id AND pi.preview = TRUE
			WHERE deleted IS NULL
			ORDER BY p.created DESC
			LIMIT {} OFFSET {}
		"#, limit, offset)
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
			values.push(format!("data = JSONB_SET(data::jsonb, '{{sold}}', {}, true)", value));
		}

		query.push_str(&values.join(", "));
		query.push_str(&format!(" WHERE id = '{}' AND deleted IS NULL RETURNING *;", id));
		query
	}

	pub fn delete_query(id: Uuid, data: PaintingDelete) -> String {
		if data.force {
			format!("DELETE FROM rosemary.paintings WHERE id = '{}'", id)
		} else {
			format!("UPDATE rosemary.paintings SET deleted = now() WHERE id = '{}'", id)
		}
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

impl <'r>FromRow<'r, PgRow> for Painting {
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
		let title: Translation = serde_json::from_value(title_json)
			.map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

		let description_json: JsonValue = row.try_get("painting_description")?;
		let description: Translation = serde_json::from_value(description_json)
			.map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

		let data_json: JsonValue = row.try_get("painting_description")?;
		let data: HashMap<String, String> = serde_json::from_value(data_json)
			.map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

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
			preview: Json(preview)
		})
	}
}