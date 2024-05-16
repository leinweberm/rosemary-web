use warp::{ Filter, Rejection, Reply };
use futures_util::TryStreamExt;
use warp::multipart::FormData;
use warp::Buf;
use warp::http::{ Response, StatusCode };
use sha2::{ Sha256, Digest };
use std::sync::{Arc, Mutex};
// use tokio::task;
use std;

async fn post_upload(form: FormData) -> Result<impl Reply, Rejection> {
    let result: Vec<_> = form
        .and_then(| mut field | async move {
            let content_type = String::from("form-data/field");
            let hasher = Arc::new(Mutex::new(Sha256::new()));
            let size = Arc::new(Mutex::new(0));
            let task_counter = Arc::new(Mutex::new(0));

            while let Some(content) = field.data().await {
                let mut handles = vec![];

                let part = content.unwrap();
                let shared_part = Arc::new(Mutex::new(part.chunk().to_vec()));

                let size_task_counter = Arc::clone(&task_counter);
                let size_task_part = Arc::clone(&shared_part);
                let size_task_size = Arc::clone(&size);

                let hash_task_counter = Arc::clone(&task_counter);
                let hash_task_part = Arc::clone(&shared_part);
                let hash_task_hasher = Arc::clone(&hasher);

                let hash_task = std::thread::spawn(move || {
                    let mut counter = hash_task_counter.lock().unwrap();
                    let mut hasher = hash_task_hasher.lock().unwrap();
                    let part = hash_task_part.lock().unwrap();
                    *counter += 1;
                    hasher.update(part.as_slice());
                });

                handles.push(hash_task);

                let size_task = std::thread::spawn(move || {
                    let mut counter = size_task_counter.lock().unwrap();
                    let mut size = size_task_size.lock().unwrap();
                    let part = size_task_part.lock().unwrap();
                    *counter += 1;
                    *size += part.len() as u64;
                });

                handles.push(size_task);

                for handle in handles {
                    handle.join().unwrap();
                }
            }

            let hash: String;
            {
                let hasher = hasher.lock().unwrap();
                hash = format!("{:x}", hasher.clone().finalize());
            }

            let final_size;
            {
                let size = size.lock().unwrap();
                final_size = *size;
            }

            let task_count;
            {
                let counter = task_counter.lock().unwrap();
                task_count = *counter;
            }

            Ok((
                format!("Content-Type: {}", content_type),
                format!("hash: {}", hash),
                format!("size: {} MB", (&final_size / 1024 / 1024).to_string()),
                format!("task count: {}", task_count)
            ))
        })
        .try_collect()
        .await
        .unwrap();

    Ok::<_, warp::Rejection>(
        Response::builder()
            .status(StatusCode::OK)
            .body(String::from(format!("{:?}", result)))
    )
}

pub fn post() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::multipart::form()
        .and(warp::path("upload"))
        .and(warp::body::content_length_limit(1024 * 1024 * 20))
        .and_then(post_upload)
}