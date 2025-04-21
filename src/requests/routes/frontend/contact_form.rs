use askama::Template;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use warp::filters::addr::remote;
use warp::{path, query, Filter, Rejection, Reply};

use crate::client::component_props::{FooterProps, MetaProps, NavbarProps};
use crate::client::translations::{get_translation, Language, TranslationKeys};
use crate::config::load::{self, ConfigField};
use crate::database::connection::get_client;
use crate::database::models::emails::{Email, EmailCreate};
use crate::database::models::user_entries::{UserEntry, UserEntryCount, UserEntryType};

#[derive(Debug, Clone, Copy)]
struct ContactSentData<'a> {
    pub result: &'a str,
    pub description: &'a str,
    pub back: &'a str,
    pub title: &'a str,
}

#[derive(Debug, Template, Clone)]
#[template(path = "./contactSent.html")]
struct ContactSentPage<'a> {
    meta: MetaProps<'a>,
    page: ContactSentData<'a>,
    navbar: NavbarProps<'a>,
    footer: FooterProps<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContactFormQuery {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub subject: String,
    pub body: String,
}

fn internal_error_texts(language: Language) -> Result<Box<dyn Reply>, Rejection> {
    let (result, description) = match language {
        Language::Cs => (
            "Něco se pokazilo!",
            "Zkuste akci opakovat později. Děkuji za pochopení.",
        ),
        Language::En => (
            "Something went terribly wrong!",
            "Please repeat the action later. Thank you",
        ),
    };

    let mut meta_props = MetaProps::default(Some(language));
    meta_props.description = get_translation(TranslationKeys::IndexMetaDescription, language);

    let template = ContactSentPage {
        meta: meta_props,
        page: ContactSentData {
            result,
            description,
            back: get_translation(TranslationKeys::Back, language),
            title: "Rosemary - Artist",
        },
        navbar: NavbarProps::default(Some(language)),
        footer: FooterProps::default(Some(language)),
    };

    let rendered = template.render().unwrap();
    Ok(Box::new(warp::reply::html(rendered)))
}

fn unauthorized_error_texts<'a>(language: Language) -> Result<Box<dyn Reply>, Rejection> {
    let (result, description) = match language {
        Language::Cs => (
            "Neoprávněný přístup!",
            "Vyčerpali jste limit pro odeslání formuláře, zkuste to prosím později.",
        ),
        Language::En => (
            "Unauthorized access!",
            "You ran out of limit for submitting forms, please try it later.",
        ),
    };

    let mut meta_props = MetaProps::default(Some(language));
    meta_props.description = get_translation(TranslationKeys::IndexMetaDescription, language);

    let template = ContactSentPage {
        meta: meta_props,
        page: ContactSentData {
            result,
            description,
            back: get_translation(TranslationKeys::Back, language),
            title: "Rosemary - Artist",
        },
        navbar: NavbarProps::default(Some(language)),
        footer: FooterProps::default(Some(language)),
    };

    let rendered = template.render().unwrap();
    Ok(Box::new(warp::reply::html(rendered)))
}

