use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sql_query_builder as sql;
use sqlx::Row;
use sqlx::{postgres::PgRow, prelude::FromRow};
use warp::Reply;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserEntryCount {
	pub count: i8,
}

impl<'r> FromRow<'r, PgRow> for UserEntryCount {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        Ok(Self {
						count: row.try_get("count")?
        })
    }
}

pub enum UserEntryType {
    ContactForm,
}

impl UserEntryType {
    pub fn to_str(&self) -> &str {
        match self {
            UserEntryType::ContactForm => "contact_form",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            UserEntryType::ContactForm => String::from("contact_form"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserEntry {
    pub id: i64,
    pub email: Option<String>,
    pub entry_type: String,
    pub ip_address: String,
    pub created: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for UserEntry {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            email: row.try_get("email")?,
            entry_type: row.try_get("entry_type")?,
            ip_address: row.try_get("ip_address")?,
            created: row.try_get("created")?,
        })
    }
}

impl Reply for UserEntry {
    fn into_response(self) -> warp::reply::Response {
        let json = json!({
            "id": self.id,
            "email": self.email,
            "entry_type": self.entry_type,
            "ip_address": self.ip_address,
            "created": self.created,
        });

        warp::reply::json(&json).into_response()
    }
}

impl UserEntry {
	pub fn count_user_entries_query(
		entry_type: UserEntryType,
		email: Option<&str>,
		address: Option<&str>
	) -> String {
		let mut select = sql::Select::new()
			.select("COUNT(*) AS count")
			.from("user_entries ue")
			.where_clause(format!(
				"ue.entry_type = '{}'",
				entry_type.to_str()
			).as_str());

		if let (Some(user_email), Some(user_address)) = email, address {
			select.where_clause("ue.email = '{}' OR ue.ip_address = '{}'")
		} else if let Some(user_email) = email {

		} else if let Some(user_address) = address {

		}

		select.to_string()
	}
}