use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::prelude::FromRow;
use sqlx::postgres::PgRow;
use sqlx::Row;
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCreate {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLogin {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserChangePassword {
	pub old_password: String,
	pub new_password: String,
	pub new_password_again: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDelete {
	pub force: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
	pub user_uid: Uuid,
	pub role_type: Option<String>,
	pub username: String,
	pub password: String,
	pub created: DateTime<Utc>,
	pub deleted: Option<DateTime<Utc>>,
	pub preferences: Option<JsonValue>
}

impl <'r>FromRow<'r, PgRow> for User {
	fn from_row(row: &'r PgRow) -> sqlx::Result<Self, sqlx::Error> {
		Ok(Self {
			user_uid: row.try_get("user_uid")?,
			role_type: row.try_get("role_type").unwrap_or(Some(String::from("user"))),
			username: row.try_get("username")?,
			password: row.try_get("password")?,
			created: row.try_get("created")?,
			deleted: row.try_get("deleted").unwrap_or(None),
			preferences: row.try_get("preferences").unwrap_or(None)
		})
	}
}

impl User {
	pub fn create_qeury(data: UserCreate) -> String {
		format!(
			r#"INSERT INTO rosemary.users(
				role_type,
				username,
				password,
				deleted,
				preferences
			) VALUES (
				'user',
				'{}',
				'{}',
				NULL,
				NULL
			)
			"#,
			data.username,
			data.password
		)
	}

	pub fn get_by_username(username: String) -> String {
		format!(
			r#"SELECT *
			FROM rosemary.users
			WHERE username = '{}'
				AND deleted IS NULL"#,
			username
		)
	}
}