async fn get_template(
    language: Language,
    query: ContactFormQuery,
    ip: Option<SocketAddr>,
) -> Result<impl Reply, Rejection> {
    let client = get_client().await.unwrap();
    let http_client = reqwest::Client::new();
    let user_action_limit = load::get::<i8>(ConfigField::UserActionDaily).await.unwrap();
    let mailer_token = load::get::<String>(ConfigField::MailerSendToken)
        .await
        .unwrap();
    let user_ip = ip
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".into());

    debug!(target: "template", "contact:form - user  IP {}", &user_ip);
    debug!(target: "template", "contact:form - query {:?}", &query);

    let count_query = Email::count_user_entries_query(&query.email, &user_ip);
    let actions_count = sqlx::query_as::<_, UserEntryCount>(&count_query)
        .fetch_one(client)
        .await;

    let mut meta_props = MetaProps::default(Some(language));
    meta_props.url = format!("www.rosemary-artist.com/{}/contact/sent", language.to_str());

    let (page_result, page_description) = match language {
        Language::Cs => (
            "Zprává úspěšně odeslána",
            "Jsem ráda, že jste se rozhodli se mnou spojit. Budu Vás kontaktovat jakmile to bude možné.",
        ),
        Language::En => (
            "Message sent",
            "Thank you for getting in touch with me, I will get back to you as soon as possible."
        ),
    };

    let page_data = ContactSentData {
        result: page_result,
        description: page_description,
        title: "Rosemary - Artist",
        back: get_translation(TranslationKeys::Back, language),
    };

    let template = ContactSentPage {
        meta: meta_props,
        page: page_data,
        navbar: NavbarProps::default(Some(language)),
        footer: FooterProps::default(Some(language)),
    };

    match actions_count {
        Ok(value) => {
            if value.count >= user_action_limit as i16 {
                return unauthorized_error_texts(language);
            }
        }
        Err(err) => {
            error!(
                target: "template",
                "Email::count_user_entries_query failed {:?}",
                err
            );
            return internal_error_texts(language);
        }
    }

    let email_payload = json!({
        "from": {
            "email": "info.rosemary.artist@gmail.com",
            "name": "rosemary-artist.com"
        },
        "to": [
            {
                "email": "rosemaryphotography@seznam.cz",
                "name": "Michaela Halasová"
            }
        ],
        "reply_to": {
            "email": &query.email,
            "name": &query.name,
        },
        "personalization": [{
            "email": &query.email,
            "data": {
                "data": {
                    "body": &query.body,
                    "name": &query.name,
                    "email": &query.email,
                    "phone": &query.phone,
                    "subject": &query.subject
                }
            }
        }],
        "template_id": "7dnvo4d381xg5r86"
    });

    let mailer_response = http_client
        .post("https://api.mailersend.com/v1/email")
        .header("Content-Type", "application/json")
        .header("X-Requested-With", "XMLHttpRequest")
        .bearer_auth(&mailer_token)
        .json(&email_payload)
        .send()
        .await;

    match mailer_response {
        Ok(_) => (),
        Err(err) => {
            error!(
                target: "template",
                "sending email via mailer failed {:?}",
                err
            );
            return internal_error_texts(language);
        }
    }

    let create_email_query = Email::create_email_query(EmailCreate {
        subject: &query.subject,
        email: &query.email,
        phone: &query.phone,
        body: &query.body,
        ip_address: &user_ip,
    });
    let created_email = sqlx::query_as::<_, Email>(&create_email_query)
        .fetch_one(client)
        .await;
    match created_email {
        Ok(_value) => (),
        Err(err) => {
            error!(
                target: "template",
                "Email::create_email_query failed {:?}",
                err
            );
            return internal_error_texts(language);
        }
    }

    let create_user_entry_query = UserEntry::create_user_entry_query(
        UserEntryType::ContactForm,
        Some(&query.email),
        &user_ip,
    );
    let created_user_entry = sqlx::query_as::<_, UserEntry>(&create_user_entry_query)
        .fetch_one(client)
        .await;
    match created_user_entry {
        Ok(_) => (),
        Err(err) => {
            error!(
                target: "template",
                "UserEntry::create_user_entry_query failed {:?}",
                err
            );
        }
    }

    let result = template.render().unwrap();
    return Ok(Box::new(warp::reply::html(result)))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("contact"))
        .and(path("sent"))
        .and(query::<ContactFormQuery>())
        .and(remote())
        .and(path::end())
        .and_then(|query, ip| async move { get_template(Language::Cs, query, ip).await })
}

pub fn get_cz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::Cs.to_str()))
        .and(path("contact-form"))
        .and(path("sent"))
        .and(query::<ContactFormQuery>())
        .and(remote())
        .and(path::end())
        .and_then(|query, ip| async move { get_template(Language::Cs, query, ip).await })
}

pub fn get_en() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path(Language::En.to_str()))
        .and(path("contact"))
        .and(path("sent"))
        .and(query::<ContactFormQuery>())
        .and(remote())
        .and(path::end())
        .and_then(|query, ip| async move { get_template(Language::En, query, ip).await })
}
