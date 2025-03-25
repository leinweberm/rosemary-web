use askama::Template;
use warp::{path, Filter, Rejection, Reply};

use crate::{
    client::{
        component_props::{FooterProps, MetaProps, NavbarProps},
        translations::{get_translation, Language, TranslationKeys},
    },
    errors::api_error::InternalServerError,
};

pub struct ContactPageData<'a> {
    title: &'a str,
    name: &'a str,
    email: &'a str,
    phone: &'a str,
    subject: &'a str,
    painting: &'a str,
    photography: &'a str,
    general: &'a str,
    message: &'a str,
    send: &'a str,
}

#[derive(Template)]
#[template(path = "./contact.html")]
pub struct ContactPage<'a> {
    meta: MetaProps<'a>,
    page: ContactPageData<'a>,
    navbar: NavbarProps<'a>,
    footer: FooterProps<'a>,
}

async fn get_template(lang: Language) -> Result<impl Reply, Rejection> {
    let lang_string = lang.to_string();

    let page_data: ContactPageData = ContactPageData {
        title: get_translation(TranslationKeys::IndexTitle, lang),
        name: get_translation(TranslationKeys::Name, lang),
        email: get_translation(TranslationKeys::Email, lang),
        phone: get_translation(TranslationKeys::Phone, lang),
        subject: get_translation(TranslationKeys::Subject, lang),
        painting: get_translation(TranslationKeys::Painting, lang),
        photography: get_translation(TranslationKeys::Photography, lang),
        general: get_translation(TranslationKeys::General, lang),
        message: get_translation(TranslationKeys::Message, lang),
        send: get_translation(TranslationKeys::Send, lang),
    };

    let mut meta_props = MetaProps::default(Some(lang));
    meta_props.url = format!("www.rosemary-artist.com/{}/contact", &lang_string);
    meta_props.description = "Rosemary, artist, contact, kontakt, form, formular, email";
    meta_props.summary_large_image = "";

    let template = ContactPage {
        meta: meta_props,
        page: page_data,
        navbar: NavbarProps::default(Some(lang)),
        footer: FooterProps::default(Some(lang)),
    };

    let result = template.render();

    match result {
        Ok(value) => Ok(warp::reply::html(value)),
        Err(error) => {
            error!(target: "template", "contact:form - failed to prepare template {}", error);
            Ok(warp::reply::html(
                InternalServerError::new().html_response(),
            ))
        }
    }
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("contact"))
        .and(path::end())
        .and_then(|| async { get_template(Language::Cs).await })
}

pub fn get_cz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::Cs.to_str()))
        .and(path("contact"))
        .and(path::end())
        .and_then(|| async { get_template(Language::Cs).await })
}

pub fn get_en() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::En.to_str()))
        .and(path("contact"))
        .and(path::end())
        .and_then(|| async { get_template(Language::En).await })
}
