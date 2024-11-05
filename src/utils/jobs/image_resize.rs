use std::sync::Arc;
use std::path::Path;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use uuid::Uuid;
use tokio::sync::{mpsc, Semaphore};
use image::{DynamicImage, GenericImageView, ImageFormat};
use lazy_static::lazy_static;
use tokio::sync::OnceCell;

use crate::config::load::{get, ConfigField};

lazy_static! {
    pub static ref IMAGE_JOBS: OnceCell<Arc<ImageJobProcessor>> = OnceCell::new();
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
    semaphore: Arc<Semaphore>,
    job_sender: mpsc::Sender<ImageJobDefinition>,
    pub job_receiver: mpsc::Receiver<ImageJobDefinition>,
    is_working: Arc<AtomicBool>,
}

impl ImageJobProcessor {
    pub fn new(concurrent_tasks: usize, queue_size: usize) -> Self {
        let (job_sender, job_receiver) = mpsc::channel(queue_size);
        let working = Arc::new(AtomicBool::new(true));

        let mut processor = Self {
            semaphore: Arc::new(Semaphore::new(concurrent_tasks)),
            is_working: working.clone(),
            job_sender,
            job_receiver,
        };

        processor.spawn_processor();

        processor
    }

    pub async fn submit_job(&self, job: ImageJobDefinition) -> Result<(), Box<dyn Error>> {
        self.job_sender.send(job).await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub fn shutdown(&self) {
        self.is_working.store(false, Ordering::SeqCst);
    }

    fn spawn_processor(&mut self) {
        let semaphore = self.semaphore.clone();
        let is_running = self.is_working.clone();
        let job_receiver = self.job_receiver.clone();

        tokio::spawn(async move {
            self.run_processing_loop(semaphore, is_running, job_receiver).await;
        });
    }

    async fn run_processing_loop(
        &mut self,
        semaphore: Arc<Semaphore>,
        is_running: Arc<AtomicBool>,
        mut job_receiver: mpsc::Receiver<ImageJobDefinition>,
    ) {
        while is_running.load(Ordering::SeqCst) {
            match job_receiver.recv().await {
                Some(job) => {
                    let worker = semaphore.clone();
                    tokio::spawn(async move {
                        let permit = worker.acquire_owned().await.unwrap();
                        let result = ImageJobData::process_image(job).await;
                        if !result {
                            error!(target: "images", "Failed to process image job");
                        }
                        drop(permit);
                    });
                }
                None => {
                    debug!(target: "images", "Job channel closed, checking for new jobs");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    if let Err(e) = self.check_and_restart_processor().await {
                        error!(target: "images", "Error checking for new jobs: {}", e);
                    }
                }
            }
        }
    }

    async fn check_and_restart_processor(&mut self) -> Result<(), Box<dyn Error>> {
        if let Ok(Some(_)) = self.job_receiver.try_recv() {
            self.spawn_processor();
        }
        Ok(())
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
    IMAGE_JOBS.set(Arc::new(ImageJobProcessor::new(1, 100)))
        .expect("Failed to set image job processor");
    Ok(())
}

pub async fn get_image_jobs() -> Result<Arc<ImageJobProcessor>, Box<dyn Error>> {
    IMAGE_JOBS.get()
        .ok_or_else(|| "Image processor not initialized".into())
        .map(|processor| processor.clone())
}