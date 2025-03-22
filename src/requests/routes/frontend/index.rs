use askama::Template;
use warp::{path, Filter, Rejection, Reply};

use crate::client::translations::{get_translation, Language, TranslationKeys};
use crate::requests::routes::frontend::common_dto::MetaProps;

pub struct IndexPageData<'a> {
    title: &'a str,
    hero_banner: &'a str,
    hero_banner_alt: &'a str,
    author_picture: &'a str,
    author_picture_alt: &'a str,
    author_description: &'a str,
}

#[derive(Template)]
#[template(path = "./index.html")]
pub struct IndexPage<'a> {
    meta: MetaProps<'a>,
    page: IndexPageData<'a>,
}

pub async fn get_template(locale: i8) -> Result<impl Reply, Rejection> {
    let language = if locale == 0 {
        Language::Cs
    } else {
        Language::En
    };

    let page_data: IndexPageData = IndexPageData {
        title: get_translation(TranslationKeys::IndexTitle, language),
        hero_banner: "http://static.localhost/images/herobaner_1920.jpeg",
        hero_banner_alt: get_translation(TranslationKeys::IndexHeroAlt, language),
        author_picture: "http://static.localhost/images/author_home.jpeg",
        author_picture_alt: get_translation(TranslationKeys::IndexPicturAlt, language),
        author_description: &get_translation(TranslationKeys::IndexDescription, language)
            .replace("_", "&nbsp;"),
    };

    let meta_props: MetaProps = MetaProps {
        description: get_translation(TranslationKeys::IndexMetaDescription, Language::Cs),
        keywords: get_translation(TranslationKeys::IndexMetaKeywords, Language::Cs),
        author: "Rosemary - Michaela Halásová",
        robots: "index, follow",
        image: "http://static.localhost/images/herobaner_640.jpeg",
        locale: language.to_string(),
        favicon: "",
        url: &format!("www.rosemary-artist.com/{}", language.to_string()),
        twitter_handle: "",
        summary_large_image: get_translation(TranslationKeys::IndexMetaImageSummary, Language::Cs),
    };

    let template = IndexPage {
        meta: meta_props,
        page: page_data,
    };

    let result = template
        .render()
        .unwrap_or_else(|_| String::from("<h1>Internal Server Error</h1>"));

    Ok(warp::reply::html(result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
        .and_then(|| async { get_template(0).await })
}

pub fn get_cz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("cz"))
        .and(path::end())
        .and_then(|| async { get_template(0).await })
}

pub fn get_en() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("en"))
        .and(path::end())
        .and_then(|| async { get_template(1).await })
}
