use askama::Template;
use uuid::Uuid;
use warp::{path, Filter, Rejection, Reply};

use crate::client::component_props::{FooterProps, MetaProps, NavbarProps};
use crate::client::translations::{get_translation, Language, TranslationKeys};
use crate::database::connection::get_client;
use crate::database::models::image::PaintingImage;
use crate::database::models::painting::Painting;
use crate::errors::api_error::InternalServerError;

#[derive(Debug)]
struct PaintingPhoto {
    url: String,
    alt: String,
}

#[derive(Debug)]
struct GalleryDetailPageData<'a> {
    title: &'a str,
    description: &'a str,
    dimenzions: &'a str,
    unit: &'a str,
    technique: &'a str,
    main_photo_sizes: &'a str,
    main_photo_srcset: &'a str,
    main_photo_src: &'a str,
    main_photo_alt: &'a str,
    photos: Vec<PaintingPhoto>,
    photos_count: &'a str,
}

#[derive(Debug)]
struct GalleryDetailLabels<'a> {
    buy_with_price: &'a str,
    full_name: &'a str,
    email: &'a str,
    close: &'a str,
    prev: &'a str,
    next: &'a str,
}

#[derive(Debug, Template)]
#[template(path = "./galleryDetail.html")]
struct GalleryDetailPage<'a> {
    page: GalleryDetailPageData<'a>,
    labels: GalleryDetailLabels<'a>,
    navbar: NavbarProps<'a>,
    footer: FooterProps<'a>,
    meta: MetaProps<'a>,
}

async fn get_template(language: Language, id: Uuid) -> Result<impl Reply, Rejection> {
    let client = get_client().await.unwrap();

    let painting_task = tokio::spawn(async move {
        let query = Painting::get_one_query(id.clone());
        debug!(target: "template", "painting:get - Painting::get_one_query {}", &query);

        let painting = sqlx::query_as::<_, Painting>(&query)
            .fetch_one(client)
            .await
            .unwrap_or_else(|err| {
                error!(target: "api", "paitings:get - Painting::get_one_query failed {:?}", err);
                panic!("running Painting::get_one_query() for the client failed");
            });
        painting
    });

    let images_task = tokio::spawn(async move {
        let query = PaintingImage::get_all_for_query(id.clone());
        debug!(target: "template", "paintings:get - PaintingImage::get_all_for_query {}", &query);

        let images = sqlx::query_as::<_, PaintingImage>(&query)
            .fetch_all(client)
            .await
            .unwrap_or_else(|err| {
                error!(
                    target: "api",
                    "paitings:get - PaintingImage::get_all_for_query failed {:?}",
                    err
                );
                panic!("PaintingImage::get_all_for_query failed");
            });
        images
    });

    let (painting_result, images_result) = tokio::join!(painting_task, images_task);

    if painting_result.is_err() {
        return Err(warp::reject::custom(InternalServerError::new()));
    }

    if images_result.is_err() {
        return Err(warp::reject::custom(InternalServerError::new()));
    }

    let metadata = painting_result.unwrap();
    let images = images_result.unwrap();

    let mut painting_photos: Vec<PaintingPhoto> = Vec::new();
    for image in images.into_iter() {
        painting_photos.push(PaintingPhoto {
            url: image.urls[0].clone(),
            alt: image.get_alt(language).to_string(),
        });
    }

    let mut meta_props = MetaProps::default(Some(language));
    meta_props.url = format!("www.rosemary-artist.com/{}/{}", language.to_str(), id);

    let main_srcset = format!(
        "{} 320w, {} 640w, {} 1024w, {} 1900w",
        format!(
            "{}/{}",
            &meta_props.static_base_url, &metadata.preview.urls[0]
        ),
        format!(
            "{}/{}",
            &meta_props.static_base_url, &metadata.preview.urls[1]
        ),
        format!(
            "{}/{}",
            &meta_props.static_base_url, &metadata.preview.urls[2]
        ),
        format!(
            "{}/{}",
            &meta_props.static_base_url, &metadata.preview.urls[3]
        ),
    );

    let photo_count_number = painting_photos.len();
    let (price, currency, photo_count) = match language {
        Language::Cs => {
            let photo_count_label = if photo_count_number == 1 {
                "fotka"
            } else if photo_count_number > 1 && photo_count_number < 5 {
                "fotky"
            } else {
                "fotek"
            };
            (
                metadata.price,
                String::from("CZK"),
                format!("{} {}", photo_count_number, photo_count_label),
            )
        }
        Language::En => {
            let photo_count_label = if photo_count_number == 1 {
                "photo"
            } else {
                "photos"
            };
            (
                (metadata.price as f64 / 23.0).round() as i64,
                String::from("€"),
                format!("{} {}", photo_count_number, photo_count_label),
            )
        }
    };

    let dimenzions = format!("{}x{}", metadata.width, metadata.height);

    let page_data = GalleryDetailPageData {
        title: metadata.get_title(language),
        description: metadata.get_description(language),
        dimenzions: &dimenzions,
        unit: "cm",
        technique: "oil on canvas",
        main_photo_sizes:
            "(max-width: 320px) 320px, (max-width: 640px) 640px, (max-width: 1024px) 1024px, 1900px",
        main_photo_srcset: &main_srcset,
        main_photo_src: &metadata.preview.urls[3],
        main_photo_alt: metadata.preview.get_alt(language),
        photos: painting_photos,
        photos_count: &photo_count,
    };

    let buy_with_price_string = format!(
        "{} {}{}",
        get_translation(TranslationKeys::Buy, language),
        price,
        currency
    );

    let template = GalleryDetailPage {
        meta: meta_props,
        navbar: NavbarProps::default(Some(language)),
        footer: FooterProps::default(Some(language)),
        page: page_data,
        labels: GalleryDetailLabels {
            buy_with_price: &buy_with_price_string,
            full_name: get_translation(TranslationKeys::FullName, language),
            email: get_translation(TranslationKeys::Email, language),
            close: get_translation(TranslationKeys::Close, language),
            prev: get_translation(TranslationKeys::Prev, language),
            next: get_translation(TranslationKeys::Next, language),
        },
    };

    let result = template
        .render()
        .unwrap_or_else(|_| String::from("<h1>Internal Server Error</h1>"));

    Ok(warp::reply::html(result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("gallery"))
        .and(path::param::<Uuid>())
        .and(path::end())
        .and_then(|param| async move { get_template(Language::Cs, param).await })
}

pub fn get_cz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::Cs.to_str()))
        .and(path("gallery"))
        .and(path::param::<Uuid>())
        .and(path::end())
        .and_then(|param| async move { get_template(Language::Cs, param).await })
}

pub fn get_en() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::En.to_str()))
        .and(path("gallery"))
        .and(path::param::<Uuid>())
        .and(path::end())
        .and_then(|param| async move { get_template(Language::En, param).await })
}
