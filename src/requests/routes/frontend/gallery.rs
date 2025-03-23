use askama::Template;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use warp::{path, query, Filter, Rejection, Reply};

use crate::client::component_props::{FooterProps, MetaProps, NavbarProps};
use crate::client::translations::{get_translation, Language, TranslationKeys};
use crate::database::connection::get_client;
use crate::database::models::painting::{Painting, PaintingStub};
use crate::requests::dto::get_paintings_query::GetPaintingsQuery;

#[derive(Debug)]
struct PaginationData<'a> {
    total: i64,
    current_start: u32,
    current_end: u64,
    prev_page: &'a str,
    next_page: &'a str,
}

#[derive(Debug)]
struct GalleryPageData<'a> {
    created: &'a str,
    price: &'a str,
    width: &'a str,
    height: &'a str,
    title: &'a str,
    sold: &'a str,
    ascending: &'a str,
    descending: &'a str,
    gallery_item_stubs: Vec<PaintingStub>,
    filter_form_action: &'a str,
}

#[derive(Template, Debug)]
#[template(path = "./gallery.html")]
struct GalleryPage<'a> {
    meta: MetaProps<'a>,
    page: GalleryPageData<'a>,
    navbar: NavbarProps<'a>,
    footer: FooterProps<'a>,
    pagination: PaginationData<'a>,
}

pub async fn get_template(
    language: Language,
    query: GetPaintingsQuery,
) -> Result<impl Reply, Rejection> {
    let language_string = language.to_string();
    let parsed_query = query.safe_parse(Some(language));

    let count_client: Arc<Pool<Postgres>> = Arc::new(get_client().await.unwrap().clone());
    let count_task = tokio::spawn(async move {
        let (count,): (i64,) = sqlx::query_as(&Painting::count_all_query())
            .fetch_one(&*count_client)
            .await
            .expect("Failed to count paintings");
        count
    });

    let client: Arc<Pool<Postgres>> = Arc::new(get_client().await.unwrap().clone());
    let rows_task = tokio::spawn(async move {
        let select_query = Painting::get_all_stubs_query(query, Some(language));
        let rows = sqlx::query_as::<_, PaintingStub>(&select_query)
            .fetch_all(&*client)
            .await
            .expect("Failed to select paintings rows");
        rows
    });

    let (count, rows) = tokio::join!(count_task, rows_task);
    debug!(target: "template", "paintings:get_all - rows {:?}", &rows);

    let page_data: GalleryPageData = GalleryPageData {
        created: get_translation(TranslationKeys::Created, language),
        price: get_translation(TranslationKeys::Price, language),
        width: get_translation(TranslationKeys::Width, language),
        height: get_translation(TranslationKeys::Height, language),
        title: get_translation(TranslationKeys::Title, language),
        sold: get_translation(TranslationKeys::Sold, language),
        ascending: get_translation(TranslationKeys::Ascending, language),
        descending: get_translation(TranslationKeys::Descending, language),
        gallery_item_stubs: rows.unwrap_or_else(|error| {
            error!(target: "template", "paitings:get_all - failed to get rows {}", error);
            Vec::new()
        }),
        filter_form_action: "",
    };

    let pagination_data: PaginationData = PaginationData {
        total: count.unwrap_or_else(|error| {
            error!(target: "template", "paintings:get_all - failed to count paintings {}", error);
            0_i64
        }),
        current_start: parsed_query.offset,
        current_end: (parsed_query.offset + parsed_query.limit as u32) as u64,
        prev_page: &format!(
            "/{}/gallery?limit={}&offset={}&sort={}&order={}",
            &language_string,
            parsed_query.limit,
            parsed_query.offset - parsed_query.limit as u32,
            parsed_query.sort,
            parsed_query.order
        ),
        next_page: &format!(
            "/{}/gallery?limit={}&offset={}&sort={}&order={}",
            &language_string,
            parsed_query.limit,
            parsed_query.offset - parsed_query.limit as u32,
            parsed_query.sort,
            parsed_query.order
        ),
    };

    let mut meta_props = MetaProps::default(Some(language));
    meta_props.url = format!("www.rosemary-artist.com/{}", language.to_str());

    let template = GalleryPage {
        meta: meta_props,
        page: page_data,
        navbar: NavbarProps::default(Some(language)),
        footer: FooterProps::default(Some(language)),
        pagination: pagination_data,
    };
    debug!(target: "template", "paintings:get_all - template data {:?}", &template);

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
        .and_then(|query| async { get_template(Language::Cs, query).await })
}

pub fn get_cz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::Cs.to_str()))
        .and(path("gallery"))
        .and(path::end())
        .and(query::<GetPaintingsQuery>())
        .and_then(|query| async { get_template(Language::Cs, query).await })
}

pub fn get_en() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::En.to_str()))
        .and(path("gallery"))
        .and(path::end())
        .and(query::<GetPaintingsQuery>())
        .and_then(|query| async { get_template(Language::En, query).await })
}
