use askama::Template;
use warp::{path, query, Filter, Rejection, Reply};

use crate::client::component_props::{FooterProps, MetaProps, NavbarProps};
use crate::client::translations::{get_translation, Language, TranslationKeys};
use crate::requests::dto::get_paintings_query::GetPaintingsQuery;

pub struct GalleryItemStub {
    preview: String,
    preview_alt: String,
    title: String,
    size: String,
    price: String,
}

pub struct GalleryPageData<'a> {
    created: &'a str,
    price: &'a str,
    width: &'a str,
    height: &'a str,
    title: &'a str,
    sold: &'a str,
    ascending: &'a str,
    descending: &'a str,
    gallery_item_stubs: Vec<GalleryItemStub>,
}

#[derive(Template)]
#[template(path = "./gallery.html")]
pub struct GalleryPage<'a> {
    meta: MetaProps<'a>,
    page: GalleryPageData<'a>,
    navbar: NavbarProps<'a>,
    footer: FooterProps<'a>,
}

pub async fn get_template(locale: i8, _query: GetPaintingsQuery) -> Result<impl Reply, Rejection> {
    let language = if locale == 0 {
        Language::Cs
    } else {
        Language::En
    };

    let page_data: GalleryPageData = GalleryPageData {
        created: get_translation(TranslationKeys::Created, language),
        price: get_translation(TranslationKeys::Price, language),
        width: get_translation(TranslationKeys::Width, language),
        height: get_translation(TranslationKeys::Height, language),
        title: get_translation(TranslationKeys::Title, language),
        sold: get_translation(TranslationKeys::Sold, language),
        ascending: get_translation(TranslationKeys::Ascending, language),
        descending: get_translation(TranslationKeys::Descending, language),
        gallery_item_stubs: Vec::new(),
    };

    let mut meta_props = MetaProps::default(Some(language));
    meta_props.url = format!("www.rosemary-artist.com/{}", language.to_str());

    let template = GalleryPage {
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
        .and(path("gallery"))
        .and(path::end())
        .and(query::<GetPaintingsQuery>())
        .and_then(|query| async { get_template(0, query).await })
}

pub fn get_cz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("cz"))
        .and(path("gallery"))
        .and(path::end())
        .and(query::<GetPaintingsQuery>())
        .and_then(|query| async { get_template(0, query).await })
}

pub fn get_en() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("en"))
        .and(path("gallery"))
        .and(path::end())
        .and(query::<GetPaintingsQuery>())
        .and_then(|query| async { get_template(1, query).await })
}
