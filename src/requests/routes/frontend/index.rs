use warp::{Filter, Rejection, Reply, path};
use askama::Template;

use crate::client::index;

pub async fn get_template() -> Result<impl Reply, Rejection> {
    let template = index::Page {};
    let result = template.render().unwrap_or_else(|_| String::from("<h1>Internal Server Error</h1>"));

    Ok(warp::reply::html(result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
        .and_then(get_template)
}