use crate::client::translations::{get_translation, Language, TranslationKeys};
use crate::config::load;

#[derive(Debug)]
pub struct MetaProps<'a> {
    pub description: &'a str,
    pub keywords: &'a str,
    pub author: &'a str,
    pub robots: &'a str,
    pub image: &'a str,
    pub locale: String,
    pub favicon: &'a str,
    pub summary_large_image: &'a str,
    pub twitter_handle: &'a str,
    pub url: String,
    pub static_base_url: String,
}

impl<'a> MetaProps<'a> {
    pub fn default(language: Option<Language>) -> Self {
        let lang = if let Some(language_enum) = language {
            language_enum
        } else {
            Language::Cs
        };

        let static_base_url = match load::get_sync::<String>(load::ConfigField::StaticFileUrl) {
            Ok(path) => path,
            Err(error) => {
                error!(target: "template", "paintings:get_all - failed to get static base url {}", error);
                String::new()
            }
        };

        Self {
            description: "",
            keywords: "Rosemary, Michaela, Halásová, malíř, painter, fotograf, photograph, abstract oil paintings, weddings",
            author: "Rosemary - Michaela Halásová",
            robots: "index, follow",
            image: "",
            locale: lang.to_string(),
            favicon: "",
            summary_large_image: "",
            twitter_handle: "",
            url: format!("www.rosemary-artist.com/{}", lang.to_str()),
            static_base_url: static_base_url.clone(),
        }
    }
}

#[derive(Debug)]
pub struct NavbarProps<'a> {
    pub home: &'a str,
    pub gallery: &'a str,
    pub photo: &'a str,
    pub blog: &'a str,
    pub contact: &'a str,
}

impl<'a> NavbarProps<'a> {
    pub fn default(language: Option<Language>) -> Self {
        let lang = if let Some(language_enum) = language {
            language_enum
        } else {
            Language::Cs
        };

        Self {
            home: get_translation(TranslationKeys::Home, lang),
            gallery: get_translation(TranslationKeys::Gallery, lang),
            photo: get_translation(TranslationKeys::Photo, lang),
            blog: get_translation(TranslationKeys::Blog, lang),
            contact: get_translation(TranslationKeys::Contact, lang),
        }
    }
}

#[derive(Debug)]
pub struct FooterProps<'a> {
    pub upcoming_events: &'a str,
    pub photo_pricing: &'a str,
    pub photo_reservation: &'a str,
    pub home: &'a str,
    pub gallery: &'a str,
    pub blog: &'a str,
    pub contact: &'a str,
    pub faq: &'a str,
    pub navigation: &'a str,
}

impl<'a> FooterProps<'a> {
    pub fn default(language: Option<Language>) -> Self {
        let lang = if let Some(language_enum) = language {
            language_enum
        } else {
            Language::Cs
        };

        Self {
            upcoming_events: get_translation(TranslationKeys::UpcomingEvents, lang),
            photo_pricing: get_translation(TranslationKeys::PhotoPricing, lang),
            photo_reservation: get_translation(TranslationKeys::PhotoReservation, lang),
            home: get_translation(TranslationKeys::Home, lang),
            gallery: get_translation(TranslationKeys::Gallery, lang),
            blog: get_translation(TranslationKeys::Blog, lang),
            contact: get_translation(TranslationKeys::Contact, lang),
            faq: get_translation(TranslationKeys::FAQ, lang),
            navigation: get_translation(TranslationKeys::Navigation, lang),
        }
    }
}
