use image::{GenericImageView, ImageError, ImageFormat};
use std::io;
use std::path::Path;
use tokio::task;

use crate::config::load::{get, ConfigField};

#[derive(Debug)]
pub struct ResizeImageJob {
    pub max_sizes: Vec<u32>,
    pub path: String,
    pub new_name: Option<String>,
}

async fn resize_to_max_image(path: &str, new_path: String, size: u32) -> Result<(), ImageError> {
    debug!(target: "app", "images:resize - path {}", &path);
    debug!(target: "app", "images:resize - new path {}", &new_path);
    debug!(target: "app", "images:resize - new size {}", &size);
    let path_string = path.to_string();
    let new_path_string = new_path.clone();
    let path_string_clone_1 = path_string.clone();
    let image_file = task::spawn_blocking(move || image::open(path_string_clone_1)).await;
    let file = match image_file {
        Ok(Ok(image)) => image,
        Ok(Err(image_error)) => {
            error!(target: "app", "images:resize - failed to open original file {}", image_error);
            return Err(image_error);
        }
        Err(join_error) => {
            error!(target: "app", "image:resize - failed to open original file {}", join_error);
            return Err(ImageError::IoError(io::Error::new(
                io::ErrorKind::Other,
                format!("{}", join_error),
            )));
        }
    };

    let (image_width, image_height) = file.dimensions();
    debug!(target: "app", "images:resize - original image size {} x {}", &image_width, &image_height);

    let mut max_width = 0;
    if image_width > image_height {
        debug!(target: "app", "images:resize - original image is landscape oriented");
        max_width = size;
    }
    let mut max_height = 0;
    if image_width < image_height {
        debug!(target: "app", "images:resize - original image is portrait oriented");
        max_height = size;
    }
    debug!(target: "app", "images:resize - max size {} x {}", &max_width, &max_height);

    let new_height;
    let new_width;
    if max_width > max_height {
        let ration = max_width as f32 / image_width as f32;
        new_height = (image_height as f32 * ration) as u32;
        new_width = (image_width as f32 * ration) as u32;
    } else {
        let ration = max_height as f32 / image_height as f32;
        new_height = (image_height as f32 * ration) as u32;
        new_width = (image_width as f32 * ration) as u32;
    }

    debug!(target: "app", "image:resize new file size {} x {}", &new_width, &new_height);

    let resized_file = task::spawn_blocking(move || {
        file.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    })
    .await;

    let resized = match resized_file {
        Ok(value) => {
            debug!(target: "app", "images:resize - resized image size {} x {}", &value.width(), &value.height());
            value
        }
        Err(join_error) => {
            error!(target: "app", "images:resize - failed to resize image {}", join_error);
            return Err(ImageError::IoError(io::Error::new(
                io::ErrorKind::Other,
                "failed to resize image",
            )));
        }
    };

    let resize_result = task::spawn_blocking(move || {
        resized
            .to_rgb8()
            .save_with_format(&new_path_string, ImageFormat::Jpeg)?;
        match image::open(&new_path_string) {
            Ok(_) => {
                debug!(target: "app", "images:resize - image created {}", &new_path_string);
            }
            Err(error) => {
                error!(target: "app", "images:resize - can not open resized image {}", error);
            }
        };
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await;

    match resize_result {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(image_error)) => {
            error!(target: "app", "images:resize - failed to save resized image {}", image_error);
            Err(ImageError::IoError(io::Error::new(
                io::ErrorKind::Other,
                format!("{}", image_error),
            )))
        }
        Err(join_error) => {
            error!(target: "app", "images:resize - failed to save resized image {}", join_error);
            Err(ImageError::IoError(io::Error::new(
                io::ErrorKind::Other,
                format!("{}", join_error),
            )))
        }
    }
}

pub async fn resize_to_max(mut job: ResizeImageJob) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];

    job.max_sizes.sort_by(|a, b| b.cmp(a));
    debug!(target: "app", "images:resize - new job {:?}", &job);

    let static_dir_path = match get::<String>(ConfigField::StaticFilesDir).await {
        Ok(value) => {
            debug!(target: "app", "images:resize - got static file dir path {}", &value);
            value
        }
        Err(error) => {
            error!(target: "app", "images:resize - failed to get static file dir {}", error);
            for _width in job.max_sizes.iter() {
                result.push(false);
            }
            return result;
        }
    };
    debug!(target: "app", "images:resize - static file dir path {}", static_dir_path);

    let mut last_resized_path: Option<String> = None;

    for width in job.max_sizes.into_iter() {
        debug!(target: "app", "images:resize - starting resizing to width {}", width);
        let new_file_path: String;

        if let Some(name) = &job.new_name {
            let temp_new_file_path =
                Path::new(&static_dir_path).join(format!("images/{}_{}.jpeg", &name, width));
            new_file_path = temp_new_file_path.to_string_lossy().to_string();
        } else {
            let current_file_path = Path::new(&job.path);

            let current_file_name = current_file_path
                .file_stem()
                .and_then(|name| name.to_str().map(|s| s.to_string()))
                .unwrap_or_else(|| String::new());
            if current_file_name.is_empty() {
                result.push(false);
                continue;
            }

            let temp_new_file_path = Path::new(&static_dir_path)
                .join(format!("images/{}_{}.jpeg", &current_file_name, width));
            new_file_path = temp_new_file_path.to_string_lossy().to_string();
        }

        if let Some(saved_path) = &last_resized_path {
            job.path = saved_path.clone();
        }
        debug!(target: "app", "images:resize - new resizing to width {}", width);
        debug!(target: "app", "images:resize - new file path {:?}", &new_file_path);
        debug!(target: "app", "images:resize - olf file path {:?}", &last_resized_path);

        match resize_to_max_image(&job.path, new_file_path.clone(), width).await {
            Ok(_) => {
                debug!(target: "app", "images:resize_to_max - successfully resized to {}", width);
                result.push(true);
            }
            Err(error) => {
                error!(target: "app", "images:resize_to_max - failed to resize {} {}", width, error);
                result.push(false);
            }
        };

        last_resized_path = Some(new_file_path.clone());
    }

    result
}
