use warp::{Filter, Rejection, Reply};
use bytes::BufMut;
use futures_util::TryStreamExt;
use warp::multipart::FormData;
use warp::Buf;
use warp::http::{ Response, StatusCode };

async fn post_upload(form: FormData) -> Result<impl Reply, Rejection> {
    let field_names: Vec<_> = form
        .and_then(| mut field | async move {
            let mut bytes: Vec<u8> = Vec::new();

            if !field.name().is_empty() {
                println!("field: {:?}", field.name());
            }
            if let Some(filename) = field.filename() {
                println!("filename: {:?}", filename);
            }

            // field.data() only returns a piece of the content, you should call over it until it replies None
            while let Some(content) = field.data().await {
                let content = content.unwrap();
                println!("Content: {:?}", std::str::from_utf8(content.chunk()));
                bytes.put(content);
            }
            Ok((
                field.name().to_string(),
                field.filename().unwrap().to_string(),
                String::from_utf8_lossy(&*bytes).to_string(),
            ))
        })
        .try_collect()
        .await
        .unwrap();

    Ok::<_, warp::Rejection>(
        Response::builder()
            .status(StatusCode::OK)
            .body(String::from(format!("{:?}", field_names)))
    )
}

pub fn post() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::multipart::form()
        .and(warp::path("upload"))
        .and_then(post_upload)
}