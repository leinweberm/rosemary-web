use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::postgres::PgRow;
use sqlx::prelude::FromRow;
use sqlx::Row;
use uuid::Uuid;

use crate::requests::dto::generic_response::Status;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub status: Status,
    pub token: String,
    pub ui: Uuid,
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
    pub secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub user_uid: Uuid,
    pub role_type: Option<String>,
    pub username: String,
    pub password: String,
    pub created: DateTime<Utc>,
    pub deleted: Option<DateTime<Utc>>,
    pub preferences: Option<JsonValue>,
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self, sqlx::Error> {
        Ok(Self {
            user_uid: row.try_get("user_uid")?,
            role_type: row
                .try_get("role_type")
                .unwrap_or(Some(String::from("user"))),
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            created: row.try_get("created")?,
            deleted: row.try_get("deleted").unwrap_or(None),
            preferences: row.try_get("preferences").unwrap_or(None),
        })
    }
}

impl User {
    pub fn create_query(data: UserCreate) -> String {
        format!(
            r#"
			INSERT INTO network.users(
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
			RETURNING *
		"#,
            data.username, data.password
        )
    }

    pub fn get_by_id_query(user_uid: Uuid) -> String {
        format!(
            r#"
			SELECT *
			FROM network.users
			WHERE user_uid = '{}'
				AND deleted IS NULL
		"#,
            user_uid
        )
    }

    pub fn get_by_username_query(username: String) -> String {
        format!(
            r#"
			SELECT *
			FROM network.users
			WHERE username = '{}'
				AND deleted IS NULL
		"#,
            username
        )
    }

    pub fn delete_query(user_uid: Uuid, force: bool) -> String {
        if force == true {
            format!(
                r#"
				DELETE FROM network.users
				WHERE user_uid = '{}'
			"#,
                user_uid
            )
        } else {
            format!(
                r#"
				UPDATE network.users
				SET deleted = TRUE
				WHERE user_uid = '{}'
			"#,
                user_uid
            )
        }
    }
}
