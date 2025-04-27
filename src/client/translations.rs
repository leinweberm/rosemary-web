use crate::client::localization::PAGE_TRANSLATIONS;

pub struct PageTranslation {
    pub en: &'static str,
    pub cs: &'static str,
}

#[derive(Copy, Debug, Clone)]
pub enum Language {
    En,
    Cs,
}

impl Language {
    pub fn to_str(&self) -> &'static str {
        match self {
            Language::Cs => "cs",
            Language::En => "en",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Language::Cs => String::from("cs"),
            Language::En => String::from("en"),
        }
    }
}

#[derive(Copy, Debug, Clone)]
pub enum TranslationKeys {
    Ascending,
    Back,
    Blog,
    Buy,
    Created,
    Contact,
    Close,
    Descending,
    Email,
    FAQ,
    Filter,
    FullName,
    Gallery,
    GalleryTitle,
    General,
    Height,
    Home,
    IndexTitle,
    IndexHeroAlt,
    IndexPicturAlt,
    IndexDescription,
    IndexMetaDescription,
    IndexMetaKeywords,
    IndexMetaImageSummary,
    Message,
    Name,
    Navigation,
    Next,
    Painting,
    Phone,
    Photo,
    Photography,
    PhotoPricing,
    PhotoReservation,
    Prev,
    Price,
    Send,
    Sold,
    Subject,
    Title,
    UpcomingEvents,
    Width,
}

pub struct PageTranslations {
    pub ascending: PageTranslation,
    pub blog: PageTranslation,
    pub buy: PageTranslation,
    pub created: PageTranslation,
    pub contact: PageTranslation,
    pub close: PageTranslation,
    pub descending: PageTranslation,
    pub email: PageTranslation,
    pub faq: PageTranslation,
    pub filter: PageTranslation,
    pub full_name: PageTranslation,
    pub gallery: PageTranslation,
    pub gallery_title: PageTranslation,
    pub general: PageTranslation,
    pub height: PageTranslation,
    pub home: PageTranslation,
    pub index_title: PageTranslation,
    pub index_hero_alt: PageTranslation,
    pub index_picture_alt: PageTranslation,
    pub index_description: PageTranslation,
    pub index_meta_description: PageTranslation,
    pub index_meta_keywords: PageTranslation,
    pub index_meta_image_summary: PageTranslation,
    pub message: PageTranslation,
    pub name: PageTranslation,
    pub navigation: PageTranslation,
    pub next: PageTranslation,
    pub painting: PageTranslation,
    pub phone: PageTranslation,
    pub photo: PageTranslation,
    pub photography: PageTranslation,
    pub photo_pricing: PageTranslation,
    pub photo_reservation: PageTranslation,
    pub prev: PageTranslation,
    pub price: PageTranslation,
    pub send: PageTranslation,
    pub sold: PageTranslation,
    pub subject: PageTranslation,
    pub title: PageTranslation,
    pub upcoming_events: PageTranslation,
    pub width: PageTranslation,
    pub back: PageTranslation,
}

impl PageTranslations {
    pub fn get_translation(&self, key: TranslationKeys, lang: Language) -> &'static str {
        let translation = match key {
            TranslationKeys::Ascending => &self.ascending,
            TranslationKeys::Back => &self.back,
            TranslationKeys::Blog => &self.blog,
            TranslationKeys::Buy => &self.buy,
            TranslationKeys::Created => &self.created,
            TranslationKeys::Contact => &self.contact,
            TranslationKeys::Close => &self.close,
            TranslationKeys::Descending => &self.descending,
            TranslationKeys::Email => &self.email,
            TranslationKeys::FAQ => &self.faq,
            TranslationKeys::Filter => &self.filter,
            TranslationKeys::FullName => &self.full_name,
            TranslationKeys::Gallery => &self.gallery,
            TranslationKeys::GalleryTitle => &self.gallery_title,
            TranslationKeys::General => &self.general,
            TranslationKeys::Height => &self.height,
            TranslationKeys::Home => &self.home,
            TranslationKeys::IndexTitle => &self.index_title,
            TranslationKeys::IndexHeroAlt => &self.index_hero_alt,
            TranslationKeys::IndexPicturAlt => &self.index_picture_alt,
            TranslationKeys::IndexDescription => &self.index_description,
            TranslationKeys::IndexMetaDescription => &self.index_meta_description,
            TranslationKeys::IndexMetaKeywords => &self.index_meta_keywords,
            TranslationKeys::IndexMetaImageSummary => &self.index_meta_image_summary,
            TranslationKeys::Navigation => &self.navigation,
            TranslationKeys::Next => &self.next,
            TranslationKeys::Message => &self.message,
            TranslationKeys::Name => &self.name,
            TranslationKeys::Painting => &self.painting,
            TranslationKeys::Phone => &self.phone,
            TranslationKeys::Photo => &self.photo,
            TranslationKeys::Photography => &self.photography,
            TranslationKeys::PhotoPricing => &self.photo_pricing,
            TranslationKeys::PhotoReservation => &self.photo_reservation,
            TranslationKeys::Prev => &self.prev,
            TranslationKeys::Price => &self.price,
            TranslationKeys::Send => &self.send,
            TranslationKeys::Sold => &self.sold,
            TranslationKeys::Subject => &self.subject,
            TranslationKeys::Title => &self.title,
            TranslationKeys::UpcomingEvents => &self.upcoming_events,
            TranslationKeys::Width => &self.width,
        };

        match lang {
            Language::En => translation.en,
            Language::Cs => translation.cs,
        }
    }
}

pub fn get_translation(key: TranslationKeys, lang: Language) -> &'static str {
    PAGE_TRANSLATIONS.get_translation(key, lang)
}
