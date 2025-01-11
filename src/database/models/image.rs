use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use sqlx::prelude::FromRow;
use sqlx::postgres::PgRow;
use sqlx::{Error, Row};
use uuid::Uuid;
use serde_json::Value;

use crate::database::models::generics::{Translation};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingImageCreate {
	pub preview: bool,
	pub alt_cs: String,
	pub alt_en: String,
	pub title_cs: String,
	pub title_en: String,
	pub painting_id: Uuid,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingImageUpdate {
	pub alt_cs: Option<String>,
	pub alt_en: Option<String>,
	pub title_cs: Option<String>,
	pub title_en: Option<String>,
	pub status: Option<String>,
	pub preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingImage {
	pub id: Uuid,
	pub preview: bool,
	pub alt: Option<Translation>,
	pub title: Option<Translation>,
	pub painting_id: Uuid,
	pub status: Option<String>,
	pub file_location: Option<String>,
	pub urls: Vec<String>
}

impl PaintingImage {
	pub fn get_all_for_query(id: Uuid) -> String {
		format!(r#"
			SELECT *
			FROM rosemary.painting_images pi
			WHERE pi.painting_id = '{}'
			AND status = 'PROCESSED'
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
				alt,
				title,
				painting_id,
				status
			) VALUES (
				{},
				JSONB_BUILD_OBJECT(
					'cs', '{}',
					'en', '{}'
				),
				JSONB_BUILD_OBJECT(
					'cs', '{}',
					'en', '{}'
				),
				'{}',
				'CREATED'
			) RETURNING *"#,
			data.preview,
			data.alt_cs,
			data.alt_en,
			data.title_cs,
			data.title_en,
			data.painting_id
		)
	}

	pub fn update_query(data: PaintingImageUpdate, id: Uuid) -> String {
		let mut values: Vec<String> = Vec::new();
		let mut query = String::from("UPDATE rosemary.painting_images SET ");

		if let Some(new_status) = data.status {
			values.push(format!("status = '{}'", new_status));
		};

		debug!(target: "db", "preview {:?}", &data.preview);
		if let Some(preview) = data.preview {
			debug!(target: "db", "preview update {}", &preview);
			values.push(format!("preview = {}", preview))
		};

		if let (Some(cs), Some(en)) = (data.alt_cs.as_ref(), data.alt_en.as_ref()) {
			values.push(format!(
				"alt = JSONB_SET(JSONB_SET(alt::jsonb, '{{cs}}', '\"{}\"', true), '{{en}}', '\"{}\"', true)",
				cs,
				en
			));
		} else if let Some(cs) = data.alt_cs {
			values.push(format!(
				"alt = JSONB_SET(alt::jsonb, '{{cs}}', '\"{}\"')", cs
			));
		} else if let Some(en) = data.alt_en {
			values.push(format!(
				"alt = JSONB_SET(alt::jsonb, '{{en}}', '\"{}\"')", en
			));
		};

		if let (Some(cs), Some(en)) = (data.title_cs.as_ref(), data.title_en.as_ref()) {
			values.push(format!(
				"title = JSONB_SET(JSONB_SET(title::jsonb, '{{cs}}', '\"{}\"', true), '{{en}}', '\"{}\"', true)",
				cs,
				en
			));
		} else if let Some(cs) = data.title_cs {
			values.push(format!(
				"title = JSONB_SET(title::jsonb, '{{cs}}', '\"{}\"')", cs
			));
		} else if let Some(en) = data.title_en {
			values.push(format!(
				"title = JSONB_SET(title::jsonb, '{{en}}', '\"{}\"')", en
			));
		}

		query.push_str(&values.join(", "));
		query.push_str(&format!(" WHERE id = '{}' RETURNING *;", id));
		query
	}

	pub fn update_resized_query(id: Uuid, location: String, urls: Vec<String>) -> String {
		format!(
			r#"UPDATE rosemary.painting_images SET
				status = 'PROCESSED',
				file_location = '{}',
				urls = JSONB_BUILD_ARRAY(
					'{}',
					'{}',
					'{}',
					'{}'
				)
			WHERE id = '{}'
			RETURNING *"#,
			location,
			urls[0],
			urls[1],
			urls[2],
			urls[3],
			id
		)
	}

	pub fn delete_query(id: Uuid) -> String {
		format!(r#"
			DELETE FROM rosemary.painting_images
			WHERE id = '{}'
			LIMIT 1
		"#, id)
	}
}

fn try_from_json<T>(field: Option<Value>) -> Result<Option<T>, Error> where T: DeserializeOwned {
	match field.map(|json| serde_json::from_value(json)).transpose() {
		Ok(value) => Ok(value),
		Err(err) => {
			error!(target: "db", "images:try_from_json - error decoding json/jsonb column {}", err);
			Err(Error::Decode(Box::new(err)))
		}
	}
}

impl<'r> FromRow<'r, PgRow> for PaintingImage {
	fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
		let alt_json: Option<Value> = row.try_get("alt")?;
		let image_alt = try_from_json::<Translation>(alt_json)?;
		let title_json: Option<Value> = row.try_get("title")?;
		let image_title = try_from_json::<Translation>(title_json)?;
		let urls_json: Option<Value> = row.try_get("urls")?;
		let image_urls = try_from_json::<Vec<String>>(urls_json)?;
		let final_urls = image_urls.unwrap_or_else(|| vec![]);

		Ok(PaintingImage {
			id: row.try_get("id")?,
			preview: row.try_get("preview")?,
			alt: image_alt,
			title: image_title,
			painting_id: row.try_get("painting_id")?,
			status: row.try_get("status")?,
			urls: final_urls,
			file_location: row.try_get("file_location")?,
		})
	}
}