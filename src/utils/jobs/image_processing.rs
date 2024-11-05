use std::sync::Arc;
use std::path::Path;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use uuid::Uuid;
use tokio::sync::{mpsc, Semaphore, Mutex};
use image::{DynamicImage, GenericImageView, ImageFormat};
use lazy_static::lazy_static;
use tokio::sync::OnceCell;

use crate::config::load::{get, ConfigField};

lazy_static! {
	pub static ref IMAGE_JOBS: OnceCell<Arc<ImageJobManager>> = OnceCell::new();
}

#[derive(Debug)]
struct ImageJobDefinition {
	id: Uuid,
	file_name: String,
	file_path: String,
}

struct ImageJobData {
	image: DynamicImage
}

#[derive(Debug)]
struct ImageJobProcessor {
	semaphore: Semaphore,
	job_sender: mpsc::Sender<ImageJobDefinition>,
	job_receiver: mpsc::Receiver<ImageJobDefinition>,
	is_working: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct ImageJobManager {
	inner: Arc<Mutex<ImageJobProcessor>>
}

impl ImageJobManager {
	pub fn new (queue_size: usize) -> Self {
		let image_processor = ImageJobProcessor::new(1, queue_size);
		Self {
			inner: Arc::new(Mutex::new(image_processor))
		}
	}

	pub async fn submit_job(self, job: ImageJobDefinition) -> Result<(), Box<dyn Error>> {
		let processor_guard = self.inner.lock().await;
		let processor = &*processor_guard;

		if !processor.is_working.load(Ordering::SeqCst) {
			processor.is_working.store(true, Ordering::SeqCst);
			processor_guard.job_sender.send(job).await?;
			drop(processor_guard);
			self.start_processor();
		} else {
			processor_guard.job_sender.send(job).await?;
		}

		Ok(())
	}

	async fn start_processor(self) {
		let processor = self.inner.clone();
		tokio::spawn(async move {
			self.loop_jobs(processor);
		});
	}

	async fn loop_jobs(self, processor: Arc<Mutex<ImageJobProcessor>>) {
		let processor_guard = processor.clone();
		let mut empty_loops = 0;

		while empty_loops < 5 {
			let processor_inner_guard = processor_guard.clone();
			let mut processor = processor_inner_guard.lock().await;
			if !processor.is_working.load(Ordering::SeqCst) {
				break;
			}

			match processor.job_receiver.recv().await {
				Some(job) => {
					debug!(target: "app", "images:job - found job to process {:?}", &job);
					empty_loops = 0;
					let processor_innest_guard = processor_inner_guard.clone();

					tokio::spawn(async move {
						let processor_moved_guard = processor_innest_guard.clone();
						let processor_moved = processor_moved_guard.lock().await;
						let worker = &processor_moved.semaphore;
						let permit = worker.try_acquire();
						let result = ImageJobData::process_image(job).await;

						if result {
							debug!(target: "app", "images:jobs - image processing success");
						} else {
							debug!(target: "app", "images:jobs - image processing failed");
						}

						drop(permit);
					});
				},
				None => {
					debug!(target: "app", "images:jobs - no jobs waiting to be processed");
					tokio::time::sleep(Duration::from_secs(5)).await;
					empty_loops += 1;
				}
			};
		}

		let processor_final = processor_guard.lock().await;
		processor_final.is_working.store(false, Ordering::SeqCst);
	}
}

impl ImageJobProcessor {
	pub fn new(concurrent_tasks: usize, queue_size: usize) -> Self {
		let (job_sender, job_receiver) = mpsc::channel(queue_size);
		let working = Arc::new(AtomicBool::new(false));

		 Self {
			semaphore: Semaphore::new(concurrent_tasks),
			job_receiver,
			job_sender,
			is_working: working.clone(),
		}
	}
}

impl ImageJobData {
	async fn process_image (job: ImageJobDefinition) -> bool {
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

		let image_file = match image::open(job.file_path) {
			Ok(value) => {
				debug!(target: "images", "images:processor - load file successfully");
				value
			},
			Err(error) => {
				error!(target: "images", "images:processor - failed to load image {}", error);
				return false;
			}
		};

		let (image_width, image_height) = image_file.dimensions();

		for width in widths.into_iter() {
			debug!(target: "images", "images:processor - processing size {}", &width);

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

			let file_name = Path::new(&job.file_name)
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

pub async fn init_image_jobs() -> Result<(), Box<dyn Error>> {
	IMAGE_JOBS.set(Arc::new(ImageJobManager::new(200)))
		.expect("Failed to set image processor");
	Ok(())
}

pub async fn get_image_jobs() -> Result<Arc<ImageJobManager>, Box<dyn Error>> {
	IMAGE_JOBS.get()
		.ok_or_else(|| "Image processor not initialized".into())
		.map(|processor| processor.clone())
}