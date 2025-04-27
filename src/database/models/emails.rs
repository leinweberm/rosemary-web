use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;
use sqlx::{postgres::PgRow, prelude::FromRow};
use warp::Reply;

use crate::database::models::user_entries::{UserEntry, UserEntryType};

#[derive(Serialize)]
pub struct EmailAddress {
    email: String,
}

#[derive(Serialize)]
pub struct PersonalizationData {
    body: String,
    name: String,
    email: String,
    phone: String,
    subject: String,
}

#[derive(Serialize)]
pub struct PersonalizationEntry {
    email: String,
    data: PersonalizationDataWrapper,
}

#[derive(Serialize)]
pub struct PersonalizationDataWrapper {
    data: PersonalizationData,
}

#[derive(Serialize)]
pub struct EmailRequest {
    from: EmailAddress,
    to: Vec<EmailAddress>,
    personalization: Vec<PersonalizationEntry>,
    template_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailCreate<'a> {
    pub subject: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub body: &'a str,
    pub ip_address: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub id: i64,
    pub subject: String,
    pub email: String,
    pub phone: String,
    pub body: String,
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
            "created": self.created,
        });

        warp::reply::json(&json).into_response()
    }
}

impl Email {
    pub fn count_user_entries_query(email: &str, address: &str) -> String {
        let ip_address = if address != "unknown" {
            Some(address)
        } else {
            None
        };
        UserEntry::count_user_entries_query(UserEntryType::ContactForm, Some(email), ip_address)
    }

    pub fn create_email_query(data: EmailCreate) -> String {
        let query = format!(
            r#"
		INSERT INTO rosemary.emails(subject, email, body, phone, ip_address)
		VALUES ('{}', '{}', '{}', '{}', '{}')
		RETURNING *;
		"#,
            data.subject, data.email, data.body, data.phone, data.ip_address,
        );

        query
    }
}