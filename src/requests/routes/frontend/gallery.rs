use askama::Template;
use warp::{path, query, Filter, Rejection, Reply};

use crate::client::component_props::{FooterProps, MetaProps, NavbarProps};
use crate::client::translations::{get_translation, Language, TranslationKeys};
use crate::config::load;
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
    filter_action: &'a str,
    current_sort: &'a str,
    current_search: &'a str,
    current_order: &'a str,
    show_prev_page: bool,
    show_next_page: bool,
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

async fn get_template(
    language: Language,
    query: GetPaintingsQuery,
) -> Result<impl Reply, Rejection> {
    let language_string = language.to_string();
    let parsed_query = query.safe_parse(Some(language));
    let search = if let Some(search) = parsed_query.search.clone() {
        search
    } else {
        String::new()
    };

    let client = get_client().await.unwrap();
    let count_task = tokio::spawn(async move {
        let (count,): (i64,) = sqlx::query_as(&Painting::count_all_query())
            .fetch_one(client)
            .await
            .expect("Failed to count paintings");
        count
    });

    let parsed_query_clone = parsed_query.clone();
    let rows_task = tokio::spawn(async move {
        let static_base_url = match load::get::<String>(load::ConfigField::StaticFileUrl).await {
            Ok(path) => path,
            Err(error) => {
                error!(target: "template", "paintings:get_all - failed to get static base url {}", error);
                String::new()
            }
        };
        let select_query =
            Painting::get_all_stubs_query(parsed_query_clone, Some(language), &static_base_url);
        let rows = sqlx::query_as::<_, PaintingStub>(&select_query)
            .fetch_all(client)
            .await
            .expect("Failed to select paintings rows");
        rows
    });

    let (count, rows) = tokio::join!(count_task, rows_task);
    debug!(target: "template", "paintings:get_all - rows {:?}", &rows);
    let parsed_count = count.unwrap_or_else(|error| {
        error!(target: "template", "paintings:get_all - failed to count paintings {}", error);
        0_i64
    });

    let mut show_prev_page = false;
    let mut show_next_page = false;
    let mut current_end = (parsed_query.offset + parsed_query.limit as u32) as u64;
    match (query.offset, query.limit) {
        (Some(offset), Some(limit)) => {
            if offset > limit as u32 {
                show_prev_page = true;
            }
            if (offset as u64 + limit as u64) < parsed_count as u64 {
                show_next_page = true;
            } else {
                current_end = parsed_count as u64;
            }
        }
        _ => {}
    }

    let page_data: GalleryPageData = GalleryPageData {
        created: get_translation(TranslationKeys::Created, language),
        price: get_translation(TranslationKeys::Price, language),
        width: get_translation(TranslationKeys::Width, language),
        height: get_translation(TranslationKeys::Height, language),
        title: get_translation(TranslationKeys::GalleryTitle, language),
        sold: get_translation(TranslationKeys::Sold, language),
        ascending: get_translation(TranslationKeys::Ascending, language),
        descending: get_translation(TranslationKeys::Descending, language),
        gallery_item_stubs: rows.unwrap_or_else(|error| {
            error!(target: "template", "paitings:get_all - failed to get rows {}", error);
            Vec::new()
        }),
        filter_form_action: &format!("/{}/gallery?limit=20&offset=0", &language_string),
        filter_action: get_translation(TranslationKeys::Filter, language),
        current_sort: &parsed_query.sort,
        current_order: &parsed_query.order,
        current_search: &search,
        show_prev_page,
        show_next_page,
    };

    let prev_page_offset = if parsed_query.limit as u32 > parsed_query.offset {
        0
    } else {
        parsed_query.offset - parsed_query.limit as u32
    };

    let pagination_data: PaginationData = PaginationData {
        total: parsed_count,
        current_start: parsed_query.offset + 1,
        prev_page: &format!(
            "/{}/gallery?limit={}&offset={}&sort={}&order={}",
            &language_string,
            parsed_query.limit,
            prev_page_offset,
            parsed_query.sort,
            parsed_query.order
        ),
        next_page: &format!(
            "/{}/gallery?limit={}&offset={}&sort={}&order={}",
            &language_string,
            parsed_query.limit,
            parsed_query.offset + parsed_query.limit as u32,
            parsed_query.sort,
            parsed_query.order
        ),
        current_end,
    };

    let mut meta_props = MetaProps::default(Some(language));
    meta_props.url = format!("www.rosemary-artist.com/{}/gallery", language.to_str());

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
