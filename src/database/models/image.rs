use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;

use crate::database::models::generics::Translation;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintingImage {
	pub id: Uuid,
	pub preview: bool,
	pub url: String,
	pub alt: Option<Translation>,
	pub title: Option<Translation>,
	pub painting_id: Uuid,
}

impl PaintingImage {
	pub fn get_all_for_query(id: &Uuid) -> String {
		format!(r#"
			SELECT *
			FROM rosemary.painting_images pi
			WHERE pi.painting_id = {}
		"#, id)
	}

	// pub fn count_all_for_query(id: &Uuid) -> String {
	// 	format!(r#"
	// 		SELECT COUNT(pi.id)
	// 		FROM rosemary.painting_images pi
	// 		WHERE pi.painting_id = {}
	// 	"#, id)
	// }
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