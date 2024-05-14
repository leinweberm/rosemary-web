use warp::{ Filter, Rejection, Reply };
// use bytes::BufMut;
use futures_util::TryStreamExt;
use warp::multipart::FormData;
use warp::Buf;
use warp::http::{ Response, StatusCode };
use sha2::{ Sha256, Digest };

async fn post_upload(form: FormData) -> Result<impl Reply, Rejection> {
    let result: Vec<_> = form
        .and_then(| mut field | async move {
            // let mut bytes: Vec<u8> = Vec::new();
            let mut hasher = Sha256::new();
            println!("field content-type: {:?}", field.content_type());
            println!("field name: {:?}", field.name());

            if let Some(filename) = field.filename() {
                println!("filename: {:?}", filename);
            }

            // field.data() only returns a piece of the content, you should call over it until it replies None
            while let Some(content) = field.data().await {
                let content = content.unwrap();
                println!("Content: {:?}", content.chunk().len());
                hasher.update(content.chunk());
                // bytes.put(content);
            }

            let hash = hasher.finalize();

            Ok((
                // field.filename().unwrap().to_string(),
                format!("{:x}", hash),
                // String::from_utf8_lossy(&*bytes).to_string(),
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