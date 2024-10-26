use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;

use crate::database::models::generics::{Translation, deserialize_json_string};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingImageCreate {
	pub preview: bool,
	pub url: String,
	pub alt_cs: String,
	pub alt_en: String,
	pub title_cs: String,
	pub title_en: String,
	pub painting_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingImage {
	pub id: Uuid,
	pub preview: bool,
	pub url: String,
	#[serde(deserialize_with = "deserialize_json_string")]
	pub alt: Option<Translation>,
	#[serde(deserialize_with = "deserialize_json_string")]
	pub title: Option<Translation>,
	pub painting_id: Uuid,
}

impl PaintingImage {
	pub fn get_all_for_query(id: Uuid) -> String {
		format!(r#"
			SELECT *
			FROM rosemary.painting_images pi
			WHERE pi.painting_id = {}
		"#, id)
	}

	pub fn get_by_id_query(id: Uuid) -> String {
		format!(r#"
			SELECT *
			FROM rosemary.painting_images
			WHERE id = '{}'
			LIMIT 1
		"#, id)
	}

	pub fn create_query(data: PaintingImageCreate) -> String {
		format!(r#"
			INSERT INTO rosemary.painting_images(
				preview,
				url,
				alt,
				title,
				painting_id
			) VALUES (
				{},
				'{}',
				JSON_BUILD_OBJECT(
					'cs', '{}',
					'en', '{}'
				),
				JSON_BUILD_OBJECT(
					'cs', '{}',
					'en', '{}'
				),
				'{}'
			) RETURNING *"#,
			data.preview,
			data.url,
			data.alt_cs,
			data.alt_en,
			data.title_cs,
			data.title_en,
			data.painting_id
		)
	}

	pub fn delete_query(id: Uuid) -> String {
		format!(r#"
			DELETE FROM rosemary.paintin_images
			WHERE id = '{}'
			LIMIT 1
		"#, id)
	}
}

impl<'r> FromRow<'r, PgRow> for PaintingImage {
	fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
		let alt_json: Option<&str> = row.try_get("alt")?;
		let image_alt: Option<Translation> = match alt_json
			.map(|json| serde_json::from_str(json))
			.transpose() {
				Ok(alt) => alt,
				Err(err) => return Err(sqlx::Error::Decode(Box::new(err)))
			};

		let title_json: Option<&str> = row.try_get("title")?;
		let image_title: Option<Translation> = match title_json
			.map(|json| serde_json::from_str(json))
			.transpose() {
				Ok(title) => title,
				Err(err) => return Err(sqlx::Error::Decode(Box::new(err)))
			};

		Ok(PaintingImage {
			id: row.try_get("id")?,
			preview: row.try_get("preview")?,
			url: row.try_get("url")?,
			alt: image_alt,
			title: image_title,
			painting_id: row.try_get("painting_id")?,
		})
	}
}