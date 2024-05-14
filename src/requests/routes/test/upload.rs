use warp::{ Filter, Rejection, Reply };
use futures_util::TryStreamExt;
use warp::multipart::FormData;
use warp::Buf;
use warp::http::{ Response, StatusCode };
use sha2::{ Sha256, Digest };
use std::sync::{Arc, Mutex};
use tokio::task;
use std;

async fn post_upload(form: FormData) -> Result<impl Reply, Rejection> {
    let result: Vec<_> = form
        .and_then(| mut field | async move {
            let mut hasher = Sha256::new();
            let mut size: usize = 0;

            let mut content_type = String::from("form-data/field");
            if field.content_type().is_some() {
                content_type = String::from(format!("{:?}", field.content_type()));
            }

            while let Some(content) = field.data().await {
                // println!("Content: {:?}", content.chunk().len());
                // size += content.chunk().len();
                // hasher.update(content.chunk());

                let part = content.unwrap();
                let chunk = Arc::new(Mutex::new(part.chunk()));
                let thread_1_chunk = Arc::clone(&chunk);
                let thread_2_chunk = Arc::clone(&chunk);

                let size_task = task::spawn(async {
                    let data = thread_1_chunk.lock().unwrap();
                    size += data.len();
                    std::thread::sleep(std::time::Duration::from_micros(500_000));
                });

                let hash_task = task::spawn(async {
                    let data = thread_2_chunk.lock().unwrap();
                    std::thread::sleep(std::time::Duration::from_micros(500_000));
                });

                size_task.await.unwrap();
                hash_task.await.unwrap();
            }

            let hash = hasher.finalize();

            Ok((
                format!("Content-Type: {}", content_type),
                format!("{:x}", hash),
                format!("{} MB", (&size / 1024 / 1024).to_string())
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