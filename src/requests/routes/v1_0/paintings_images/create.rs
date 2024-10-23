use std::path::Path;
use std::sync::Arc;
use bytes::Buf;
use futures_util::TryStreamExt;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use warp::{path, Filter, Rejection, Reply, query};
use warp::filters::multipart::FormData;

use crate::database::connection::get_client;
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};
use crate::config::load::{ConfigField, get};
use crate::utils::file_system::fs_write::append_bytes;

#[derive(Serialize)]
struct FormDataRecord {
	pub name: String,
	pub file_name: String,
	pub size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageMetaQuery {
	pub preview: Option<bool>,
	pub title: Option<String>,
	pub alt: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageProcessData {
	pub file_name: String,
	pub file_path: String,
	pub size: usize,
	pub preview: bool,
	pub title: String,
	pub alt: String,
}

async fn create_painting_image(data: FormData, params: ImageMetaQuery) -> Result<impl Reply, Rejection> {
	let errors: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
	let errors_clone = Arc::new(&errors);
	let client = get_client().await.unwrap();

	let error_response = GenericResponse::<Vec<FormDataRecord>> {
		status: Status::Error,
		message: "internalServerError",
		data: None,
	};

	let processed_data: Arc<Mutex<ImageProcessData>> = Arc::new(Mutex::new(ImageProcessData {
		file_name: String::from(""),
		file_path: String::from(""),
		size: 0,
		preview: params.preview.unwrap_or(false),
		title: params.title.unwrap_or(String::from("")),
		alt: params.alt.unwrap_or(String::from("")),
	}));
	let processed_data_clone = Arc::clone(&processed_data);

	let processed = data.try_for_each(|data_field: warp::filters::multipart::Part| {
		let mut field = data_field;
		let errors_move_clone = Arc::clone(&errors_clone);
		let process_data_move_clone = Arc::clone(&processed_data_clone);

		async move {
			let errors_move_nested_clone = Arc::clone(&errors_move_clone);
			let mut errors_move_nested_lock = errors_move_nested_clone.lock().await;

			let process_data_move_nested_clone = Arc::clone(&process_data_move_clone);
			let mut process_data_move_nested_lock = process_data_move_nested_clone.lock().await;

			let file_name: &str = field.filename().unwrap_or("unknown");
			let file_path = match get::<String>(ConfigField::StaticFilesDir).await {
				Ok(value) => Path::new(&value).join(format!("/images/{}", &file_name)),
				Err(error) => {
					error!(target: "api", "Creating file path failed - {}", error);
					errors_move_nested_lock.push(format!("Creating file path failed - {}", error));
					return Ok(())
				}
			};

			process_data_move_nested_lock.file_name = file_name.to_string();
			process_data_move_nested_lock.file_path = String::from(
				format!("https://static.rosemary-artist.com/static/images/{}", &file_name)
			);

			while let Some(content) = field.data().await {
				match content {
					Ok(value) => {
						process_data_move_nested_lock.size += value.chunk().len();
						let _ = append_bytes(value.chunk(), file_path.to_str().expect("NO filepath specified"), true).await;
					},
					Err(error) => {
						error!(target: "api", "Failed to process file chunk - {}", error);
						errors_move_nested_lock.push(format!("Failed to process file chunk - {}", error));
						return Ok(())
					}
				}
			}

			Ok(())
		}
	}).await;

	let errors_main_lock = errors.lock().await;
	if !errors_main_lock.is_empty() {
		return Ok(warp::reply::with_status(warp::reply::json(&error_response), warp::http::StatusCode::INTERNAL_SERVER_ERROR))
	}

	match processed {
		Ok(_) => {
			let response = GenericResponse::<Vec<FormDataRecord>> {
				status: Status::Success,
				message: "fileUploaded",
				data: None,
			};
			Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::CREATED))
		},
		Err(_) => {
			Ok(warp::reply::with_status(warp::reply::json(&error_response), warp::http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}

pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::post()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("paintings"))
		.and(path("images"))
		.and(path::end())
		.and(query::<ImageMetaQuery>())
		.and(warp::multipart::form())
		.and(jwt_auth())
		.and_then(|params: ImageMetaQuery, data: FormData, _claims: Claims| async move {
			create_painting_image(data, params).await
		})
		.with(warp::log("api"))
}