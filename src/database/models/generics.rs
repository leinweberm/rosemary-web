use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
	pub en: String,
	pub cs: String,
}

pub fn deserialize_json_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
	D: serde::Deserializer<'de>,
	T: serde::de::DeserializeOwned,
{
	let json_str: Option<String> = Option::deserialize(deserializer)?;

	match json_str {
		Some(s) => serde_json::from_str(&s)
			.map(Some)
			.map_err(serde::de::Error::custom),
		None => Ok(None),
	}
}