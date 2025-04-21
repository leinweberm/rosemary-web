use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sql_query_builder as sql;
use sqlx::Row;
use sqlx::{postgres::PgRow, prelude::FromRow};
use warp::Reply;

use crate::database::models::user_entries::UserEntry;

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailCreate {
    pub subject: String,
    pub email: String,
    pub phone: String,
    pub body: String,
    pub ip_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub id: i64,
    pub subject: String,
    pub email: String,
    pub phone: String,
    pub body: String,
    pub ip_address: String,
    pub created: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for Email {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            subject: row.try_get("subject")?,
            email: row.try_get("email")?,
            phone: row.try_get("phone")?,
            body: row.try_get("body")?,
            ip_address: row.try_get("ip_address")?,
            created: row.try_get("created")?,
        })
    }
}

impl Reply for Email {
    fn into_response(self) -> warp::reply::Response {
        let json = json!({
            "id": self.id,
            "subject": self.subject,
            "email": self.email,
            "phone": self.phone,
            "body": self.body,
            "ip_address": self.ip_address,
            "created": self.created,
        });

        warp::reply::json(&json).into_response()
    }
}

impl Email {
    pub fn count_user_entries_query(email: &str, address: &str) -> String {
        let select = sql::Select::new()
            .select("COUNT(*) as count")
            .from("user_entries ue")
            .where_clause(format!("ue.email = '{}'", email).as_str())
            .where_or(format!("ue.ip_address = '{}'", address).as_str());

        select.to_string()
    }

    pub fn create_email_query(data: EmailCreate) -> String {
        let query = format!(
            r#"
		INSERT INTO emails(subject, email, body, phone, ip_address)
		VALUES ('{}', '{}', '{}', '{}', '{}');
		"#,
            data.subject, data.email, data.body, data.phone, data.ip_address,
        );

        query
    }
}
