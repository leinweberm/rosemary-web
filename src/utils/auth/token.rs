#![allow(dead_code)]
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use tokio::sync::OnceCell;
use uuid::Uuid;
use warp::{reject::Rejection, Filter};
use warp::http::header::{HeaderMap, HeaderValue};
use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::load::{ConfigField, get};
use crate::database::connection::get_client;
use crate::database::models::user::User;
use crate::errors::api_error::{InternalServerError, TokenExpiredError, UnauthorizedError};

lazy_static! {
	pub static ref JWT_KEYS: OnceCell<JwtKeys> = OnceCell::new();
}

#[derive(Clone)]
pub struct JwtKeys {
	encoding: EncodingKey,
	decoding: DecodingKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub ui: Uuid,
	exp: usize,
}

impl JwtKeys {
	fn new(secret: &[u8]) -> Self {
		Self {
			encoding: EncodingKey::from_secret(secret),
			decoding: DecodingKey::from_secret(secret)
		}
	}
}

pub async fn set_keys() -> Result<JwtKeys, Error> {
	let config_jwt_secret = get::<String>(ConfigField::JwtSecret).await?;
	debug!(target: "auth", "jwt:keys secret from config {}", &config_jwt_secret);
	let secret = config_jwt_secret.as_bytes().to_vec();
	let keys = JwtKeys::new(&secret);
	let _ = JWT_KEYS.set(keys.clone());
	Ok(keys)
}

pub async fn get_keys() -> Result<&'static JwtKeys, Error> {
	debug!(target: "auth", "jwt:keys - getting static jwt keys reference");
	JWT_KEYS
		.get()
		.ok_or_else(||Error::new(std::io::ErrorKind::Other, "JWT keys do not exist"))
}

pub async fn encode_token(user_id: &str, expiration_in_seconds: usize, secret: &JwtKeys) -> Result<String, Rejection> {
	let expiration = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("Failed to set JWT token expiration")
		.as_secs() as usize + expiration_in_seconds;
	let expiration_copy = expiration;

	let current_time = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("Failed to get current time")
		.as_secs() as usize;

	let expires_in = expiration - current_time;

	let uid = match user_id.parse::<Uuid>() {
		Ok(value) => value,
		Err(error) => {
			error!(target: "auth", "jwt:encode - failed to parse User Id {}", error);
			return Err(warp::reject::custom(UnauthorizedError::new()))
		}
	};

	let claims = Claims {
		ui: uid,
		exp: expiration_copy,
	};

	let token = match encode(&Header::new(Algorithm::HS256), &claims, &secret.encoding) {
		Ok(value) => value,
		Err(error) => {
			error!(target: "auth", "jwt:encode- failed to encode JWT token {}", error);
			return Err(warp::reject::custom(UnauthorizedError::new()))
		}
	};

	let mut headers = HeaderMap::new();
	headers.insert("X-Access-Token-Expires", HeaderValue::from_str(&expires_in.to_string()).unwrap());
	headers.insert("X-Token-Valid-Until", HeaderValue::from_str(&expiration.to_string()).unwrap());

	Ok(token)
}

pub async fn create(user_id: &str, expiration_in_seconds: usize) -> Result<String, Rejection> {
	let secret = match get_keys().await {
		Ok(value) => value,
		Err(error) => {
			error!(target: "auth", "jwt:encode - failed to get JWT secrets {}", error);
			return Err(warp::reject::custom(UnauthorizedError::new()))
		}
	};
	encode_token(user_id, expiration_in_seconds, secret).await
}

pub fn extract_token(auth_header: String) -> String {
	if auth_header.starts_with("Bearer") {
		auth_header[7..].to_string()
	} else {
		auth_header
	}
}

pub async fn decode_token(token: String, secret: &JwtKeys) -> Result<Claims, Rejection> {
	match decode::<Claims>(
		&token,
		&secret.decoding,
		&Validation::new(Algorithm::HS256),
	) {
		Ok(token_data) => {
			let current_time = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.expect("Failed to get current unix time")
				.as_secs() as usize;

			if current_time > token_data.claims.exp {
				return Err(warp::reject::custom(TokenExpiredError::new()))
			}

			Ok(token_data.claims)
		},
		Err(error) => {
			error!(target: "auth", "jwt:decode - failed to decode JWT token {}", error);
			Err(warp::reject::custom(UnauthorizedError::new()))
		}
	}
}

pub async fn verify(token: String) -> Result<Claims, Rejection> {
	let secret = match get_keys().await {
		Ok(value) => value,
		Err(error) => {
			error!(target: "auth", "jwt:decode - failed to get JWT secrets {}", error);
			return Err(warp::reject::custom(InternalServerError::new()))
		}
	};

	let claims = match decode_token(token, secret).await {
		Ok(value) => {
			let client = get_client().await.unwrap().clone();
			let query = User::get_by_id_query(value.ui);
			let find_user = sqlx::query_as::<_, User>(&query)
				.fetch_one(&client)
				.await;

			match find_user {
				Ok(user) => {
					if user.user_uid.is_nil() || user.deleted.is_some() {
						return Err(warp::reject::custom(UnauthorizedError::new()))
					}
				},
				Err(error) => {
					error!(target: "auth", "jwt:decode - failed to find user - {}", error);
					return Err(warp::reject::custom(UnauthorizedError::new()))
				}
			}
			value
		},
		Err(error) => {
			return Err(error)
		}
	};

	Ok(claims)
}

pub fn jwt_auth() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
	warp::header::<String>("Authorization")
		.map(extract_token)
		.and_then(|token: String| verify(token))
}

#[cfg(test)]
mod tests {
	use super::*;
	use uuid::Uuid;

	#[tokio::test]
	async fn test_jwt_token() {
		let jwt_secret = String::from("TotoJeTestJwtSecret");
		let secret = jwt_secret.as_bytes().to_vec();
		let keys = JwtKeys::new(&secret);
		let expiration = 3600usize;
		let user_id = Uuid::new_v4();
		assert!(!user_id.is_nil());

		let token = encode_token(&user_id.to_string(), expiration, &keys).await.unwrap();
		assert!(!token.is_empty());

		let decoded_token = decode_token(token, &keys).await.unwrap();
		assert_eq!(&decoded_token.ui, &user_id);

		let current_time = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Failed to get current unix time")
			.as_secs() as usize;

		let future_time = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("Failed to set JWT token expiration")
		.as_secs() as usize + 6969usize;

		let more_then_current = decoded_token.exp > current_time;
		let less_then_future = decoded_token.exp < future_time;

		assert_eq!(more_then_current, true);
		assert_eq!(less_then_future, true);
	}
}