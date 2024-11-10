use std::path::Path;
use std::sync::Arc;
use bytes::Buf;
use futures_util::TryStreamExt;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::{path, Filter, Rejection, Reply, query};
use warp::filters::multipart::FormData;

use crate::database::connection::get_client;
use crate::database::models::image::{PaintingImage, PaintingImageCreate};
use crate::errors::api_error::{ImageResizeError, InternalServerError};
use crate::requests::dto::generic_response::{GenericResponse, Status};
use crate::utils::auth::token::{jwt_auth, Claims};
use crate::config::load::{ConfigField, get};
use crate::utils::file_system::fs_write::append_bytes;
use crate::utils::images::resize_to_max::{ResizeImageJob, resize_to_max};

#[derive(Serialize)]
struct FormDataRecord {
	pub name: String,
	pub file_name: String,
	pub size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageMetaQuery {
	pub preview: Option<bool>,
	pub title_cs: Option<String>,
	pub title_en: Option<String>,
	pub alt_cs: Option<String>,
	pub alt_en: Option<String>,
	pub painting_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageProcessData {
	pub file_name: String,
	pub file_path: String,
	pub file_system_path: String,
	pub size: usize,
	pub preview: bool,
	pub title_cs: String,
	pub title_en: String,
	pub alt_cs: String,
	pub alt_en: String,
	pub painting_id: Uuid,
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
		file_system_path: String::from(""),
		size: 0,
		preview: params.preview.unwrap_or(false),
		title_cs: params.title_cs.unwrap_or(String::from("")),
		title_en: params.title_en.unwrap_or(String::from("")),
		alt_cs: params.alt_cs.unwrap_or(String::from("")),
		alt_en: params.alt_en.unwrap_or(String::from("")),
		painting_id: params.painting_id
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
			debug!(target: "api", "images:create - file_name {}", &file_name);
			let file_path = match get::<String>(ConfigField::StaticFilesDir).await {
				Ok(value) => {
					debug!(target: "api", "images:create - config file path {}", &value);
					Path::new(&value).join(format!("images/{}", &file_name))
				},
				Err(error) => {
					error!(target: "api", "images:create - path creating file path failed {:?}", error);
					errors_move_nested_lock.push(format!("Creating file path failed {:?}", error));
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
						debug!(target: "api", "images:create path {:?} data chunk processing", &file_path.to_str());
						let _ = append_bytes(
							value.chunk(),
							file_path.to_str().expect("NO filepath specified"),
							true
						).await;
					},
					Err(error) => {
						error!(target: "api", "images:create - processing failed to process file chunk {:?}", error);
						errors_move_nested_lock.push(format!("Failed to process file chunk {:?}", error));
						return Ok(())
					}
				}
			}

			Ok(())
		}
	}).await;

	let errors_main_lock = errors.lock().await;
	if !errors_main_lock.is_empty() {
		return Ok(warp::reply::with_status(
			warp::reply::json(&error_response),
			warp::http::StatusCode::INTERNAL_SERVER_ERROR)
		)
	}

	match processed {
		Ok(_) => {
			debug!(target: "api", "images:create - processing success")
		},
		Err(error) => {
			error!(target: "api", "images:create - processing error {:?}", error);
			return Ok(warp::reply::with_status(
				warp::reply::json(&error_response),
				warp::http::StatusCode::INTERNAL_SERVER_ERROR)
			)
		}
	};

	let image_data = processed_data.lock().await;

	let static_file_dir = match get::<String>(ConfigField::StaticFilesDir).await {
		Ok(path) => {
			path
		},
		Err(error) => {
			error!(target: "api", "images:create - error getting static dir path {}", error);
			return Ok(InternalServerError::new().response().await)
		}
	};

	let data = PaintingImageCreate {
		preview: image_data.preview,
		alt_cs: image_data.alt_cs.clone(),
		alt_en: image_data.alt_en.clone(),
		title_cs: image_data.title_cs.clone(),
		title_en: image_data.title_en.clone(),
		painting_id: image_data.painting_id
	};

	let query = PaintingImage::create_query(data);
	debug!(target: "db", "images:create - PaintingImage::create_query - {}", &query);
	let create_result = sqlx::query_as::<_, PaintingImage>(&query)
		.fetch_one(&*client).await;

	let mut painting = match create_result {
		Ok(painting_image) => {
			debug!(target: "api", "images:create - result {:?}", &painting_image);
						painting_image
		},
		Err(error) => {
			error!(target: "api", "images:create - error {:?}", error);
			return Ok(InternalServerError::new().response().await)
		}
	};

	let file_system_path = Path::new(&static_file_dir)
		.join(format!("images/{}", &image_data.file_name));

	let sizes = vec![320, 640, 1024, 1090];
	let image_job = ResizeImageJob {
		max_sizes: sizes.clone(),
		path: file_system_path.to_string_lossy().to_string(),
		new_name: Some(String::from(format!("{}", &painting.id))),
	};

	let resize_result = resize_to_max(image_job).await;
	let mut index = 0;
	let mut failed_resize = false;
	let mut resized_paths: Vec<String> = Vec::new();

	for result in resize_result.into_iter() {
		if !result {
			error!(target: "api", "images:create - resize failed {:?}", sizes[index]);
			failed_resize = true;
		} else {
			resized_paths.push(String::from(format!("images/{}_{}.jpeg", &painting.id, sizes[index])));
		}
		index += 1;
	}

	if failed_resize {
		return Ok(ImageResizeError::new().response().await)
	}

	if resized_paths.len() == 4 {
		let update_query = PaintingImage::update_resized_query(
			painting.id, static_file_dir.clone(), resized_paths);
		debug!(target: "api", "images:create - update resized query {}", &update_query);
		let update_result = sqlx::query_as::<_, PaintingImage>(&update_query)
			.fetch_one(&*client)
			.await;
		match update_result {
			Ok(value) => {
				debug!(target: "api", "images:create - updated resized image painting record");
				painting = value;
			},
			Err(error) => {
				error!(target: "api", "images:create - updating resized image record failed {}", error);
				return Ok(InternalServerError::new().response().await);
			}
		}
	}

	let response = GenericResponse::<PaintingImage> {
		status: Status::Success,
		message: "Painting image created successfully",
		data: Some(painting),
	};
	Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::CREATED))
}

pub fn create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::post()
		.and(path("api"))
		.and(path("v1.0"))
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