use warp::Filter;
use warp::Buf;
use bytes::BufMut;
use futures_util::TryStreamExt;
// use warp::http::StatusCode;
// use warp::multipart::{FormData, Part};
// use sha2::{Sha256, Digest};

async fn post_file(form: warp::multipart::FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let field_names: Vec<_> = form
        .and_then(|mut field| async move {
            let mut bytes: Vec<u8> = Vec::new();

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

    Ok(format!("{:?}", field_names))
}

pub fn post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("file"))
        .and(warp::multipart::form())
        .and_then(post_file)
}