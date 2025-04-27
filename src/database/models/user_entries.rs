use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sql_query_builder as sql;
use sqlx::Row;
use sqlx::{postgres::PgRow, prelude::FromRow};
use warp::Reply;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserEntryCount {
    pub count: i64,
}

impl<'r> FromRow<'r, PgRow> for UserEntryCount {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            count: row.try_get("count")?,
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
    pub created: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for UserEntry {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            email: row.try_get("email")?,
            entry_type: row.try_get("entry_type")?,
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
            "created": self.created,
        });

        warp::reply::json(&json).into_response()
    }
}

impl UserEntry {
    pub fn count_user_entries_query(
        entry_type: UserEntryType,
        email: Option<&str>,
        address: Option<&str>,
    ) -> String {
        let mut select = sql::Select::new()
            .select("COUNT(*) AS count")
            .from("rosemary.user_entries ue")
            .where_clause(format!("ue.entry_type = '{}'", entry_type.to_str()).as_str())
            .where_and("ue.created >= NOW() - INTERVAL '24 hours'");

        if let (Some(user_email), Some(user_address)) = (email, address) {
            select = select.where_and(
                format!(
                    "ue.email = '{}' OR ue.ip_address = '{}'",
                    user_email, user_address
                )
                .as_str(),
            );
        } else if let Some(user_email) = email {
            select = select.where_and(format!("ue.email = '{}'", user_email).as_str());
        } else if let Some(user_address) = address {
            select = select.where_and(format!("use.ip_address = '{}'", user_address).as_str());
        }

        select.to_string()
    }

    pub fn create_user_entry_query(
        entry_type: UserEntryType,
        email: Option<&str>,
        address: &str,
    ) -> String {
        let user_email = if let Some(u_email) = email {
            u_email
        } else {
            "unknown"
        };

        format!(
            r#"
			INSERT INTO rosemary.user_entries(email, ip_address, entry_type)
			VALUES('{}', '{}', '{}')
			RETURNING *;
		"#,
            user_email,
            address,
            entry_type.to_str()
        )
    }
}
