use askama::Template;
use warp::{path, Filter, Rejection, Reply};

use crate::client::component_props::{FooterProps, MetaProps, NavbarProps};
use crate::client::translations::{get_translation, Language, TranslationKeys};

pub struct IndexPageData<'a> {
    title: &'a str,
    hero_banner: &'a str,
    hero_banner_alt: &'a str,
    author_picture: &'a str,
    author_picture_alt: &'a str,
    author_description: &'a str,
}

#[derive(Template)]
#[template(path = "./index.html", escape = "none")]
pub struct IndexPage<'a> {
    meta: MetaProps<'a>,
    page: IndexPageData<'a>,
    navbar: NavbarProps<'a>,
    footer: FooterProps<'a>,
}

pub async fn get_template(language: Language) -> Result<impl Reply, Rejection> {
    let page_data: IndexPageData = IndexPageData {
        title: get_translation(TranslationKeys::IndexTitle, language),
        hero_banner: "http://static.localhost/images/hero_baner",
        hero_banner_alt: get_translation(TranslationKeys::IndexHeroAlt, language),
        author_picture: "http://static.localhost/images/author_home",
        author_picture_alt: get_translation(TranslationKeys::IndexPicturAlt, language),
        author_description: &get_translation(TranslationKeys::IndexDescription, language)
            .replace("_", "&nbsp;"),
    };

    let mut meta_props = MetaProps::default(Some(language));
    meta_props.url = format!("www.rosemary-artist.com/{}/gallery", language.to_str());
    meta_props.description = get_translation(TranslationKeys::IndexMetaDescription, language);
    meta_props.keywords = get_translation(TranslationKeys::IndexMetaKeywords, language);
    meta_props.summary_large_image =
        get_translation(TranslationKeys::IndexMetaImageSummary, language);

    let template = IndexPage {
        meta: meta_props,
        page: page_data,
        navbar: NavbarProps::default(Some(language)),
        footer: FooterProps::default(Some(language)),
    };

    let result = template
        .render()
        .unwrap_or_else(|_| String::from("<h1>Internal Server Error</h1>"));

    Ok(warp::reply::html(result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
        .and_then(|| async { get_template(Language::Cs).await })
}

pub fn get_cz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::Cs.to_str()))
        .and(path::end())
        .and_then(|| async { get_template(Language::Cs).await })
}

pub fn get_en() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::En.to_str()))
        .and(path::end())
        .and_then(|| async { get_template(Language::En).await })
}
