use warp::{Filter, Rejection, Reply, path};
use askama::Template;

use crate::client::index;
#[allow(dead_code)]
pub async fn get_template() -> Result<impl Reply, Rejection> {
    let template = index::Page {};
    let result = match template.render() {
        Ok(html) => html,
        Err(_) => String::from("<h1>Internal Server Error</h1>")
    };

    Ok(warp::reply::html(result))
}

#[allow(dead_code)]
pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
        .and_then(get_template)
}