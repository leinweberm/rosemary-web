use std::sync::Arc;
use std::path::Path;
use std::error::Error;
use uuid::Uuid;
use tokio::sync::{mpsc, Semaphore};
use image::{DynamicImage, GenericImageView, ImageFormat};

use crate::config::load::{get, ConfigField};

#[derive(Debug)]
struct ImageJob {
	id: Uuid,
	file_name: String,
	file_path: String,
}

#[derive(Debug)]
struct ImageJobProcessor {
	semaphore: Arc<Semaphore>,
	job_sender: mpsc::Sender<ImageJob>
}

impl ImageJobProcessor {
	fn new(queue_size: usize) -> (Self, mpsc::Receiver<ImageJob>) {
		let (job_sender, job_receiver) = mpsc::channel(queue_size);

		(ImageJobProcessor {
			semaphore:Arc::new(Semaphore::new(1)),
			job_sender
		}, job_receiver)
	}

	async fn process_job(&self, job: ImageJob) -> Result<(), Box<dyn Error>> {
		let permit = self.semaphore.acquire().await?;

		let _ = tokio::task::spawn_blocking(move || {
			let processing_result = ImageProcessor::process_image(job.file_name, job.file_path);
			processing_result
		}).await;

		drop(permit);
		Ok(())
	}

	fn get_job_sender(&self) -> mpsc::Sender<ImageJob> {
		self.job_sender.clone()
	}
}

struct ImageProcessor {
	image: DynamicImage
}

impl ImageProcessor {
	async fn process_image (name: String, path: String) -> bool {
		let widths = vec![320, 640, 1080, 1920];

		let static_dir_path = match get::<String>(ConfigField::StaticFilesDir).await {
			Ok(value) => {
				debug!(target: "images", "images:processor - static files directory loaded");
				value
			},
			Err(error) => {
				error!(target: "images", "images:processor- failed to load static files directory {}", error);
				return false;
			}
		};

		let image_file = match image::open(path) {
			Ok(value) => {
				debug!(target: "images", "images:processor - load file successfuly");
				value
			},
			Err(error) => {
				error!(target: "images", "images:processor - failed to load image {}", error);
				return false;
			}
		};

		let (image_width, image_height) = image_file.dimensions();

		for width in widths.into_iter() {
			debug!(target: "images", "images:processor - procesing size {}", &width);

			let mut max_width = 0;
			if image_width > image_height {
				max_width = width;
			}

			let mut max_height = 0;
			if image_width < image_height {
				max_height = width;
			}

			let resized = image_file.resize_to_fill(
				max_width,
				max_height,
				image::imageops::FilterType::Lanczos3
			);

			let file_name = Path::new(&name)
				.file_stem()
				.and_then(|s| s.to_str())
				.unwrap_or("");

			let final_path = Path::new(&static_dir_path)
				.join(format!("images/{}_{}", &file_name, &width));

			match resized.save_with_format(final_path, ImageFormat::Jpeg) {
				Ok(_value) => {
					debug!(target: "images", "images:processor - image with size {} resized and saved", &width);
				},
				Err(error) => {
					error!(target: "error", "images:processor - failed saving resized image {}", error);
					return false;
				}
			};
		};

		true
	}
}
