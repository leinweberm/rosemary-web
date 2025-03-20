pub struct PageTranslation {
    pub en: &'static str,
    pub cs: &'static str,
}

pub enum Language {
    En,
    Cs,
}

pub enum TranslationKeys {
    Ascending,
    Created,
    Contact,
    Descending,
    Height,
    Navigation,
    Price,
    Sold,
    Title,
    Width,
    None,
}

pub struct PageTranslations {
    ascending: PageTranslation,
    created: PageTranslation,
    contact: PageTranslation,
    descending: PageTranslation,
    height: PageTranslation,
    navigation: PageTranslation,
    price: PageTranslation,
    sold: PageTranslation,
    title: PageTranslation,
    width: PageTranslation,
    none: PageTranslation,
}

impl PageTranslations {
    pub fn get_translation(&self, key: TranslationKeys, lang: Language) -> &'static str {
        let translation = match key {
            TranslationKeys::Ascending => &self.ascending,
            TranslationKeys::Created => &self.created,
            TranslationKeys::Contact => &self.contact,
            TranslationKeys::Descending => &self.descending,
            TranslationKeys::Height => &self.height,
            TranslationKeys::Navigation => &self.navigation,
            TranslationKeys::Price => &self.price,
            TranslationKeys::Sold => &self.sold,
            TranslationKeys::Title => &self.title,
            TranslationKeys::Width => &self.width,
            TranslationKeys::None => &self.none,
        };

        match lang {
            Language::En => translation.en,
            Language::Cs => translation.cs,
        }
    }
}

static PAGE_TRANSLATIONS: PageTranslations = PageTranslations {
    ascending: PageTranslation {
        en: "ascending",
        cs: "vzestupně",
    },
    created: PageTranslation {
        en: "created",
        cs: "vytvořeno",
    },
    contact: PageTranslation {
        en: "contact",
        cs: "kontakt",
    },
    descending: PageTranslation {
        en: "descending",
        cs: "sestupně",
    },
    height: PageTranslation {
        en: "height",
        cs: "výška",
    },
    navigation: PageTranslation {
        en: "navigation",
        cs: "navigace",
    },
    price: PageTranslation {
        en: "price",
        cs: "cena",
    },
    sold: PageTranslation {
        en: "sold",
        cs: "prodáno",
    },
    title: PageTranslation {
        en: "title",
        cs: "název",
    },
    width: PageTranslation {
        en: "width",
        cs: "šířka",
    },
    none: PageTranslation {
        en: "- none -",
        cs: "- none -",
    },
};

pub fn get_translation(key: TranslationKeys, lang: Language) -> &'static str {
    PAGE_TRANSLATIONS.get_translation(key, lang)
}